use bevy::prelude::*;

// Size of each tile in pixels
pub const TILE_SIZE: f32 = 8.0;

#[derive(Component)]
pub struct Tile {
    pub is_solid: bool,
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain);
    }
}

fn spawn_terrain(mut commands: Commands) {
    // Calculate how many tiles we need based on window size
    let window_width = 800.0;
    let window_height = 600.0;
    let tiles_x = (window_width / TILE_SIZE).ceil() as i32;
    let tiles_y = (window_height / TILE_SIZE).ceil() as i32;

    // Center position for the circular cavity
    let center_x = 0.0;
    let center_y = 0.0;
    let cavity_radius = 50.0; // Radius in pixels

    for y in -tiles_y / 2..=tiles_y / 2 {
        for x in -tiles_x / 2..=tiles_x / 2 {
            let pos_x = x as f32 * TILE_SIZE;
            let pos_y = y as f32 * TILE_SIZE;

            // Calculate distance from center
            let distance = ((pos_x - center_x).powi(2) + (pos_y - center_y).powi(2)).sqrt();

            // Skip spawning tile if inside the cavity
            if distance < cavity_radius {
                continue;
            }

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.45, 0.29, 0.14), // Brown color
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..default()
                },
                Tile { is_solid: true },
            ));
        }
    }

    println!("Terrain generated with {}x{} tiles", tiles_x, tiles_y);
}
