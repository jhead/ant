use wasm_bindgen::prelude::*;

// Re-export all modules
pub mod ant;
pub mod colony;
pub mod terrain;

use ant::AntPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use colony::ColonyPlugin;
use terrain::TerrainPlugin;

pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ant 123 Farm".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins((TerrainPlugin, ColonyPlugin, AntPlugin))
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO, // No gravity for top-down movement
            ..default()
        })
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        camera: Camera {
            order: 0,
            ..default()
        },
        ..default()
    });
}

#[wasm_bindgen]
pub fn start_wasm() -> Result<(), JsValue> {
    // Set up console error panic hook for better error messages in the browser
    console_error_panic_hook::set_once();

    // Log that the WASM module is starting
    web_sys::console::log_1(&JsValue::from_str("Starting Ant Farm Simulation..."));

    // Run the main Bevy app
    ant::run_app();

    Ok(())
}
