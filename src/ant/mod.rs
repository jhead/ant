mod behaviors;
mod components;
mod pathfinding;
mod systems;

use bevy::prelude::*;
use systems::{ant_movement, handle_mouse_click, handle_spacebar_spawn, spawn_initial_ant};

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_initial_ant).add_systems(
            Update,
            (ant_movement, handle_mouse_click, handle_spacebar_spawn),
        );
    }
}
