use crate::consts::{BOARD_RADIUS, BOARD_SUBDIVISIONS};
use crate::BoardPosition;
pub use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy_inspector_egui::Inspectable;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::simulation::board::Cell;

#[derive(Component)]
pub struct BoardSphere;

#[derive(Debug, Default, Clone, Reflect, Inspectable)]
#[reflect(Resource)]
pub struct IcoBoard {
    pub adj: Vec<Vec<usize>>,
    pub vertex: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub position: Vec<[f32; 3]>,
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
        self.position[pos.idx()]
    }

    pub fn get_all_adjacent(&self, pos: &BoardPosition) -> Vec<BoardPosition> {
        self.adj[pos.idx()]
            .iter()
            .map(|f| BoardPosition::new(*f))
            .collect()
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

    let centers: Vec<[f32; 3]> = indices
        .chunks_exact(3)
        .map(|idx| {
            let p1 = vertex[idx[0] as usize];
            let p2 = vertex[idx[1] as usize];
            let p3 = vertex[idx[2] as usize];
            let x = (p1[0] + p2[0] + p3[0]) / 3.;
            let y = (p1[1] + p2[1] + p3[1]) / 3.;
            let z = (p1[2] + p2[2] + p3[2]) / 3.;
            [x, y, z]
        })
        .collect();

    let adj = {
        let mut triangle_per_vertex = vec![vec![]; vertex.len()];
        indices.chunks_exact(3).enumerate().for_each(|(i, idx)| {
            triangle_per_vertex[idx[0] as usize].push(i);
            triangle_per_vertex[idx[1] as usize].push(i);
            triangle_per_vertex[idx[2] as usize].push(i);
        });

        let mut adj = vec![vec![]; centers.len()];
        for triangles in triangle_per_vertex {
            for i in 0..triangles.len() {
                for j in 0..triangles.len() {
                    if i != j {
                        adj[triangles[i]].push(triangles[j]);
                    }
                }
            }
        }

        adj.iter_mut().for_each(|v| v.sort_unstable());
        adj.iter_mut().for_each(|v| v.dedup());
        adj
    };

    info!("Board with {} triangles!", centers.len());

    let cells = vec![Cell::default(); centers.len()];

    commands.insert_resource(IcoBoard {
        adj,
        vertex: vertex.clone(),
        indices: indices.clone(),
        position: centers,
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
