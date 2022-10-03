use bevy::prelude::*;

mod data;

use data::{ Dataset, DatasetAssetLoader };

pub struct DatasetPlugin;

impl Plugin for DatasetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Dataset>()
            .init_asset_loader::<DatasetAssetLoader>()
            .add_startup_system(load_dataset)
            .add_system(on_dataset_load);
    }
}

#[derive(Debug, Deref)]
pub struct DatasetHandle(Handle<Dataset>);

pub fn load_dataset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("dataset/dataset_04.txt");
    commands.insert_resource(DatasetHandle(handle));
}

pub fn on_dataset_load(
    mut ev_asset: EventReader<AssetEvent<Dataset>>,
    assets: ResMut<Assets<Dataset>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                let dataset = assets.get(handle).unwrap();
                for (k, v) in &dataset.data {
                    debug!("{:?}:", k);
                    for [x, y] in v {
                        trace!("{} {}", x, y);
                    }
                }
            }
            _ => (),
        }
    }
}
