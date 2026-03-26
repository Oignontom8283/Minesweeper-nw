
use crate::common::*;
use alloc::vec::Vec;
use alloc::vec;

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

pub fn is_flagged(shared: &mut SharedState, x: u8, y: u8) -> bool {
    let index = index_by_coords(x, y, shared.width);
    (shared.grid[index] & 0b0000_0100) != 0 // Vérifier si le troisième bit est à 1
}

pub fn set_adjacent_mines(shared: &mut SharedState, x: u8, y: u8, count: u8) {
    let index = index_by_coords(x, y, shared.width);
    shared.grid[index] |= (count << 3) & 0b1111_0000; // Stocker le nombre de mines adjacentes dans les 4 bits supérieurs
    // Pourais stocker dans les 3 bits supérieurs car 2^3 = 8, mais devrais fait +1 chaque fois donc flm
}

pub fn get_adjacent_mines(shared: &mut SharedState, x: u8, y: u8) -> u8 {
    let index = index_by_coords(x, y, shared.width);
    (shared.grid[index] & 0b1111_0000) >> 3 // Récupérer le nombre de mines adjacentes
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


