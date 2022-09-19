use crate::{App, Board, Entity, Plugin, With};

pub struct DebugInspectorPlugin;

use crate::simulation::ant::Ant;
#[allow(unused_imports)]
use bevy_inspector_egui::{
    widgets::{InspectorQuery, ResourceInspector},
    Inspectable, InspectorPlugin, WorldInspectorPlugin,
};

#[derive(Default, Inspectable)]
pub struct AntInspector {
    ants: InspectorQuery<Entity, With<Ant>>,
}

#[derive(Default, Inspectable)]
pub struct BoardInspector {
    board: ResourceInspector<Board>,
}

impl Plugin for DebugInspectorPlugin {
    #[allow(unused_variables, path_statements)]
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                // .add_plugin(WorldInspectorPlugin::new())
                // .add_plugin(InspectorPlugin::<AntInspector>::new())
                // .add_plugin(InspectorPlugin::<BoardInspector>::new())
            ;
        }
    }
}
