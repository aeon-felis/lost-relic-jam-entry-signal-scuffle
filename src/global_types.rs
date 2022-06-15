use bevy::prelude::*;
use ezinput::prelude::BindingTypeView;

#[derive(Clone, PartialEq)]
pub struct MenuActionForKbgp;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    Menu(MenuState),
    LoadLevel,
    Game,
    #[allow(dead_code)]
    LevelCompleted,
    Editor,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum MenuState {
    Main,
    LevelSelect,
    Pause,
    LevelCompleted,
    GameOver,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum GameSystemLabel {
    ApplyMovement,
}

pub struct LevelProgress {
    pub just_completed: Option<String>,
    pub current_level: Option<String>,
    pub num_levels_available: usize,
}

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum InputBinding {
    MoveHorizontal,
    MoveVertical,
    Grab,
}

#[derive(Component)]
pub struct IsPlayer;

#[derive(Component)]
pub struct IsZombie;

#[derive(Component)]
pub struct Grabbable;

#[derive(Component)]
pub enum GrabStatus {
    NoGrab,
    GrabFailed,
    Reaching { hands_entity: Entity, how_long: f32 },
    Holding { hands_entity: Entity, other: Entity },
}

#[derive(Component)]
pub struct IsWifi;

#[derive(Component)]
pub struct DoorStatus {
    pub is_open: bool,
}

#[derive(Default, Component)]
pub struct WifiClient {
    pub access_point: Option<Entity>,
    pub signal_strength: f32,
}

#[derive(Component)]
pub enum DownloadProgress {
    Disconnected,
    LosingConnection {
        time_before_disconnection: f32,
        progress: f32,
    },
    Downloading {
        progress: f32,
    },
    Completed,
}
