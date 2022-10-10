use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics};
use bevy::prelude::*;

use crate::dataset::{Dataset, DatasetHandle};
use crate::simulation::item;
use crate::simulation::item::Item;
use crate::{BoardPosition, IcoBoard};

#[derive(Default)]
pub struct SimilarityDiagnosticsPlugin;

impl Plugin for SimilarityDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::setup_system)
            .add_system(Self::diagnostic_system);
    }
}

impl SimilarityDiagnosticsPlugin {
    pub const SIMILARITY: DiagnosticId =
        DiagnosticId::from_u128(28817418947123934917808528866909494);

    pub fn setup_system(
        mut diagnostics: ResMut<Diagnostics>,
        mut ev_asset: EventReader<AssetEvent<Dataset>>,
        assets: ResMut<Assets<Dataset>>,
        dataset_handle: Res<DatasetHandle>,
    ) {
        for ev in ev_asset.iter() {
            match ev {
                AssetEvent::Created { handle } => {
                    if *handle != **dataset_handle {
                        continue;
                    }
                    debug!("Added Diagnostic");

                    let dataset = assets.get(handle).unwrap();
                    let item_count = dataset.data.len();

                    diagnostics.add(Diagnostic::new(Self::SIMILARITY, "similarity", item_count));
                }
                _ => (),
            }
        }
    }

    pub fn diagnostic_system(
        mut diagnostics: ResMut<Diagnostics>,
        query: Query<(Entity, &BoardPosition), With<Item>>,
        item_query: Query<&Item>,
        board: Res<IcoBoard>,
    ) {
        for (entity, pos) in &query {
            let similarity = item::compute_similarity(entity, pos, &*board, &item_query);
            diagnostics.add_measurement(Self::SIMILARITY, || similarity);
        }
    }
}
