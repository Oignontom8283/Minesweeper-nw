use alloc::{vec::Vec};
use alloc::vec;

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

    pub asset_dirt: eadkp::Image,
}

pub enum RenderCommand {

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