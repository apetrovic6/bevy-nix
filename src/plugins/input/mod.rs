use bevy::{app::Plugin, reflect::Reflect};
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Walk,
    Jump,
}
