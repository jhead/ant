use super::{behaviors::*, components::*};
use crate::colony::{Colony, ColonyMember};
use crate::terrain::{Tile, TILE_SIZE};
use bevy::prelude::*;

pub fn handle_mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ants: Query<&mut Ant>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_q.get_single() {
                    if let Some(world_pos) =
                        camera.viewport_to_world_2d(camera_transform, cursor_pos)
                    {
                        println!("Issuing move command to position: {:?}", world_pos);
                        for mut ant in ants.iter_mut() {
                            ant.command = AntCommand::MoveTo(world_pos);
                            ant.worker_state = WorkerState::SearchingForDigSite;
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
                speed: 50.0,
                direction: Vec2::new(1.0, 0.0),
                on_ground: false,
                command: AntCommand::Work,
                role: AntRole::Worker,
                worker_state: WorkerState::SearchingForDigSite,
                search_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            },
            ColonyMember { colony_id },
        ));

        println!("Worker ant spawned at (0,0) for colony {:?}", colony_id);
    } else {
        println!("Failed to spawn ant: no colony found");
    }
}

pub fn ant_movement(
    time: Res<Time>,
    mut query_set: ParamSet<(
        Query<(&mut Transform, &mut Ant, &ColonyMember)>,
        Query<(&Transform, &mut Tile, &mut Sprite)>,
        Query<&Transform, With<Colony>>,
    )>,
) {
    let mut ant_positions = Vec::new();
    let mut ant_on_grounds = Vec::new();
    let mut ant_directions = Vec::new();
    let mut ant_commands = Vec::new();
    let mut ant_worker_states = Vec::new();
    let mut ant_search_timers = Vec::new();
    let mut dig_positions = Vec::new();

    // Get colony position
    let colony_pos = if let Ok(colony_transform) = query_set.p2().get_single() {
        colony_transform.translation.truncate()
    } else {
        Vec2::ZERO
    };

    // Gather all terrain positions
    let terrain_positions: Vec<_> = query_set
        .p1()
        .iter()
        .filter(|(_, tile, _)| tile.is_solid)
        .map(|(transform, _, _)| transform.translation.truncate())
        .collect();

    // Process ant movement
    for (transform, ant, _colony_member) in query_set.p0().iter_mut() {
        let mut new_pos = transform.translation.truncate();
        let mut new_on_ground = false;
        let mut new_direction = ant.direction;
        let mut new_command = ant.command;
        let mut new_worker_state = ant.worker_state;
        let mut new_search_timer = ant.search_timer.clone();

        let distance_to_colony = (new_pos - colony_pos).length();
        if distance_to_colony > MAX_COLONY_DISTANCE {
            let to_colony = (colony_pos - new_pos).normalize();
            new_direction = to_colony;
            println!(
                "Ant too far from colony ({:.1}), returning",
                distance_to_colony
            );
        } else {
            match ant.command {
                AntCommand::MoveTo(target_pos) => {
                    if handle_move_command(new_pos, target_pos, &mut new_direction) {
                        new_command = AntCommand::Work;
                        println!("Move command complete, returning to work");
                    }
                }
                AntCommand::Work => match ant.role {
                    AntRole::Worker => {
                        if let Some(dig_pos) = handle_worker_work(
                            new_pos,
                            colony_pos,
                            &terrain_positions,
                            &mut new_worker_state,
                            &mut new_direction,
                            &mut new_search_timer,
                            &time,
                        ) {
                            dig_positions.push(dig_pos);
                        }
                    }
                },
            }
        }

        // Calculate and apply movement
        let movement = new_direction * ant.speed * time.delta_seconds();
        new_pos += movement;

        // Handle collisions
        let ant_size = 5.0;
        let half_size = ant_size / 2.0;
        let half_tile = TILE_SIZE / 2.0;

        for &terrain_pos in &terrain_positions {
            if new_pos.x + half_size > terrain_pos.x - half_tile
                && new_pos.x - half_size < terrain_pos.x + half_tile
                && new_pos.y + half_size > terrain_pos.y - half_tile
                && new_pos.y - half_size < terrain_pos.y + half_tile
            {
                if movement.y < 0.0 && transform.translation.y > terrain_pos.y {
                    new_pos.y = terrain_pos.y + half_tile + half_size;
                    new_on_ground = true;
                } else if movement.y > 0.0 && transform.translation.y < terrain_pos.y {
                    new_pos.y = terrain_pos.y - half_tile - half_size;
                } else if movement.x != 0.0 {
                    if transform.translation.x < terrain_pos.x {
                        new_pos.x = terrain_pos.x - half_tile - half_size;
                    } else {
                        new_pos.x = terrain_pos.x + half_tile + half_size;
                    }
                    new_direction.x *= -1.0;
                }
            }
        }

        ant_positions.push(new_pos);
        ant_on_grounds.push(new_on_ground);
        ant_directions.push(new_direction);
        ant_commands.push(new_command);
        ant_worker_states.push(new_worker_state);
        ant_search_timers.push(new_search_timer);
    }

    // Update terrain (dig)
    if !dig_positions.is_empty() {
        println!("Processing {} dig positions", dig_positions.len());
        let mut terrain_query = query_set.p1();
        for dig_pos in dig_positions {
            println!("Attempting to dig at position {:?}", dig_pos);
            let mut tiles_dug = 0;
            for (transform, mut tile, mut sprite) in terrain_query.iter_mut() {
                let tile_pos = transform.translation.truncate();
                if tile_pos.distance(dig_pos) < TILE_SIZE {
                    if tile.is_solid {
                        tile.is_solid = false;
                        sprite.color = Color::rgba(0.0, 0.0, 0.0, 0.0); // Make tile transparent
                        tiles_dug += 1;
                        println!(
                            "Dug tile at {:?} (distance: {:.1})",
                            tile_pos,
                            tile_pos.distance(dig_pos)
                        );
                    }
                }
            }
            println!("Dug {} tiles around position {:?}", tiles_dug, dig_pos);
        }
    }

    // Update ant states
    let mut ant_query = query_set.p0();
    for i in 0..ant_positions.len() {
        if let Some((mut transform, mut ant, _)) = ant_query.iter_mut().nth(i) {
            transform.translation.x = ant_positions[i].x;
            transform.translation.y = ant_positions[i].y;
            ant.on_ground = ant_on_grounds[i];
            ant.direction = ant_directions[i];
            ant.command = ant_commands[i];
            ant.worker_state = ant_worker_states[i];
            ant.search_timer = ant_search_timers[i].clone();
        }
    }
}
