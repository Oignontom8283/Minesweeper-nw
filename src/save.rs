use crate::{common::*};


pub fn save_game(shared: &SharedState) {

    if shared.state != StateEnum::Playing { panic!("Trying to save game while not in playing state !"); } // Garde fou

    let save = GameSave {
        grid: shared.grid.clone(),
        width: shared.width,
        height: shared.height,
        cursor_x: shared.cursor_x,
        cursor_y: shared.cursor_y,
        first_action: shared.first_action,
        num_mines: shared.num_mines,
        remaining_safe_cells: shared.remaining_safe_cells,
        theoretical_remaining_mines: shared.theoretical_remaining_mines,
        large_cells: shared.large_cells,

        time_base: shared.time_base + (eadkp::timing::millis() - shared.time_started), // temps écoule depuis l'init et temps de base cumulé
    };

    // Sérialiser le GameSave en utilisant postcard
    let serialized = postcard::to_allocvec(&save).unwrap_or_else(|e| {
        panic!("Failed to serialize game save: {:?}", e)
    });

    // Si le fichier de save existe déja, le supprimer
    if eadkp::storage::file_exists(SAVE_GAME_FILE_NAME).unwrap() {
        unsafe { eadkp::storage::file_erase(SAVE_GAME_FILE_NAME).unwrap() };
    } 
    
    // Écrire les données sérialisées le fichier de save
    eadkp::storage::file_write_raw(SAVE_GAME_FILE_NAME, &serialized).unwrap_or_else(|e| {
        panic!("Failed to save game: {:?}", e)
    });

}