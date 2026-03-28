
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
    let mut mines_placed = 0;

    // Générer des coordonnées uniques pour les mines sans passer par un vecteur temporaire
    while mines_placed < shared.num_mines {
        let random_x = eadkp::random::randint(0, shared.width as u32) as u8;
        let random_y = eadkp::random::randint(0, shared.height as u32) as u8;

        // Vérifier qu'on n'est pas sur la première case et que la case ne contient pas déjà une mine
        if !(random_x == first_x && random_y == first_y) && !is_mine(shared, random_x, random_y) {
            set_mine(shared, random_x, random_y);
            mines_placed += 1;
        }
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

pub fn reveal_infect(shared: &mut SharedState, x_start: u8, y_start: u8) -> Vec<(u8, u8)> {
    let mut stack = Vec::new();
    let mut revealed = Vec::new();

    // S'assurer qu'on ne traite pas une case déjà gérée
    if is_revealed(shared, x_start, y_start) || is_flagged(shared, x_start, y_start) {
        return revealed;
    }

    // On marque et push le pixel de départ
    set_revealed(shared, x_start, y_start);
    revealed.push((x_start, y_start));
    stack.push((x_start, y_start));

    while let Some((x, y)) = stack.pop() {
        // stop si c'est une mine (théoriquement impossible sur le premier clic grâce à generate_mines)
        if is_mine(shared, x, y) {
            continue;
        }

        // stop si la case est adjacente à une mine (bordure de la zone vide)
        if get_adjacent_mines(shared, x, y) > 0 {
            continue;
        }

        // Propager la révélation aux voisins (8 directions)
        for ny in y.saturating_sub(1)..=(y + 1).min(shared.height - 1) {
            for nx in x.saturating_sub(1)..=(x + 1).min(shared.width - 1) {
                if nx == x && ny == y {
                    continue;
                }
                
                // Si la case n'a pas encore été traitée
                if !is_revealed(shared, nx, ny) && !is_flagged(shared, nx, ny) {
                    set_revealed(shared, nx, ny); // Marquer pour éviter de l'ajouter en double dans le stack
                    revealed.push((nx, ny));
                    stack.push((nx, ny));
                }
            }
        }
    }

    revealed
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

    let p_x = x as u16 * cell_size + shared.start_x;
    let p_y = y as u16 * cell_size + shared.start_y;

    let screen_pos = eadkp::Point {
        x: p_x,
        y: p_y,
    };

    screen_pos
}

pub fn set_start_pos(shared: &mut SharedState) {
    let cell_size = cell_size(shared);

    // Taille totale avec toutes les bordures : nb_cellules * (taille_cellule + marge) + 1 marge initiale
    let total_width = (shared.width as u16 * cell_size) + CELL_MARGIN;
    let total_height = (shared.height as u16 * cell_size) + CELL_MARGIN;

    // Centrage global du bloc
    let full_start_x = eadkp::SCREEN_RECT.width / 2 - total_width / 2;
    let full_start_y = TITLEBAR_RECT.height + PLAY_AREA_RECT.height / 2 - total_height / 2;

    // start_x et start_y sont les coordonnées de dessin de la PREMIÈRE case.
    // Il faut donc décaler d'une marge car draw commence après la bordure initiale.
    shared.start_x = full_start_x + CELL_MARGIN;
    shared.start_y = full_start_y + CELL_MARGIN;
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
    eadkp::display::push_rect_uniform(background_rect, BACKGROUND_PLAYING_COLOR);

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
            BACKGROUND_PLAYING_COLOR
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