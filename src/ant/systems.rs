use crate::ant::components::{
    Ant, AntCommand, AntRole, WorkerState, ANT_SPEED, MAX_COLONY_DISTANCE,
};
use crate::ant::pathfinding::{find_nearest_accessible_point, find_path};
use crate::colony::{Colony, ColonyMember};
use crate::terrain::TileStore;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn handle_mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&Transform, &mut Ant, &mut Velocity)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_q.get_single() {
                    if let Some(world_pos) =
                        camera.viewport_to_world_2d(camera_transform, cursor_pos)
                    {
                        println!("Issuing new move command to position: {:?}", world_pos);
                        for (transform, mut ant, mut velocity) in query.iter_mut() {
                            // Immediately stop current movement
                            velocity.linvel = Vec2::ZERO;

                            // Clear existing path
                            ant.current_path = None;
                            ant.current_path_index = 0;

                            // Set new target
                            ant.target_position = Some(world_pos);
                            ant.worker_state = WorkerState::SearchingForDigSite;

                            println!(
                                "Interrupted ant at {:?}, setting new target",
                                transform.translation.truncate()
                            );
                        }
                    }
                }
            }
        }
    }
}

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

pub fn ant_movement(
    mut query: Query<(&Transform, &mut Ant, &mut Velocity, &ColonyMember)>,
    colony_query: Query<&Colony>,
    _time: Res<Time>,
    tile_store: Res<TileStore>,
) {
    for (transform, mut ant, mut velocity, colony_member) in query.iter_mut() {
        if let Some(target_pos) = ant.target_position {
            let current_pos = transform.translation.truncate();

            // Get colony position and check distance
            if let Ok(colony) = colony_query.get(colony_member.colony_id) {
                let distance_to_colony = (colony.position - current_pos).length();
                if distance_to_colony > MAX_COLONY_DISTANCE {
                    println!(
                        "Target too far from colony ({} > {}), returning to colony",
                        distance_to_colony, MAX_COLONY_DISTANCE
                    );
                    ant.target_position = Some(colony.position);
                    continue;
                }
            }

            // Check if we're close enough to the final destination
            let distance_to_target = (target_pos - current_pos).length();
            if distance_to_target < 5.0 {
                println!(
                    "Reached final destination at {:?}, distance: {}",
                    current_pos, distance_to_target
                );
                velocity.linvel = Vec2::ZERO;
                ant.target_position = None;
                ant.current_path = None;
                ant.current_path_index = 0;
                continue;
            }

            // If we don't have a path or need to recalculate
            if ant.current_path.is_none()
                || ant.current_path_index >= ant.current_path.as_ref().unwrap().len()
            {
                println!("Finding path to target at {:?}", target_pos);

                // First, find the nearest accessible point to the target
                let solid_tiles = tile_store.get_solid_tiles();
                if let Some(accessible_point) =
                    find_nearest_accessible_point(current_pos, target_pos, &solid_tiles)
                {
                    println!("Found nearest accessible point at {:?}", accessible_point);

                    // If we're at the accessible point, start digging
                    let distance_to_accessible = (accessible_point - current_pos).length();
                    if distance_to_accessible < 5.0 {
                        println!("Reached accessible point, starting to dig towards target");
                        ant.worker_state = WorkerState::Digging(target_pos);
                        velocity.linvel = Vec2::ZERO;
                        continue;
                    }

                    // Otherwise, find a path to the accessible point
                    if let Some(path) = find_path(current_pos, accessible_point, &solid_tiles) {
                        println!(
                            "Found path with {} waypoints to accessible point",
                            path.len()
                        );
                        ant.current_path = Some(path);
                        ant.current_path_index = 0;
                    } else {
                        println!("No path found to accessible point");
                        velocity.linvel = Vec2::ZERO;
                        ant.target_position = None;
                        continue;
                    }
                } else {
                    println!("No accessible point found near target");
                    velocity.linvel = Vec2::ZERO;
                    ant.target_position = None;
                    continue;
                }
            }

            // Follow the current path
            let path_len = ant.current_path.as_ref().map(|p| p.len()).unwrap_or(0);
            let current_index = ant.current_path_index;

            if current_index < path_len {
                let next_waypoint = ant.current_path.as_ref().unwrap()[current_index];
                let to_target = next_waypoint - current_pos;
                let distance = to_target.length();

                // Increased threshold for waypoint detection
                if distance < 2.0 {
                    println!(
                        "Reached waypoint {} at {:?}, distance: {}",
                        current_index, next_waypoint, distance
                    );

                    // Update path index
                    ant.current_path_index += 1;

                    // Only zero velocity if we're at the final waypoint
                    if ant.current_path_index >= path_len {
                        velocity.linvel = Vec2::ZERO;
                    }
                } else {
                    let direction = to_target.normalize();
                    // Apply a stronger impulse when starting to move
                    if velocity.linvel.length() < 10.0 {
                        velocity.linvel = direction * ANT_SPEED * 1.5;
                    } else {
                        velocity.linvel = direction * ANT_SPEED;
                    }
                }
            } else {
                // We've reached the end of our path
                println!("Reached final destination at {:?}", current_pos);
                velocity.linvel = Vec2::ZERO;
                ant.target_position = None;
                ant.current_path = None;
                ant.current_path_index = 0;
            }
        } else {
            velocity.linvel = Vec2::ZERO;
        }
    }
}
