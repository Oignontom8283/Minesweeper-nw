use crate::{common::*, grid, logic::*, menu, render, save};
use alloc::{format, string::ToString, vec::Vec};

pub fn init_playing(shared: &mut SharedState, difficulty: DifficultyEnum, large_cells: bool) {
    
    let (width, height) = size_by_difficulty(difficulty);

    // Initialiser la grille
    shared.width = width;
    shared.height = height;
    
    let size = (width as usize) * (height as usize);
    shared.grid.clear();          // On vide la grille précédente
    shared.grid.resize(size, 0);  // On la remplit de 0 en réutilisant la mémoire allouée
    
    shared.large_cells = large_cells; // Choix de la taille des cellules

    // Calculer la position de départ pour centrer la grille à l'écran
    grid::set_start_pos(shared);

    // calculer le nb de mines en fonction de la densité. +0.5 pour arrondir correctement a l'entier le plus proche
    shared.num_mines = (MINES_DENSITY_HARD*(width*height) as f32 + 0.5) as usize;
    
    shared.remaining_safe_cells = (width as usize) * (height as usize) - shared.num_mines;
    shared.theoretical_remaining_mines = shared.num_mines as i32;
    shared.first_action = true;

    shared.cursor_x = 0;
    shared.cursor_y = 0;

    shared.time_base = 0;
    shared.time_started = eadkp::timing::millis();
    shared.time_to_next_update = shared.time_started; // Déclencher une update imméditatement

    // Changer le state
    shared.state = StateEnum::Playing;

    // Demander un redraw
    shared.need_redraw = true;
}

pub fn resume_playing(shared: &mut SharedState) {
    
    // Charger la save
    save::load_game(shared, SAVE_GAME_FILE_NAME);

    save::delete_game_save(SAVE_GAME_FILE_NAME); // Supprimer la save. save_game() le fait, mais comme ça on vide déja le storage, pas besoin de le faire au end_game

    // Recalculer la position de base pour l'affiche
    grid::set_start_pos(shared);

    shared.time_started = eadkp::timing::millis();
    shared.time_to_next_update = shared.time_started; // Déclencher une update imméditatement

    // Changer le state
    shared.state = StateEnum::Playing;

    // Demander un redraw
    shared.need_redraw = true;
}

pub fn pause_playing(shared: &mut SharedState) {

    // Sauvegarder la partie
    save::save_game(shared, SAVE_GAME_FILE_NAME);

    // Exit game
    shared.running = false;
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
            cells_to_render.push(RenderCommand::Background { color: BACKGROUND_COLOR });
            cells_to_render.push(RenderCommand::Frame { color: FRAME_COLOR }); // Rendre le cadre du jeu

            // Rerendre tout les cellules
            for y in 0.._shared.height {
                for x in 0.._shared.width {
                    cells_to_render.push(RenderCommand::Cell { x: x, y: y });
                }
            }

            cells_to_render.push(RenderCommand::Cursor { x: _shared.cursor_x, y: _shared.cursor_y }); // Rerendre le curseur
            cells_to_render.push(RenderCommand::TitleBackground { color: TITLE_BACKGROUND_COLOR_PLAYING }); // Rendre le title
            cells_to_render.push(RenderCommand::TitleMines { mines: _shared.theoretical_remaining_mines.to_string(), color: TITLE_COLOR, background: TITLE_BACKGROUND_COLOR_PLAYING }); // Rendre le nb de mines

            return cells_to_render;
        }


        // Générer les entrées clavier
        let just = _keyboard.get_just_pressed(_old_keyboard);

        // Quitter le jeu
        if just.key_down(KEY_EXIT) {
            pause_playing(_shared);
            _shared.running = false;
            #[cfg(not(target_os = "none"))]
            println!("Saving game and exiting...");
            return cells_to_render;
        }

        // Retourner au menu / abandonner la partie en cours
        if just.key_down(KEY_MENU) {
            init_main_menu(_shared);
            return cells_to_render;
        }

        // Déplacement du curseur :
        let before_cursor_x = _shared.cursor_x;
        let before_cursor_y = _shared.cursor_y;

        let mut after_cursor_x = before_cursor_x;
        let mut after_cursor_y = before_cursor_y;

        if just.key_down(KEY_UP) {
            if before_cursor_y > 0 {
                after_cursor_y -= 1;
            }
        }
        if just.key_down(KEY_DOWN) {
            if before_cursor_y < _shared.height - 1 {
                after_cursor_y += 1;
            }
        }
        if just.key_down(KEY_LEFT) {
            if before_cursor_x > 0 {
                after_cursor_x -= 1;
            }
        }
        if just.key_down(KEY_RIGHT) {
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

        let interact = just.key_down(KEY_REVEAL);
        let flag = just.key_down(KEY_FLAG);

        if flag { // On priorise le flag en cas d'appui simultané pour éviter une catastrophe
            
            // Toggle du flag de la cellule
            let flag_change = -grid::toggle_flag(_shared, before_cursor_x, before_cursor_y);

            // Actualiser le nombre de mines théorique restante
            _shared.theoretical_remaining_mines += flag_change as i32;

            // Redessiner la cellule pour afficher le changement de flag
            cells_to_render.push(RenderCommand::Cell { x: before_cursor_x, y: before_cursor_y });
            cells_to_render.push(RenderCommand::Cursor { x: before_cursor_x, y: before_cursor_y });
            cells_to_render.push(RenderCommand::TitleMines { mines: format!(" {} ", _shared.theoretical_remaining_mines), color: TITLE_COLOR, background: TITLE_BACKGROUND_COLOR_PLAYING })
        }
        else if interact && !grid::is_flagged(_shared, before_cursor_x, before_cursor_y) {
            // Première interaction : génération des mines
            if _shared.first_action {
                grid::generate_mines(_shared, before_cursor_x, before_cursor_y);
                grid::calculate_adjacent_mines(_shared);
                _shared.first_action = false;
            }

            // Si la cellule est une mine, GAME OVER
            if grid::is_mine(_shared, before_cursor_x, before_cursor_y) {
                
                // Faire afficher toutes les mines
                for y in 0.._shared.height {
                    for x in 0.._shared.width {
                        if grid::is_mine(_shared, x, y) {
                            grid::set_revealed(_shared, x, y);
                            cells_to_render.push(RenderCommand::Cell { x: x, y: y });
                        }
                    }
                }

                // Lancer le state de fin en lose
                end_game::init_end_game(_shared, false);

                return cells_to_render;
            }
            // Si ce n'est pas une mine, on révéle
            else {

                // Révéler les cellules par propagation et les redraw
                let revealed = grid::reveal_infect(_shared, before_cursor_x, before_cursor_y);
                for (rx, ry) in revealed.iter() {
                    cells_to_render.push(RenderCommand::Cell { x: *rx, y: *ry });
                }

                _shared.remaining_safe_cells -= revealed.len();

                if _shared.remaining_safe_cells <= 0 {
                    end_game::init_end_game(_shared, true);
                }
    
                // Toujours redessiner le curseur après l'interaction
                cells_to_render.push(RenderCommand::Cursor { x: before_cursor_x, y: before_cursor_y });
            }
            
        }


        // Gérer le timer
        let now = eadkp::timing::millis(); // temps actuelle
        if now >= _shared.time_to_next_update {
            _shared.time_to_next_update = now + UPDATE_TIME_INTERVAL; // Planifier la prochaine update

            let elapsed_time = _shared.time_base + (now - _shared.time_started); // Calculer le temps écoulé depuis le début de la partie
            let time_str = time_to_string(elapsed_time); // Convertir le temps écoulé en une string formatée

            // Remander le redraw du temps dans le titre
            cells_to_render.push(RenderCommand::TitleTime { time: time_str, color: TITLE_COLOR, background: TITLE_BACKGROUND_COLOR_PLAYING });
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
                            render::render_cell_mine(_shared, point); // Rendre la mine si c'est une mine
                        }
                        else {
                            let nub = grid::get_adjacent_mines(_shared, x, y); 
                            render::render_cell_number(_shared, point, nub); // Rendre le nombre de mines adjacentes sinon
                        }
                    }
                    else if grid::is_flagged(_shared, x, y) { // Si la cellule n'est pas révélée mais est flaggée, on affiche un drapeau
                        render::render_cell_flag(_shared, point);
                    }
                    else {
                        render::render_cell_dirt(_shared, point); // Sinon on affiche de la terre
                    }

                },
                RenderCommand::Cursor { x, y } => {
                    let point = grid::cell_to_coords(_shared, x as u16, y as u16);

                    // Rendre le curseur a la positon de la cellule
                    render::render_cell_cursor(_shared, point);
                },
                RenderCommand::Frame { color } => {

                    // Rendre le cadre du jeu
                    render::render_frame(_shared, color, FRAME_THICKNESS);
                },
                RenderCommand::TitleBackground { color } => {
                    // rendre le background du titre
                    eadkp::display::push_rect_uniform(TITLEBAR_RECT, color);
                },
                RenderCommand::TitleTime { time, color, background } => {
                    menu::draw_texts(&menu::TextLayout {
                        lines: &[menu::TextStyle { text: time.as_str(), color, bg_color: background, is_large:TITLE_FONT_IS_LARGE }],
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Center,
                        spacing: 0
                    }, title_point_pourcent(0.25));
                },
                RenderCommand::TitleMines { mines, color, background } => {
                    menu::draw_texts(&menu::TextLayout {
                        lines: &[menu::TextStyle { text: mines.as_str(), color, bg_color: background, is_large:TITLE_FONT_IS_LARGE }],
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Center,
                        spacing: 0
                    }, title_point_pourcent(0.75));
                },
                _ => {}
            }
        }
    }
}