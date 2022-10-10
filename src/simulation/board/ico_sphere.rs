use crate::consts::{BOARD_RADIUS, BOARD_SUBDIVISIONS};
use crate::BoardPosition;
pub use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy_inspector_egui::Inspectable;
use hexasphere::AdjacentStore;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::simulation::board::Cell;

#[derive(Component)]
pub struct BoardSphere;

#[derive(Debug, Default, Clone, Reflect, Inspectable)]
#[reflect(Resource)]
pub struct IcoBoard {
    pub adj: Vec<Vec<BoardPosition>>,
    pub vertex: Vec<[f32; 3]>,
    pub cells: Vec<Cell>,
}

impl IcoBoard {
    pub fn new_random_position(&self) -> BoardPosition {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.size());
        BoardPosition::new(idx)
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn get_cell(&self, pos: &BoardPosition) -> &Cell {
        &self.cells[pos.idx()]
    }

    pub fn get_cell_mut(&mut self, pos: &BoardPosition) -> &mut Cell {
        &mut self.cells[pos.idx()]
    }

    pub fn world_position(&self, pos: &BoardPosition) -> [f32; 3] {
        self.vertex[pos.idx()]
    }

    pub fn get_all_adjacent(&self, pos: &BoardPosition) -> &Vec<BoardPosition> {
        &self.adj[pos.idx()]
    }

    pub fn get_random_adjacent(&self, pos: &BoardPosition) -> &BoardPosition {
        let mut rng = rand::thread_rng();
        self.adj[pos.idx()].choose(&mut rng).unwrap()
    }
}

pub fn icosphere_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(shape::Icosphere {
        radius: BOARD_RADIUS,
        subdivisions: BOARD_SUBDIVISIONS,
    });

    let vertex = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
        VertexAttributeValues::Float32x3(v) => v,
        _ => panic!("Unexpected Attribute Format"),
    };

    let indices = match mesh.indices().unwrap() {
        Indices::U32(v) => v,
        Indices::U16(_) => panic!("Unexpected U16 indices"),
    };

    let adj_store = AdjacentStore::from_indices(indices);
    let adj = (0..vertex.len())
        .map(|i| {
            adj_store
                .neighbours(i as u32)
                .unwrap()
                .iter()
                .map(|i| BoardPosition::new(*i as usize))
                .collect()
        })
        .collect();

    info!("Board with {} positions!", vertex.len());

    let cells = vec![Cell::default(); vertex.len()];

    commands.insert_resource(IcoBoard {
        adj,
        vertex: vertex.clone(),
        cells,
    });

    let mesh_handle = meshes.add(mesh);

    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh_handle,
            material: materials.add(Color::GREEN.into()),
            ..default()
        })
        .insert(BoardSphere);
}
