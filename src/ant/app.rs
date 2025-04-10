use super::AntPlugin;
use crate::colony::ColonyPlugin;
use crate::terrain::TerrainPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ant Farm".to_string(),
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
