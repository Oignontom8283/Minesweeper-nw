use alloc::{str, vec::Vec};

pub enum StateEnum {
    MainMenu,
    Playing,
    GameOver,
}

pub struct SharedState {
    state: StateEnum,
    grid: Vec<u8>,
    width: u8,
    height: u8,
    cursor_x: u8,
    cursor_y: u8,
}

pub enum RenderCommand {
    // jeu
    Cell { x: u8, y: u8 },
    Cursor { x: u8, y: u8 },
}

pub trait StateRuntime {
    fn enter(shared: &mut SharedState);
    fn update(shared: &mut SharedState) -> Vec<RenderCommand>;
    fn exit(shared: &mut SharedState, to_render: Vec<RenderCommand>);
}