use bevy::prelude::*;

pub struct DebugInspectorPlugin;

use crate::simulation::ant::Ant;
// use crate::simulation::food::Food;
use crate::timestep::FixedTimestepConfig;
use crate::simulation::board::IcoBoard;

#[allow(unused_imports)]
use bevy_inspector_egui::{
    widgets::{InspectorQuery, ResourceInspector},
    Inspectable, InspectorPlugin, WorldInspectorPlugin,
};

#[derive(Default, Inspectable)]
pub struct AntInspector {
    ants: InspectorQuery<Entity, With<Ant>>,
}

// #[derive(Default, Inspectable)]
// pub struct FoodInspector {
//     foods: InspectorQuery<Entity, With<Food>>,
// }

#[derive(Default, Inspectable)]
pub struct BoardInspector {
    board: ResourceInspector<IcoBoard>,
}

#[derive(Default, Inspectable)]
pub struct SimulationControlInspector {
    config: ResourceInspector<FixedTimestepConfig>,
}

impl Plugin for DebugInspectorPlugin {
    #[allow(unused_variables, path_statements)]
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
                // .add_plugin(WorldInspectorPlugin::new())
                // .add_plugin(InspectorPlugin::<AntInspector>::new())
                // .add_plugin(InspectorPlugin::<BoardInspector>::new())
                // .add_plugin(InspectorPlugin::<FoodInspector>::new())
                // .add_plugin(InspectorPlugin::<SimulationControlInspector>::new())
            ;
        }
    }
}
