use crate::colony::{Colony, ColonyMember};
use crate::terrain::{Tile, TILE_SIZE};
use bevy::prelude::*;
use rand::prelude::*;

const MAX_COLONY_DISTANCE: f32 = 500.0;
const COMMAND_COMPLETE_DISTANCE: f32 = 5.0;
const WORKER_WORK_RADIUS: f32 = 400.0;
const DIG_CHANCE: f32 = 0.8; // Higher chance to start digging when dirt is found
const BRANCH_CHANCE: f32 = 0.05;
const PREFERRED_DIG_ANGLE: f32 = std::f32::consts::FRAC_PI_4;
const SEARCH_RADIUS: f32 = 100.0; // How far to look for dirt when searching
const MAX_SEARCH_ATTEMPTS: i32 = 8; // Number of random directions to try when searching

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntCommand {
    MoveTo(Vec2),
    Work,
}

impl Default for AntCommand {
    fn default() -> Self {
        Self::Work
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntRole {
    Worker,
}

impl Default for AntRole {
    fn default() -> Self {
        Self::Worker
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum WorkerState {
    SearchingForDigSite,
    MovingToDigSite(Vec2), // Moving to a promising area to dig
    Digging(Vec2),         // Actually digging at a specific position
}

#[derive(Component)]
pub struct Ant {
    pub speed: f32,
    pub direction: Vec2,
    pub on_ground: bool,
    pub command: AntCommand,
    pub role: AntRole,
    worker_state: WorkerState,
    search_timer: Timer, // Timer to control how often we change search direction
}

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_initial_ant)
            .add_systems(Update, (ant_movement, handle_mouse_click));
    }
}

// Handle mouse clicks to issue move commands
fn handle_mouse_click(
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
                        // Issue move command to all ants
                        for mut ant in ants.iter_mut() {
                            ant.command = AntCommand::MoveTo(world_pos);
                            ant.worker_state = WorkerState::SearchingForDigSite;
                            // Reset work state when given manual command
                        }
                    }
                }
            }
        }
    }
}

fn spawn_initial_ant(mut commands: Commands, colony_query: Query<Entity, With<Colony>>) {
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

// Find the nearest terrain tile that can be dug, with improved search
fn find_dig_target(
    current_pos: Vec2,
    terrain_positions: &[Vec2],
    rng: &mut ThreadRng,
) -> Option<Vec2> {
    // First try to find any terrain within immediate vicinity
    let mut nearest_terrain: Option<(Vec2, f32)> = None;

    for &pos in terrain_positions {
        let distance = current_pos.distance(pos);
        if distance <= SEARCH_RADIUS {
            if let Some((_, best_distance)) = nearest_terrain {
                if distance < best_distance {
                    nearest_terrain = Some((pos, distance));
                }
            } else {
                nearest_terrain = Some((pos, distance));
            }
        }
    }

    nearest_terrain.map(|(pos, _)| {
        if rng.gen::<f32>() < BRANCH_CHANCE {
            // Start a new branch at a random downward angle
            let random_angle = PREFERRED_DIG_ANGLE * (0.5 + rng.gen::<f32>());
            let offset = Vec2::new(random_angle.cos(), -random_angle.sin()) * TILE_SIZE;
            pos + offset
        } else {
            // Continue existing tunnel downward
            pos + Vec2::new(0.0, -TILE_SIZE)
        }
    })
}

// Find a promising direction to search for dirt
fn find_search_direction(
    current_pos: Vec2,
    terrain_positions: &[Vec2],
    rng: &mut ThreadRng,
) -> Option<Vec2> {
    let mut best_direction = None;
    let mut most_dirt = 0;

    // Try several random directions and count how much dirt is in each direction
    for _ in 0..MAX_SEARCH_ATTEMPTS {
        let angle = rng.gen::<f32>() * std::f32::consts::TAU;
        let direction = Vec2::new(angle.cos(), angle.sin());
        let search_pos = current_pos + direction * SEARCH_RADIUS;

        // Count dirt tiles in this direction
        let dirt_count = terrain_positions
            .iter()
            .filter(|&&pos| pos.distance(search_pos) < SEARCH_RADIUS)
            .count();

        if dirt_count > most_dirt {
            most_dirt = dirt_count;
            best_direction = Some(search_pos);
        }
    }

    best_direction
}

// Role-specific work behaviors
fn handle_worker_work(
    current_pos: Vec2,
    colony_pos: Vec2,
    terrain_positions: &[Vec2],
    worker_state: &mut WorkerState,
    new_direction: &mut Vec2,
    search_timer: &mut Timer,
    time: &Time,
) -> Option<Vec2> {
    let mut rng = thread_rng();
    let to_colony = colony_pos - current_pos;
    let distance_to_colony = to_colony.length();

    // Always respect colony distance
    if distance_to_colony > WORKER_WORK_RADIUS {
        *new_direction = to_colony.normalize();
        *worker_state = WorkerState::SearchingForDigSite;
        return None;
    }

    match worker_state {
        WorkerState::SearchingForDigSite => {
            search_timer.tick(time.delta());

            if search_timer.just_finished() {
                // Look for a promising direction with dirt
                if let Some(search_pos) =
                    find_search_direction(current_pos, terrain_positions, &mut rng)
                {
                    *worker_state = WorkerState::MovingToDigSite(search_pos);
                    *new_direction = (search_pos - current_pos).normalize();
                    println!("Found promising dig site, moving to investigate");
                } else {
                    // Random movement if no good direction found
                    let angle = rng.gen::<f32>() * std::f32::consts::TAU;
                    *new_direction = Vec2::new(angle.cos(), angle.sin());
                }
            }

            // While moving, constantly check for nearby dirt
            if let Some(dig_target) = find_dig_target(current_pos, terrain_positions, &mut rng) {
                if rng.gen::<f32>() < DIG_CHANCE {
                    *worker_state = WorkerState::Digging(dig_target);
                    *new_direction = (dig_target - current_pos).normalize();
                    println!("Found dirt while searching, starting to dig");
                }
            }
            None
        }
        WorkerState::MovingToDigSite(target_pos) => {
            let to_target = *target_pos - current_pos;
            if to_target.length() < COMMAND_COMPLETE_DISTANCE {
                // Reached the search area, look for actual dig target
                if let Some(dig_target) = find_dig_target(current_pos, terrain_positions, &mut rng)
                {
                    *worker_state = WorkerState::Digging(dig_target);
                    *new_direction = (dig_target - current_pos).normalize();
                    println!("Reached search area, found dig target");
                } else {
                    // No dirt found at search area, resume searching
                    *worker_state = WorkerState::SearchingForDigSite;
                    println!("No dirt found at search area, resuming search");
                }
            } else {
                *new_direction = to_target.normalize();

                // While moving, still check for dirt we might pass by
                if let Some(dig_target) = find_dig_target(current_pos, terrain_positions, &mut rng)
                {
                    *worker_state = WorkerState::Digging(dig_target);
                    *new_direction = (dig_target - current_pos).normalize();
                    println!("Found dirt while moving to search area");
                }
            }
            None
        }
        WorkerState::Digging(target) => {
            let to_target = *target - current_pos;
            if to_target.length() < COMMAND_COMPLETE_DISTANCE {
                // We've reached the dig target
                let result = Some(*target);
                // Find next dig target
                if let Some(new_target) = find_dig_target(current_pos, terrain_positions, &mut rng)
                {
                    *worker_state = WorkerState::Digging(new_target);
                    *new_direction = (new_target - current_pos).normalize();
                    println!("Completed dig, moving to next target");
                } else {
                    *worker_state = WorkerState::SearchingForDigSite;
                    println!("No more nearby dig targets, resuming search");
                }
                result
            } else {
                *new_direction = to_target.normalize();
                None
            }
        }
    }
}

// Handle movement to a target position, returns true if the command is complete
fn handle_move_command(current_pos: Vec2, target_pos: Vec2, new_direction: &mut Vec2) -> bool {
    let to_target = target_pos - current_pos;
    let distance_to_target = to_target.length();

    if distance_to_target <= COMMAND_COMPLETE_DISTANCE {
        return true;
    }

    *new_direction = to_target.normalize();
    false
}

fn ant_movement(
    time: Res<Time>,
    mut query_set: ParamSet<(
        Query<(&mut Transform, &mut Ant, &ColonyMember)>,
        Query<(&Transform, &mut Tile)>,
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
        .filter(|(_, tile)| tile.is_solid)
        .map(|(transform, _)| transform.translation.truncate())
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
        let mut terrain_query = query_set.p1();
        for dig_pos in dig_positions {
            for (transform, mut tile) in terrain_query.iter_mut() {
                let tile_pos = transform.translation.truncate();
                if tile_pos.distance(dig_pos) < TILE_SIZE {
                    tile.is_solid = false;
                    println!("Dug tile at {:?}", tile_pos);
                }
            }
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
