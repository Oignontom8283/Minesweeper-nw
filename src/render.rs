
use alloc::{string::ToString};
use crate::{common::*, grid};


pub fn render_cell_dirt(shared: &mut SharedState, point: eadkp::Point) {
    let cell_image = if shared.large_cells { &shared.asset_dirt_large } else { &shared.asset_dirt_small };

    // Rendre le background de la cellule
    eadkp::display::push_image(cell_image, point);
}

pub fn render_cell_flag(shared: &mut SharedState, point: eadkp::Point) {
    let cell_image = if shared.large_cells { &shared.asset_flag_large } else { &shared.asset_flag_small };

    // Rendre le flag de la cellule
    eadkp::display::push_image(cell_image, point);
}

pub fn render_cell_number(shared: &mut SharedState, point: eadkp::Point, number: u8) {
    // On ne "peint" de background blanc QUE sur la partie correspondant à la cellule elle-même, 
    // pas sur la marge, sinon on écrase le cadre si on est sur la bordure !
    let cell_visual_size = if shared.large_cells { CELL_LARGE } else { CELL_SMALL };

    let background_rect = eadkp::Rect {
        x: point.x,
        y: point.y,
        width: cell_visual_size,
        height: cell_visual_size,
    };

    // Rendre le background de la cellule
    eadkp::display::push_rect_uniform(background_rect, BACKGROUND_COLOR);

    // Si >0 afficher le nombre
    if number > 0 {
        let text = number.to_string();

        let text_x = point.x + (cell_visual_size / 2) - (eadkp::LARGE_FONT.width / 2);
        let text_y = point.y + 1;

        let text_point = eadkp::Point {
            x: text_x,
            y: text_y,
        };

        eadkp::display::draw_string(
            &text, 
            text_point, 
            shared.large_cells, 
            eadkp::COLOR_BLACK, 
            BACKGROUND_COLOR
        );
    }
}

pub fn render_cell_mine(shared: &mut SharedState, point: eadkp::Point) {
    let cell_image = if shared.large_cells { &shared.asset_mine_large } else { &shared.asset_mine_small };

    // Rendre la mine de la cellule
    eadkp::display::push_image(cell_image, point);
}

pub fn render_cell_cursor(shared: &mut SharedState, point: eadkp::Point) {
    // Le curseur doit uniquement entourer la cellule elle-même et JAMAIS sa marge
    // Sinon le curseur dessinera par-dessus l'espace entre les cases
    let cell_visual_size = if shared.large_cells { CELL_LARGE } else { CELL_SMALL };

    // Rendre la bordure du haut
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: point.x, y: point.y, width: cell_visual_size, height: 1},
        CURSOR_COLOR
    );

    // Rendre la bordure du bas
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: point.x, y: point.y + cell_visual_size - 1, width: cell_visual_size, height: 1},
        CURSOR_COLOR
    );

    // Rendre la bordure de gauche
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: point.x, y: point.y + 1, width: 1, height: cell_visual_size - 2 },
        CURSOR_COLOR
    );

    // Rendre la bordure de droite
    eadkp::display::push_rect_uniform(
        eadkp::Rect{ x: point.x + cell_visual_size - 1, y: point.y + 1, width: 1, height: cell_visual_size - 1},
        CURSOR_COLOR
    );
}

pub fn render_frame(shared: &mut SharedState, color: eadkp::Color, thickness: u8) {
    let total_width = (shared.width as u16) * grid::cell_size(shared) + CELL_MARGIN;
    let total_height = (shared.height as u16) * grid::cell_size(shared) + CELL_MARGIN;

    // Les bords du cadre intérieur commencent juste avant la première marge
    let frame_x = shared.start_x - CELL_MARGIN;
    let frame_y = shared.start_y - CELL_MARGIN;
    let t = thickness as u16;

    // On englobe complètement la zone (la barre englobe frame_x jusqu'à frame_x + total_width)

    // Barre du haut
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: frame_x - t, y: frame_y - t, width: total_width + 2 * t, height: t },
        color,
    );

    // Barre du bas
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: frame_x - t, y: frame_y + total_height, width: total_width + 2 * t, height: t },
        color,
    );

    // Barre de gauche
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: frame_x - t, y: frame_y, width: t, height: total_height },
        color,
    );

    // Barre de droite
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: frame_x + total_width, y: frame_y, width: t, height: total_height },
        color,
    );
}