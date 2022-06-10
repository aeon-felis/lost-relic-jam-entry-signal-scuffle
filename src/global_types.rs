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
}

#[derive(Component)]
pub struct IsPlayer;
