use crate::{common::*, logic::*};
use alloc::{vec, vec::Vec, string::ToString};

pub struct MainMenu;

impl StateRuntime for MainMenu {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState, _new_keyboard: eadkp::input::KeyboardState, _old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand> {
        
        if _shared.need_redraw {
            _shared.need_redraw = false;
            return vec![
                RenderCommand::Background { color: eadkp::COLOR_WHITE },
                RenderCommand::TitleBackground { color: TITLE_BACKGROUND_COLOR_MENU },
                RenderCommand::TitleText { text: TITLE_TEXT_MAIN_MENU.to_string(), color: TITLE_COLOR, background: TITLE_BACKGROUND_COLOR_MENU },
                RenderCommand::Instruction,
            ];
        }

        let just_pressed= _new_keyboard.get_just_pressed(_old_keyboard);


        if just_pressed.key_down(eadkp::input::Key::Home) {

            _shared.running = false; // Exit the game if Exe key is pressed
        }

        else if just_pressed.key_down(eadkp::input::Key::Ok) {

            #[cfg(not(target_os = "none"))]
            println!("Switching to Playing state with large cells");

            let width = 10;
            let height = 10;
            
            init_playing(_shared, width, height, (MINES_DENSITY_NORMALE*(width*height) as f32 + 0.5) as usize, true); // +0.5 pour arrondir correctement a l'entier le plus proche
        }

        else if just_pressed.key_down(eadkp::input::Key::Back) {

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

                    let is_large = true;
                    let font = if is_large { eadkp::LARGE_FONT } else { eadkp::SMALL_FONT };

                    let base_x = eadkp::SCREEN_RECT.width / 2;
                    let base_y = eadkp::SCREEN_RECT.height / 2 - font.height / 2;
                    
                    
                    let text_large = "Press OK to start";

                    let text_large_width = (text_large.len() as u16) * font.width;
                    let x_large = base_x - text_large_width / 2;
                    let y_large = base_y - 12;


                    let text_small = ",BACK to start a large grid";

                    let text_small_width = (text_small.len() as u16) * font.width;
                    let x_small = base_x - text_small_width / 2;
                    let y_small = base_y  + 15;


                    eadkp::display::draw_string(
                        text_large,
                        eadkp::Point { x: x_large, y: y_large },
                        is_large,
                        eadkp::COLOR_BLACK,
                        eadkp::COLOR_WHITE
                    );

                    eadkp::display::draw_string(
                        text_small,
                        eadkp::Point { x: x_small, y: y_small },
                        is_large,
                        eadkp::COLOR_BLACK,
                        eadkp::COLOR_WHITE
                    );
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