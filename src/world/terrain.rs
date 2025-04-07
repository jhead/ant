use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::sprite::MaterialMesh2dBundle;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_LOAD_DISTANCE: i32 = 5;
pub const WORLD_SEED: u32 = 42;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TerrainLayer {
    Surface,
    Dirt,
    Stone,
    Deep,
    Bedrock,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TerrainFeature {
    None,
    Cave,
    WaterPocket,
    ResourceVein,
    Tunnel,
}

#[derive(Component, Clone)]
pub struct TerrainChunk {
    pub position: IVec2,
    pub layer: TerrainLayer,
    pub features: Vec<(IVec2, TerrainFeature)>,
    pub generated: bool,
}

#[derive(Resource)]
pub struct TerrainCache {
    pub chunks: HashMap<IVec2, TerrainChunk>,
    pub rng: StdRng,
    pub noise: Perlin,
}

impl Default for TerrainCache {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            rng: StdRng::seed_from_u64(WORLD_SEED as u64),
            noise: Perlin::new(WORLD_SEED as u32),
        }
    }
}

pub fn generate_chunk_data(terrain_cache: &mut TerrainCache, position: IVec2) -> TerrainChunk {
    let layer = if position.y > 0 {
        TerrainLayer::Surface
    } else if position.y > -5 {
        TerrainLayer::Dirt
    } else if position.y > -10 {
        TerrainLayer::Stone
    } else if position.y > -20 {
        TerrainLayer::Deep
    } else {
        TerrainLayer::Bedrock
    };

    let mut features = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let world_x = position.x as f64 * CHUNK_SIZE as f64 + x as f64;
            let world_y = position.y as f64 * CHUNK_SIZE as f64 + y as f64;

            let cave_noise = terrain_cache.noise.get([world_x * 0.05, world_y * 0.05]);
            let water_noise = terrain_cache
                .noise
                .get([world_x * 0.03 + 100.0, world_y * 0.03 + 100.0]);
            let resource_noise = terrain_cache
                .noise
                .get([world_x * 0.02 + 200.0, world_y * 0.02 + 200.0]);

            let feature = match layer {
                TerrainLayer::Surface => {
                    if cave_noise > 0.7 {
                        TerrainFeature::Cave
                    } else {
                        TerrainFeature::None
                    }
                }
                TerrainLayer::Dirt => {
                    if cave_noise > 0.6 {
                        TerrainFeature::Cave
                    } else if water_noise > 0.8 {
                        TerrainFeature::WaterPocket
                    } else {
                        TerrainFeature::None
                    }
                }
                TerrainLayer::Stone => {
                    if cave_noise > 0.5 {
                        TerrainFeature::Cave
                    } else if resource_noise > 0.7 {
                        TerrainFeature::ResourceVein
                    } else {
                        TerrainFeature::None
                    }
                }
                TerrainLayer::Deep => {
                    if cave_noise > 0.4 {
                        TerrainFeature::Cave
                    } else if water_noise > 0.7 {
                        TerrainFeature::WaterPocket
                    } else if resource_noise > 0.6 {
                        TerrainFeature::ResourceVein
                    } else {
                        TerrainFeature::None
                    }
                }
                TerrainLayer::Bedrock => {
                    if resource_noise > 0.5 {
                        TerrainFeature::ResourceVein
                    } else {
                        TerrainFeature::None
                    }
                }
            };

            if feature != TerrainFeature::None {
                features.push((IVec2::new(x, y), feature));
            }
        }
    }

    TerrainChunk {
        position,
        layer,
        features,
        generated: true,
    }
}

pub fn spawn_chunk_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: IVec2,
    chunk_data: &TerrainChunk,
) {
    let world_x = position.x as f32 * CHUNK_SIZE as f32;
    let world_y = position.y as f32 * CHUNK_SIZE as f32;
    let chunk_size = CHUNK_SIZE as f32;

    // Create a quad mesh for the chunk
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices = vec![
        [0.0, 0.0, 0.0],
        [chunk_size, 0.0, 0.0],
        [chunk_size, chunk_size, 0.0],
        [0.0, chunk_size, 0.0],
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];
    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());
    mesh.set_indices(Some(Indices::U32(indices.clone())));

    // Spawn the main chunk mesh
    let color = match chunk_data.layer {
        TerrainLayer::Surface => Color::rgb(0.2, 0.8, 0.2), // Green
        TerrainLayer::Dirt => Color::rgb(0.6, 0.4, 0.2),    // Brown
        TerrainLayer::Stone => Color::rgb(0.5, 0.5, 0.5),   // Gray
        TerrainLayer::Deep => Color::rgb(0.3, 0.3, 0.3),    // Dark Gray
        TerrainLayer::Bedrock => Color::rgb(0.1, 0.1, 0.1), // Almost Black
    };

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(color)),
        transform: Transform::from_xyz(world_x, world_y, 0.0),
        ..default()
    });

    // Spawn features
    for feature in &chunk_data.features {
        let feature_size = chunk_size / 4.0;
        let feature_x = world_x + (feature.0.x as f32 * feature_size);
        let feature_y = world_y + (feature.0.y as f32 * feature_size);

        let feature_color = match feature.1 {
            TerrainFeature::Cave => Color::rgb(0.1, 0.1, 0.1), // Dark
            TerrainFeature::WaterPocket => Color::rgb(0.2, 0.2, 0.8), // Blue
            TerrainFeature::ResourceVein => Color::rgb(0.8, 0.8, 0.2), // Yellow
            TerrainFeature::Tunnel => Color::rgb(0.4, 0.3, 0.2), // Brown
            _ => Color::rgb(0.5, 0.5, 0.5), // Default gray for any new feature types
        };

        let mut feature_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let feature_vertices = vec![
            [0.0, 0.0, 0.0],
            [feature_size, 0.0, 0.0],
            [feature_size, feature_size, 0.0],
            [0.0, feature_size, 0.0],
        ];
        feature_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, feature_vertices);
        feature_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone());
        feature_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());
        feature_mesh.set_indices(Some(Indices::U32(indices.clone())));

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(feature_mesh).into(),
            material: materials.add(ColorMaterial::from(feature_color)),
            transform: Transform::from_xyz(feature_x, feature_y, 1.0),
            ..default()
        });
    }
}
