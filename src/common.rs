use alloc::{vec::Vec};

pub const CELL_SMALL: u16 = 16;
pub const CELL_LARGE: u16 = 20;

pub const CELL_MARGIN: u16 = 1;

pub enum StateEnum {
    MainMenu,
    Playing,
    GameOver,
}

pub struct SharedState {
    pub state: StateEnum,
    pub need_redraw: bool,

    pub grid: Vec<u8>,
    pub width: u8,
    pub height: u8,
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub first_click: bool,
    pub num_mines: usize,
    pub large_cells: bool,

    pub asset_dirt_large: eadkp::Image,
    pub asset_dirt_small: eadkp::Image,
}

pub enum RenderCommand {

    // Common
    Background { color: eadkp::Color },

    // jeu
    Cell { x: u8, y: u8 },
    Cursor { x: u8, y: u8 },

    // menu
    Instruction
}

pub trait StateRuntime {
    fn enter(shared: &mut SharedState);
    fn update(shared: &mut SharedState, new_keyboard: eadkp::input::KeyboardState, old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand>;
    fn render(shared: &mut SharedState, to_render: Vec<RenderCommand>);
}