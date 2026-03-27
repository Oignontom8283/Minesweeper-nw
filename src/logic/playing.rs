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
    shared.first_click = true;
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

        let mut cells_to_render = Vec::new();

        // Redessiner tout les éléments si besoin
        if _shared.need_redraw {
            _shared.need_redraw = false;

            // Rendre le fond d'écran
            cells_to_render.push(RenderCommand::Background { color: eadkp::COLOR_WHITE });

            // Rerendre tout les cellules
            for y in 0.._shared.height {
                for x in 0.._shared.width {
                    cells_to_render.push(RenderCommand::Cell { x: x, y: y });
                }
            }

            cells_to_render.push(RenderCommand::Cursor { x: _shared.cursor_x, y: _shared.cursor_y });

            return cells_to_render;
        }


        // Générer les entrées clavier
        let just = _keyboard.get_just_pressed(_old_keyboard);

        // Déplacement du curseur :
        let before_cursor_x = _shared.cursor_x;
        let before_cursor_y = _shared.cursor_y;

        let mut after_cursor_x = before_cursor_x;
        let mut after_cursor_y = before_cursor_y;

        if just.key_down(eadkp::input::Key::Up) {
            if before_cursor_y > 0 {
                after_cursor_y -= 1;
            }
        }
        if just.key_down(eadkp::input::Key::Down) {
            if before_cursor_y < _shared.height - 1 {
                after_cursor_y += 1;
            }
        }
        if just.key_down(eadkp::input::Key::Left) {
            if before_cursor_x > 0 {
                after_cursor_x -= 1;
            }
        }
        if just.key_down(eadkp::input::Key::Right) {
            if before_cursor_x < _shared.width - 1 {
                after_cursor_x += 1;
            }
        }

        // Si le curseur a bougé on redessine l'ancienne et la nouvelle position du curseur
        if after_cursor_x != before_cursor_x || after_cursor_y != before_cursor_y {
            
            // Actualiser la position du curseur
            _shared.cursor_x = after_cursor_x;
            _shared.cursor_y = after_cursor_y;

            // Redessiner l'ancienne position du curseur
            cells_to_render.push(RenderCommand::Cell { x: before_cursor_x, y: before_cursor_y });

            // Dessiner le curseur a ça nouvelle position
            cells_to_render.push(RenderCommand::Cursor { x: after_cursor_x, y: after_cursor_y });
        }

        let interact = just.key_down(eadkp::input::Key::Ok);
        let flag = just.key_down(eadkp::input::Key::Back);

        if flag { // On priorise le flag en cas d'appui simultané pour éviter une catastrophe
            
            // Toggle du flag de la cellule
            grid::toggle_flag(_shared, before_cursor_x, before_cursor_y);

            // Redessiner la cellule pour afficher le changement de flag
            cells_to_render.push(RenderCommand::Cell { x: before_cursor_x, y: before_cursor_y });
            cells_to_render.push(RenderCommand::Cursor { x: before_cursor_x, y: before_cursor_y });
        }
        else if interact && !grid::is_flagged(_shared, before_cursor_x, before_cursor_y) {
            // Première interaction : génération des mines
            if _shared.first_click {
                grid::generate_mines(_shared, before_cursor_x, before_cursor_y);
                grid::calculate_adjacent_mines(_shared);
                _shared.first_click = false;
            }
            
            // Révéler les cellules par propagation et les redraw
            let revealed = grid::reveal_infect(_shared, before_cursor_x, before_cursor_y);
            for (rx, ry) in revealed {
                cells_to_render.push(RenderCommand::Cell { x: rx, y: ry });
            }

            // Toujours redessiner le curseur après l'interaction
            cells_to_render.push(RenderCommand::Cursor { x: before_cursor_x, y: before_cursor_y });
        }

        cells_to_render
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        for cmd in _to_render {
            match cmd {
                RenderCommand::Background { color } => {
                    eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, color);
                },
                RenderCommand::Cell { x, y } => {

                    let point = grid::cell_to_coords(_shared, x as u16, y as u16);

                    if grid::is_revealed(_shared, x, y) { // Si la cellule est révélée, on affiche soit une mine soit un nombre

                        if grid::is_mine(_shared, x, y) {
                            grid::render_cell_mine(_shared, point); // Rendre la mine si c'est une mine
                        }
                        else {
                            let nub = grid::get_adjacent_mines(_shared, x, y); 
                            grid::render_cell_number(_shared, point, nub); // Rendre le nombre de mines adjacentes sinon
                        }
                    }
                    else if grid::is_flagged(_shared, x, y) { // Si la cellule n'est pas révélée mais est flaggée, on affiche un drapeau
                        grid::render_cell_flag(_shared, point);
                    }
                    else {
                        grid::render_cell_dirt(_shared, point); // Sinon on affiche de la terre
                    }

                },
                RenderCommand::Cursor { x, y } => {
                    let point = grid::cell_to_coords(_shared, x as u16, y as u16);

                    // Rendre le curseur a la positon de la cellule
                    grid::render_cell_cursor(_shared, point);
                },
                _ => {}
            }
        }
    }
}