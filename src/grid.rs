
use crate::common::*;
use alloc::string::ToString;
use alloc::vec::Vec;

pub fn index_by_coords(x: u8, y: u8, width: u8) -> usize {
    (y as usize) * (width as usize) + (x as usize)
}


pub fn set_mine(shared: &mut SharedState, x: u8, y: u8) {
    let index = index_by_coords(x, y, shared.width);
    shared.grid[index] |= 0b0000_0001; // Placer une mine (premier bit à 1)
}

pub fn is_mine(shared: &mut SharedState, x: u8, y: u8) -> bool {
    let index = index_by_coords(x, y, shared.width);
    (shared.grid[index] & 0b0000_0001) != 0 // Vérifier si le premier bit est à 1
}

pub fn set_revealed(shared: &mut SharedState, x: u8, y: u8) {
    let index = index_by_coords(x, y, shared.width);
    shared.grid[index] |= 0b0000_0010; // Marquer comme révélé (deuxième bit à 1)
}

pub fn is_revealed(shared: &mut SharedState, x: u8, y: u8) -> bool {
    let index = index_by_coords(x, y, shared.width);
    (shared.grid[index] & 0b0000_0010) != 0 // Vérifier si le deuxième bit est à 1
}

pub fn set_flagged(shared: &mut SharedState, x: u8, y: u8) {
    let index = index_by_coords(x, y, shared.width);
    shared.grid[index] |= 0b0000_0100; // Marquer comme flaggé (troisième bit à 1)
}

pub fn toggle_flag(shared: &mut SharedState, x: u8, y: u8) {
    let index = index_by_coords(x, y, shared.width);
    shared.grid[index] ^= 0b0000_0100; // Inverser l'état du flag (troisième bit)
}

pub fn is_flagged(shared: &mut SharedState, x: u8, y: u8) -> bool {
    let index = index_by_coords(x, y, shared.width);
    (shared.grid[index] & 0b0000_0100) != 0 // Vérifier si le troisième bit est à 1
}

pub fn set_adjacent_mines(shared: &mut SharedState, x: u8, y: u8, count: u8) {
    let index = index_by_coords(x, y, shared.width);
    shared.grid[index] &= 0b0000_1111; // Nettoyer les anciens bits au cas où
    shared.grid[index] |= (count << 4) & 0b1111_0000; // Stocker le nombre de mines adjacentes dans les 4 bits supérieurs
}

pub fn get_adjacent_mines(shared: &mut SharedState, x: u8, y: u8) -> u8 {
    let index = index_by_coords(x, y, shared.width);
    (shared.grid[index] & 0b1111_0000) >> 4 // Récupérer le nombre de mines adjacentes
}


pub fn generate_mines(shared: &mut SharedState, first_x: u8, first_y: u8) {
    let mut coords: Vec<(u8, u8)> = Vec::new();

    // Générer des coordonnées uniques pour les mines
    for _ in 0..shared.num_mines {

        let mut unique = false;
        let mut random_x = 0;
        let mut random_y = 0;

        // Générer des coordonnées jusqu'à ce qu'elles soient uniques
        while !unique {
            random_x = eadkp::random::randint(0, shared.width as u32) as u8; // x aléatoire
            random_y = eadkp::random::randint(0, shared.height as u32) as u8; // y aléatoire

            // Assurer que les coordonnées sont uniques et ne sont pas la première case révélée
            unique = !coords.contains(&(random_x, random_y)) && !(random_x == first_x && random_y == first_y);
        }

        coords.push((random_x, random_y));
    }

    // Placer les mines dans la grille
    for (x, y) in coords {
        set_mine(shared, x, y);
    }
}

pub fn calculate_adjacent_mines(shared: &mut SharedState) {
    for y in 0..shared.height {
        for x in 0..shared.width {

            if is_mine(shared, x, y) {
                continue; // Sauter les mines
            }

            let mut adjacent_mines: u8 = 0;

            // Vérifier les 8 cases entourant la case actuelle
            for dy in -1i8..=1 {
                for dx in -1i8..=1 {

                    if dx == 0 && dy == 0 {
                        continue; // Sauter la case actuelle
                    }

                    // Calculer les coordonnées de la case voisine
                    let nx = x as i8 + dx;
                    let ny = y as i8 + dy;

                    // Vérifier que les coordonnées sont dans les limites de la grille
                    if nx >= 0 && nx < shared.width as i8 && ny >= 0 && ny < shared.height as i8 {
                        // Si c'est une mine, +1
                        if is_mine(shared, nx as u8, ny as u8) {
                            adjacent_mines += 1;
                        }
                    }

                }
            }
            set_adjacent_mines(shared, x, y, adjacent_mines);
        }
    }
}


pub fn cell_size(shared: &mut SharedState) -> u16 {
    if shared.large_cells {
        CELL_LARGE + CELL_MARGIN
    } else {
        CELL_SMALL + CELL_MARGIN
    }
}

pub fn cell_to_coords(shared: &mut SharedState, x: u16, y: u16) -> eadkp::Point {
    let cell_size = cell_size(shared);

    let p_x = x as u16 * cell_size;
    let p_y = y as u16 * cell_size;
    
    let screen_pos = eadkp::Point {
        x: p_x,
        y: p_y,
    };

    screen_pos
}


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

    let cell_size = cell_size(shared);

    let background_rect = eadkp::Rect {
        x: point.x,
        y: point.y,
        width: cell_size,
        height: cell_size,
    };

    // Rendre le background de la cellule
    eadkp::display::push_rect_uniform(background_rect, BACKGROUND_PLAYING_COLOR);

    // Si >0 afficher le nombre
    if number > 0 {
        let text = number.to_string();

        let text_x = point.x + (cell_size / 2) - (eadkp::LARGE_FONT.width / 2);
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
            BACKGROUND_PLAYING_COLOR
        );
    }

}


pub fn render_cell_cursor(shared: &mut SharedState, point: eadkp::Point) {
    let cell_size = cell_size(shared);

    // Rendre la bordure du haut
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: point.x, y: point.y, width: cell_size - 1, height: 1},
        CURSOR_COLOR
    );

    // Rendre la bordure du bas
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: point.x, y: point.y + cell_size - 2, width: cell_size - 2, height: 1},
        CURSOR_COLOR
    );

    // Rendre la bordure de gauche
    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: point.x, y: point.y + 1, width: 1, height: cell_size - 2 },
        CURSOR_COLOR
    );

    // Rendre la bordure de droite
    eadkp::display::push_rect_uniform(
        eadkp::Rect{ x: point.x + cell_size - 2, y: point.y + 1, width: 1, height: cell_size - 2 },
        CURSOR_COLOR
    );
}