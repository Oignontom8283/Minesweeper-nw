use crate::{common::*, logic::*, menu};
use alloc::{vec, vec::Vec, string::ToString};

pub fn init_main_menu(shared: &mut SharedState) {
    shared.need_redraw = true;
    shared.state = StateEnum::MainMenu;
}

pub struct MainMenu;

impl StateRuntime for MainMenu {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState, _new_keyboard: eadkp::input::KeyboardState, _old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand> {
        
        if _shared.need_redraw {
            _shared.need_redraw = false;

            // Si une save existe, la charger
            if eadkp::storage::file_exists(SAVE_GAME_FILE_NAME).unwrap() {

                #[cfg(not(target_os = "none"))]
                println!("Save file found, resuming game...");

                resume_playing(_shared);
            }

            return vec![
                RenderCommand::Background { color: eadkp::COLOR_WHITE },
                RenderCommand::TitleBackground { color: TITLE_BACKGROUND_COLOR_MENU },
                RenderCommand::TitleText { text: TITLE_TEXT_MAIN_MENU.to_string(), color: TITLE_COLOR, background: TITLE_BACKGROUND_COLOR_MENU },
                RenderCommand::Instruction,
            ];
        }

        let just_pressed= _new_keyboard.get_just_pressed(_old_keyboard);


        if just_pressed.key_down(KEY_EXIT) {

            _shared.running = false; // Exit the game if Exe key is pressed
        }

        else if just_pressed.key_down(KEY_NORMALE_MODE) {
            // Starte une nouvelle partie en mode normale

            #[cfg(not(target_os = "none"))]
            println!("Switching to Playing state with large cells");

            let width = 10;
            let height = 10;
            
            init_playing(_shared, width, height, (MINES_DENSITY_NORMALE*(width*height) as f32 + 0.5) as usize, true); // +0.5 pour arrondir correctement a l'entier le plus proche
        }

        else if just_pressed.key_down(KEY_HARD_MODE) {
            // Start une nouvelle partie en mode difficile

            #[cfg(not(target_os = "none"))]
            println!("Switching to Playing state with small cells");

            let width = 17;
            let height = 12;

            init_playing(_shared, width, height, (MINES_DENSITY_HARD*(width*height) as f32 + 0.5) as usize, false);
        }

        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        
        for cmd in _to_render {
            match cmd {
                RenderCommand::Background { color } => {
                    eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, color);
                },
                RenderCommand::Instruction => {

                    let width = eadkp::SCREEN_RECT.width;
                    let height = eadkp::SCREEN_RECT.height;


                    let menu_lines = [
                        menu::TextStyle { text: "Press OK to normal game", color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: true },
                        menu::TextStyle { text: "Press BACK to hard game", color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: true },
                    ];

                    menu::draw_texts(&menu::TextLayout {
                        lines: &menu_lines,
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Center,
                        spacing: 5
                    }, eadkp::Point { x: width / 2, y: height - (height / 2) });
                    

                    let footer_lines = [
                        menu::TextStyle { text: "Press DEL to exit; Don't use HOME !", color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: false },
                        menu::TextStyle { text: "TOOL to back to menu", color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: false },
                    ];

                    menu::draw_texts(&menu::TextLayout {
                        lines: &footer_lines,
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Bottom,
                        spacing: 1
                    }, eadkp::Point { x: width / 2, y: height - 1});
                },
                RenderCommand::TitleBackground { color } => {
                  eadkp::display::push_rect_uniform(TITLEBAR_RECT, color);  
                },
                RenderCommand::TitleText { text, color, background }     => {
                    // Obtenir positon du texte
                    let point = title_text_to_point(&text, TITLE_FONT);

                    // Rendre le texte du titre
                    eadkp::display::draw_string(&text, point, TITLE_FONT_IS_LARGE, color, background);
                },
                _ => {}
            }
        }
    }
}