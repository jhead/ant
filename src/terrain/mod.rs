use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub const TILE_SIZE: f32 = 8.0;

// Define a trait for different tile types
pub trait TileType: Send + Sync {
    fn is_solid(&self) -> bool;
    fn color(&self) -> Color;
    fn name(&self) -> &str;
    fn clone_box(&self) -> Box<dyn TileType>;
}

// Concrete implementations for different tile types
#[derive(Component, Clone, Copy)]
pub struct DirtTile;

impl TileType for DirtTile {
    fn is_solid(&self) -> bool {
        true
    }

    fn color(&self) -> Color {
        Color::rgb(0.8, 0.6, 0.4) // Even more visible brown color
    }

    fn name(&self) -> &str {
        "Dirt"
    }

    fn clone_box(&self) -> Box<dyn TileType> {
        Box::new(*self)
    }
}

#[derive(Component, Clone, Copy)]
pub struct AirTile;

impl TileType for AirTile {
    fn is_solid(&self) -> bool {
        false
    }

    fn color(&self) -> Color {
        Color::rgba(0.0, 0.0, 0.0, 0.0) // Transparent
    }

    fn name(&self) -> &str {
        "Air"
    }

    fn clone_box(&self) -> Box<dyn TileType> {
        Box::new(*self)
    }
}

// Wrapper for Vec2 to implement Hash and Eq
#[derive(Debug, Clone, Copy)]
pub struct TilePosition(pub Vec2);

impl Hash for TilePosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert to fixed precision to avoid floating point issues
        let x = (self.0.x * 100.0).round() as i32;
        let y = (self.0.y * 100.0).round() as i32;
        x.hash(state);
        y.hash(state);
    }
}

impl PartialEq for TilePosition {
    fn eq(&self, other: &Self) -> bool {
        // Compare with a small epsilon to handle floating point imprecision
        let epsilon = 0.01;
        (self.0.x - other.0.x).abs() < epsilon && (self.0.y - other.0.y).abs() < epsilon
    }
}

impl Eq for TilePosition {}

// Updated Tile struct to use the TileType trait
#[derive(Component)]
pub struct Tile {
    pub position: Vec2,
    pub tile_type: Box<dyn TileType>,
}

// Implement Clone manually for Tile
impl Clone for Tile {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            tile_type: self.tile_type.clone_box(),
        }
    }
}

// Updated TileStore to use the new Tile struct and track entity IDs
#[derive(Resource, Default)]
pub struct TileStore {
    tiles: HashMap<TilePosition, Tile>,
}

impl TileStore {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, position: Vec2, tile_type: Box<dyn TileType>) {
        let tile_pos = TilePosition(position);
        let tile = Tile {
            position,
            tile_type: tile_type.clone_box(),
        };
        self.tiles.insert(tile_pos, tile);
    }

    pub fn set_tile_type(&mut self, position: &Vec2, tile_type: Box<dyn TileType>) -> bool {
        let tile_pos = TilePosition(*position);
        if let Some(tile) = self.tiles.get_mut(&tile_pos) {
            tile.tile_type = tile_type;
            true
        } else {
            false
        }
    }

    pub fn get_tile(&self, position: &Vec2) -> Option<&Tile> {
        let tile_pos = TilePosition(*position);
        self.tiles.get(&tile_pos)
    }

    pub fn get_tile_mut(&mut self, position: &Vec2) -> Option<&mut Tile> {
        let tile_pos = TilePosition(*position);
        self.tiles.get_mut(&tile_pos)
    }

    pub fn is_solid(&self, position: &Vec2) -> bool {
        let tile_pos = TilePosition(*position);
        self.tiles
            .get(&tile_pos)
            .map_or(false, |tile| tile.tile_type.is_solid())
    }

    pub fn get_solid_tiles(&self) -> Vec<Vec2> {
        self.tiles
            .iter()
            .filter(|(_, tile)| tile.tile_type.is_solid())
            .map(|(_, tile)| tile.position)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.tiles.len()
    }

    pub fn solid_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|(_, tile)| tile.tile_type.is_solid())
            .count()
    }
}

#[derive(Event)]
pub struct TileUpdateEvent {
    pub entity: Entity,
    pub new_type: Box<dyn TileType>,
}

#[derive(Resource, Default)]
pub struct TerrainMaterials {
    dirt: Handle<ColorMaterial>,
    air: Handle<ColorMaterial>,
}

impl TerrainMaterials {
    pub fn get_material(&self, tile_type: &dyn TileType) -> Handle<ColorMaterial> {
        if tile_type.is_solid() {
            self.dirt.clone()
        } else {
            self.air.clone()
        }
    }
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileStore>()
            .init_resource::<TerrainMaterials>()
            .add_event::<TileUpdateEvent>()
            .add_systems(
                Startup,
                (
                    setup_terrain_materials,
                    setup_terrain.after(setup_terrain_materials),
                    spawn_tile_entities.after(setup_terrain),
                )
                    .chain(),
            )
            .add_systems(Update, handle_tile_updates);
    }
}

fn setup_terrain_materials(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut terrain_materials: ResMut<TerrainMaterials>,
) {
    info!("Setting up terrain materials");
    terrain_materials.dirt = materials.add(ColorMaterial::from(Color::rgb(0.8, 0.6, 0.4)));
    terrain_materials.air = materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0)));
    info!("Terrain materials initialized");
}

pub fn setup_terrain(mut tile_store: ResMut<TileStore>) {
    info!("Starting terrain setup");
    // First pass: Create the initial terrain
    for y in -50..=50 {
        for x in -50..=50 {
            let pos = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
            let tile = Box::new(DirtTile);
            tile_store.add_tile(pos, tile);
        }
    }
    info!("Initial terrain created with {} tiles", tile_store.count());

    // Second pass: Create the cavity
    create_central_cavity(&mut tile_store);
    info!(
        "Terrain setup complete with {} solid tiles",
        tile_store.solid_count()
    );
}

pub fn create_central_cavity(tile_store: &mut ResMut<TileStore>) {
    let center = Vec2::new(0.0, 0.0);
    let radius = 40.0;
    let mut converted_count = 0;

    // Create a list of positions to convert to air
    let mut positions_to_convert = Vec::new();
    for y in -50..=50 {
        for x in -50..=50 {
            let pos = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
            let distance = pos.distance(center);
            if distance <= radius {
                positions_to_convert.push(pos);
            }
        }
    }

    info!(
        "Found {} positions to convert to air",
        positions_to_convert.len()
    );

    // Convert the tiles to air
    for pos in positions_to_convert {
        if let Some(tile) = tile_store.get_tile_mut(&pos) {
            if tile.tile_type.is_solid() {
                tile.tile_type = Box::new(AirTile);
                converted_count += 1;
            }
        }
    }

    info!(
        "Created central cavity by converting {} tiles to air",
        converted_count
    );
    info!("Remaining solid tiles: {}", tile_store.solid_count());
}

pub fn spawn_tile_entities(
    mut commands: Commands,
    tile_store: Res<TileStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    terrain_materials: Res<TerrainMaterials>,
) {
    info!("Starting to spawn tile entities");
    info!("Tile store contains {} tiles", tile_store.count());
    info!(
        "Terrain materials resource exists: {}",
        terrain_materials.is_added()
    );

    let quad_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        TILE_SIZE, TILE_SIZE,
    ))));
    info!("Created quad mesh for tiles");

    let mut spawned_count = 0;
    for (_, tile) in tile_store.tiles.iter() {
        let material = terrain_materials.get_material(tile.tile_type.as_ref());
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: quad_mesh.clone().into(),
                material,
                transform: Transform::from_xyz(tile.position.x, tile.position.y, 0.0),
                ..default()
            },
            tile.clone(),
        ));
        spawned_count += 1;
    }
    info!("Finished spawning {} tile entities", spawned_count);
}

fn handle_tile_updates(
    mut tile_update_events: EventReader<TileUpdateEvent>,
    mut tiles: Query<(&mut Tile, &mut Handle<ColorMaterial>)>,
    terrain_materials: Res<TerrainMaterials>,
) {
    for event in tile_update_events.read() {
        if let Ok((mut tile, mut material)) = tiles.get_mut(event.entity) {
            tile.tile_type = event.new_type.clone_box();
            *material = terrain_materials.get_material(tile.tile_type.as_ref());
        }
    }
}
