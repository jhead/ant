use crate::ant::components::{Ant, AntCommand, AntRole, WorkerState, ANT_SPEED};
use crate::colony::{Colony, ColonyMember};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod ant_movement;
pub mod mouse_click;

pub fn spawn_initial_ant(mut commands: Commands, colony_query: Query<Entity, With<Colony>>) {
    if let Ok(colony_id) = colony_query.get_single() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.6, 0.2),
                    custom_size: Some(Vec2::new(5.0, 5.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            Ant {
                speed: ANT_SPEED,
                direction: Vec2::new(1.0, 0.0),
                on_ground: false,
                command: AntCommand::Work,
                role: AntRole::Worker,
                worker_state: WorkerState::SearchingForDigSite,
                search_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                target_position: None,
                current_path: None,
                current_path_index: 0,
            },
            ColonyMember { colony_id },
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(2.5),         // Half the width of the sprite
            LockedAxes::ROTATION_LOCKED, // Prevent rotation
            Damping {
                linear_damping: 0.5, // Reduced damping for faster movement
                angular_damping: 1.0,
            },
        ));

        println!("Worker ant spawned at (0,0) for colony {:?}", colony_id);
    } else {
        println!("Failed to spawn ant: no colony found");
    }
}

pub fn handle_spacebar_spawn(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    colony_query: Query<Entity, With<Colony>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(colony_id) = colony_query.get_single() {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.6, 0.2),
                        custom_size: Some(Vec2::new(5.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 10.0),
                    ..default()
                },
                Ant {
                    speed: ANT_SPEED,
                    direction: Vec2::new(1.0, 0.0),
                    on_ground: false,
                    command: AntCommand::Work,
                    role: AntRole::Worker,
                    worker_state: WorkerState::SearchingForDigSite,
                    search_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                    target_position: None,
                    current_path: None,
                    current_path_index: 0,
                },
                ColonyMember { colony_id },
                RigidBody::Dynamic,
                Velocity::default(),
                Collider::ball(2.5),         // Half the width of the sprite
                LockedAxes::ROTATION_LOCKED, // Prevent rotation
                Damping {
                    linear_damping: 0.5, // Reduced damping for faster movement
                    angular_damping: 1.0,
                },
            ));

            println!("Worker ant spawned at (0,0) for colony {:?}", colony_id);
        } else {
            println!("Failed to spawn ant: no colony found");
        }
    }
}
