use crate::common::*;
use alloc::vec::Vec;
use alloc::vec;

pub fn init_playing(shared: &mut SharedState, width: u8, height: u8, num_mines: usize) {
    
    // Initialiser la grille
    shared.width = width;
    shared.height = height;
    shared.grid = vec![0; (width as usize) * (height as usize)];

    shared.num_mines = num_mines;
    shared.first_click = false;

    shared.cursor_x = 0;
    shared.cursor_y = 0;

    // Changer le state
    shared.state = StateEnum::Playing;

    // Demander un redraw
    shared.need_redraw = true;
}

pub fn cell_to_screen(shared: &mut SharedState, x:u8, y:u8) -> eadkp::Point {
    let cell_size = if shared.large_cells { 16 } else { 8 };
    eadkp::Point {
        x: (x as u16) * cell_size,
        y: (y as u16) * cell_size,
    }
}

pub struct Playing;

impl StateRuntime for Playing {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState, _keyboard: eadkp::input::KeyboardState, _old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand> {
        // 
        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        // 
    }
}