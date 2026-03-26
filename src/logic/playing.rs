use crate::common::*;
use crate::grid;
use alloc::vec::Vec;
use alloc::vec;

pub fn init_playing(shared: &mut SharedState, width: u8, height: u8, num_mines: usize, large_cells: bool) {
    
    // Initialiser la grille
    shared.width = width;
    shared.height = height;
    shared.grid = vec![0; (width as usize) * (height as usize)];

    shared.num_mines = num_mines;
    shared.first_click = false;
    shared.large_cells = large_cells;

    shared.cursor_x = 0;
    shared.cursor_y = 0;

    // Changer le state
    shared.state = StateEnum::Playing;

    // Demander un redraw
    shared.need_redraw = true;
}

pub struct Playing;

impl StateRuntime for Playing {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState, _keyboard: eadkp::input::KeyboardState, _old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand> {
        
        if _shared.need_redraw {
            _shared.need_redraw = false;

            let mut cells_to_render = Vec::new();

            // Rendre le fond d'écran
            cells_to_render.push(RenderCommand::Background { color: eadkp::COLOR_WHITE });

            // Rerendre tout les cellules
            for y in 0.._shared.height {
                for x in 0.._shared.width {
                    cells_to_render.push(RenderCommand::Cell { x: x, y: y });
                }
            }
            if cfg!(target_os = "none") {
                // Code pour nw
            } else {
                #[cfg(not(target_os = "none"))]
                println!("Redrawing");
            }
            return cells_to_render;
        }

        if _shared.first_click {
            // Générer les mines en s'assurant que la première case cliquée n'est pas une mine
            grid::generate_mines(_shared, _shared.cursor_x, _shared.cursor_y); // Exemple avec 10 mines
            grid::calculate_adjacent_mines(_shared);

            // Marquer que le premier clic a été effectué
            _shared.first_click = false;
        }

        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        for cmd in _to_render {
            match cmd {
                RenderCommand::Background { color } => {
                    eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, color);
                },
                RenderCommand::Cell { x, y } => {

                    let point = grid::cell_to_coords(_shared, x as u16, y as u16);

                    if grid::is_revealed(_shared, x, y) {
                        let n = grid::get_adjacent_mines(_shared, x, y);
                        grid::render_cell_number(_shared, point, n);
                    }
                    else if grid::is_flagged(_shared, x, y) {
                        grid::render_cell_flag(_shared, point);
                    }
                    else {
                        grid::render_cell_dirt(_shared, point);
                    }

                }
                _ => {}
            }
        }
    }
}