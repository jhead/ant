use super::components::*;
use bevy::prelude::*;
use rand::Rng;

pub fn find_dig_target(
    current_pos: Vec2,
    _colony_pos: Vec2,
    terrain_positions: &[Vec2],
) -> Option<Vec2> {
    let mut rng = rand::thread_rng();
    let mut closest_dirt = None;
    let mut min_distance = f32::MAX;

    // Decide if we should branch or continue downward
    let should_branch = rng.gen_bool(BRANCH_CHANCE as f64);
    let preferred_angle = if should_branch {
        // Branch left or right
        if rng.gen_bool(0.5) {
            PREFERRED_DIG_ANGLE
        } else {
            -PREFERRED_DIG_ANGLE
        }
    } else {
        // Continue downward
        -std::f32::consts::FRAC_PI_2
    };

    println!(
        "Searching for dig target from {:?} with angle {:.2}",
        current_pos, preferred_angle
    );

    for &pos in terrain_positions {
        let distance = current_pos.distance(pos);
        if distance < min_distance && distance <= WORKER_WORK_RADIUS {
            // Calculate angle between current position and potential dig target
            let angle = (pos - current_pos).angle_between(Vec2::new(1.0, 0.0));

            // Check if the angle is close to our preferred direction
            let angle_diff = (angle - preferred_angle).abs();
            if angle_diff < 0.5 {
                closest_dirt = Some(pos);
                min_distance = distance;
                println!(
                    "Found potential dig target at {:?} (distance: {:.1}, angle: {:.2})",
                    pos, distance, angle
                );
            }
        }
    }

    if let Some(target) = closest_dirt {
        println!("Selected dig target at {:?}", target);
    } else {
        println!("No suitable dig target found");
    }

    closest_dirt
}

pub fn find_search_direction(
    current_pos: Vec2,
    _colony_pos: Vec2,
    terrain_positions: &[Vec2],
) -> Vec2 {
    let mut rng = rand::thread_rng();
    let mut best_direction = Vec2::ZERO;
    let mut max_dirt_count = 0;

    println!("Searching for direction from {:?}", current_pos);

    // Try several random angles
    for _ in 0..MAX_SEARCH_ATTEMPTS {
        let angle = rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI);
        let direction = Vec2::new(angle.cos(), angle.sin());
        let search_pos = current_pos + direction * SEARCH_RADIUS;

        // Count dirt tiles in this direction
        let dirt_count = terrain_positions
            .iter()
            .filter(|&&pos| {
                let to_pos = pos - search_pos;
                to_pos.length() <= SEARCH_RADIUS && to_pos.dot(direction) > 0.0
            })
            .count();

        if dirt_count > max_dirt_count {
            max_dirt_count = dirt_count;
            best_direction = direction;
            println!(
                "Found better search direction: {:?} with {} dirt tiles",
                direction, dirt_count
            );
        }
    }

    if max_dirt_count == 0 {
        // If no dirt found, move in a random direction
        let angle = rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI);
        let direction = Vec2::new(angle.cos(), angle.sin());
        println!("No dirt found, using random direction: {:?}", direction);
        direction
    } else {
        println!(
            "Selected search direction: {:?} with {} dirt tiles",
            best_direction, max_dirt_count
        );
        best_direction
    }
}

pub fn handle_worker_work(
    current_pos: Vec2,
    colony_pos: Vec2,
    terrain_positions: &[Vec2],
    worker_state: &mut WorkerState,
    direction: &mut Vec2,
    search_timer: &mut Timer,
    time: &Time,
) -> Option<Vec2> {
    match *worker_state {
        WorkerState::SearchingForDigSite => {
            search_timer.tick(time.delta());
            if search_timer.just_finished() {
                println!("Search timer finished, looking for new dig target");
                if let Some(dig_target) =
                    find_dig_target(current_pos, colony_pos, terrain_positions)
                {
                    *worker_state = WorkerState::MovingToDigSite(dig_target);
                    *direction = (dig_target - current_pos).normalize();
                    println!("Moving to dig site at {:?}", dig_target);
                } else {
                    *direction = find_search_direction(current_pos, colony_pos, terrain_positions);
                    println!(
                        "No dig target found, searching in direction {:?}",
                        direction
                    );
                }
            }
            None
        }
        WorkerState::MovingToDigSite(target_pos) => {
            let distance = current_pos.distance(target_pos);
            println!(
                "Distance to dig site: {:.1} (need to be within {:.1})",
                distance, COMMAND_COMPLETE_DISTANCE
            );
            if distance < COMMAND_COMPLETE_DISTANCE {
                let dig_pos = target_pos;
                println!("Reached dig site at {:?}, starting to dig", dig_pos);
                *worker_state = WorkerState::Digging(dig_pos);
                Some(dig_pos)
            } else {
                *direction = (target_pos - current_pos).normalize();
                None
            }
        }
        WorkerState::Digging(pos) => {
            let dig_pos = pos;
            println!("Digging at position {:?}", dig_pos);
            *worker_state = WorkerState::SearchingForDigSite;
            Some(dig_pos)
        }
    }
}

pub fn handle_move_command(current_pos: Vec2, target_pos: Vec2, direction: &mut Vec2) -> bool {
    let distance = current_pos.distance(target_pos);
    if distance < COMMAND_COMPLETE_DISTANCE {
        true
    } else {
        *direction = (target_pos - current_pos).normalize();
        false
    }
}
