use alloc::{vec::Vec};

pub enum StateEnum {
    MainMenu,
    Playing,
    GameOver,
}

pub struct SharedState {
    pub state: StateEnum,
    pub grid: Vec<u8>,
    pub width: u8,
    pub height: u8,
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub need_redraw: bool,
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
    fn update(shared: &mut SharedState, keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand>;
    fn render(shared: &mut SharedState, to_render: Vec<RenderCommand>);
}