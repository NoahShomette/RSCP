use bevy::prelude::*;
use bevy::window::{close_on_esc};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(close_on_esc)
        .run();
}