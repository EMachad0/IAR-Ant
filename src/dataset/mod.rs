use bevy::prelude::*;

mod data;

pub use data::*;

pub struct DatasetPlugin;

impl Plugin for DatasetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Dataset>()
            .init_asset_loader::<DatasetAssetLoader>()
            .init_resource::<DatasetHandle>()
            .add_startup_system(load_dataset);
    }
}

#[derive(Debug, Default, Deref)]
pub struct DatasetHandle(Handle<Dataset>);

pub fn load_dataset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("dataset/dataset_04.txt");
    commands.insert_resource(DatasetHandle(handle));
}
