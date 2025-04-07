mod camera;
pub mod resources;
mod terrain;
mod ui;

pub use camera::*;
pub use terrain::*;
pub use ui::*;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainCache>()
            .add_systems(Startup, (setup_camera, setup_ui, generate_initial_chunks))
            .add_systems(Update, (move_camera, generate_chunks_around_camera));
    }
}

pub fn generate_initial_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut terrain_cache: ResMut<TerrainCache>,
) {
    // Generate chunks around the origin
    for x in -CHUNK_LOAD_DISTANCE..=CHUNK_LOAD_DISTANCE {
        for y in -CHUNK_LOAD_DISTANCE..=CHUNK_LOAD_DISTANCE {
            let chunk_pos = IVec2::new(x, y);
            let chunk = generate_chunk_data(&mut terrain_cache, chunk_pos);
            terrain_cache.chunks.insert(chunk_pos, chunk.clone());
            spawn_chunk_entity(
                &mut commands,
                &mut meshes,
                &mut materials,
                chunk_pos,
                &chunk,
            );
        }
    }
}

pub fn generate_chunks_around_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut terrain_cache: ResMut<TerrainCache>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_pos = camera_transform.translation.truncate();

        let camera_chunk_x = (camera_pos.x / CHUNK_SIZE as f32).floor() as i32;
        let camera_chunk_y = (camera_pos.y / CHUNK_SIZE as f32).floor() as i32;

        for x_offset in -CHUNK_LOAD_DISTANCE..=CHUNK_LOAD_DISTANCE {
            for y_offset in -CHUNK_LOAD_DISTANCE..=CHUNK_LOAD_DISTANCE {
                let chunk_x = if x_offset < 0 {
                    camera_chunk_x.saturating_sub(-x_offset)
                } else {
                    camera_chunk_x.saturating_add(x_offset)
                };

                let chunk_y = if y_offset < 0 {
                    camera_chunk_y.saturating_sub(-y_offset)
                } else {
                    camera_chunk_y.saturating_add(y_offset)
                };

                let chunk_pos = IVec2::new(chunk_x, chunk_y);

                if !terrain_cache.chunks.contains_key(&chunk_pos) {
                    let chunk = generate_chunk_data(&mut terrain_cache, chunk_pos);
                    terrain_cache.chunks.insert(chunk_pos, chunk.clone());
                    spawn_chunk_entity(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        chunk_pos,
                        &chunk,
                    );
                }
            }
        }
    }
}
