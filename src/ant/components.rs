use bevy::prelude::*;

// Constants
pub const MAX_COLONY_DISTANCE: f32 = 500.0;
pub const COMMAND_COMPLETE_DISTANCE: f32 = 10.0;
pub const WORKER_WORK_RADIUS: f32 = 400.0;
pub const DIG_CHANCE: f32 = 0.8;
pub const BRANCH_CHANCE: f32 = 0.05;
pub const PREFERRED_DIG_ANGLE: f32 = std::f32::consts::FRAC_PI_4;
pub const SEARCH_RADIUS: f32 = 100.0;
pub const MAX_SEARCH_ATTEMPTS: i32 = 8;
pub const ANT_SPEED: f32 = 100.0;

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
pub enum WorkerState {
    SearchingForDigSite,
    MovingToDigSite(Vec2),
    Digging(Vec2),
}

#[derive(Component)]
pub struct Ant {
    pub speed: f32,
    pub direction: Vec2,
    pub on_ground: bool,
    pub command: AntCommand,
    pub role: AntRole,
    pub worker_state: WorkerState,
    pub search_timer: Timer,
    pub target_position: Option<Vec2>,
    pub current_path: Option<Vec<Vec2>>,
    pub current_path_index: usize,
}

impl Default for Ant {
    fn default() -> Self {
        Self {
            speed: ANT_SPEED,
            direction: Vec2::ZERO,
            on_ground: false,
            command: AntCommand::default(),
            role: AntRole::default(),
            worker_state: WorkerState::SearchingForDigSite,
            search_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            target_position: None,
            current_path: None,
            current_path_index: 0,
        }
    }
}
