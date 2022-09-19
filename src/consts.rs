// Window setup
pub const WINDOW_SIZE: f32 = 800.;
pub const STARTING_UPS: f64 = 1. / 60.;

// Board
pub const BOARD_WIDTH: usize = (WINDOW_SIZE / CELL_SIZE) as usize;
pub const BOARD_HEIGHT: usize = (WINDOW_SIZE / CELL_SIZE) as usize;

// Cell
pub const CELL_BORDER: f32 = 1.;
pub const CELL_SIZE: f32 = 400.;
pub const CELL_PAINT: f32 = CELL_SIZE - 2. * CELL_BORDER;

// Ant
pub const ANT_COUNT: usize = 4;
