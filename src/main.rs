use bevy::prelude::*;

mod ant;
mod colony;
mod terrain;

use ant::AntPlugin;
use colony::ColonyPlugin;
use terrain::TerrainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ant Farm".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((TerrainPlugin, ColonyPlugin, AntPlugin))
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    });
}
