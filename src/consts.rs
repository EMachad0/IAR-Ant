// Window setup
pub const WINDOW_SIZE: f32 = 800.;
pub const STARTING_UPS: f64 = 1. / 10.;

// Board
pub const BOARD_WIDTH: usize = (WINDOW_SIZE / CELL_SIZE) as usize;
pub const BOARD_HEIGHT: usize = (WINDOW_SIZE / CELL_SIZE) as usize;

// Cell
pub const CELL_BORDER: f32 = 1.;
pub const CELL_SIZE: f32 = 10.;
pub const CELL_PAINT: f32 = CELL_SIZE - 2. * CELL_BORDER;

// Ant
pub const ANT_COUNT: usize = 100;
pub const VIEW_RADIUS: i32 = 3;

// Food
pub const FOOD_COUNT: usize = 1000;