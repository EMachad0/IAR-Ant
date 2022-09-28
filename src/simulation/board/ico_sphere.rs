use crate::consts::{BOARD_RADIUS, BOARD_SUBDIVISIONS};
use crate::BoardPosition;
pub use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use hexasphere::AdjacentStore;
use parking_lot::RwLock;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::{BTreeSet, VecDeque};
use std::sync::Arc;

use crate::simulation::board::Cell;

#[derive(Component)]
pub struct BoardSphere;

#[derive(Debug, Default, Clone)]
pub struct IcoBoard {
    pub adj: Vec<Vec<usize>>,
    pub vertex: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub cells: Vec<Arc<RwLock<Cell>>>,
}

impl IcoBoard {
    pub fn new_random_position(&self) -> BoardPosition {
        let idx = rand::thread_rng().gen_range(0..self.size());
        BoardPosition::new(idx)
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn get_cell(&self, pos: &BoardPosition) -> Arc<RwLock<Cell>> {
        self.cells[pos.idx()].clone()
    }

    pub fn world_position(&self, pos: &BoardPosition) -> [f32; 3] {
        self.vertex[pos.idx()]
    }

    pub fn get_all_adjacent(&self, pos: &BoardPosition, radius: u32) -> Vec<BoardPosition> {
        let mut adj = BTreeSet::new();
        let mut queue = VecDeque::from([(0, pos.idx())]);
        while !queue.is_empty() {
            let (d, u) = queue.pop_front().unwrap();
            adj.insert(u);
            if d < radius {
                for v in &self.adj[u] {
                    if *v != pos.idx() && !adj.contains(v) {
                        queue.push_back((d + 1, *v))
                    }
                }
            }
        }
        adj.iter().map(|f| BoardPosition::new(*f)).collect()
    }

    pub fn get_random_adjacent(&self, pos: &BoardPosition) -> BoardPosition {
        let mut rng = rand::thread_rng();
        let idx = self.adj[pos.idx()].choose(&mut rng).unwrap();
        BoardPosition::new(*idx)
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
                .map(|i| *i as usize)
                .collect()
        })
        .collect();

    info!("Board with {} positions!", vertex.len());

    let cells: Vec<_> = std::iter::repeat_with(|| Arc::new(RwLock::new(Cell::default())))
        .take(vertex.len())
        .collect();

    commands.insert_resource(IcoBoard {
        adj,
        vertex: vertex.clone(),
        indices: indices.clone(),
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
