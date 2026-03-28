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

        if _new_keyboard.get_just_pressed(_old_keyboard).key_down(eadkp::input::Key::Ok) {
            if cfg!(target_os = "none") {
                // Code pour nw
            } else {
                #[cfg(not(target_os = "none"))]
                println!("Switching to Playing state");
            }
            
            init_playing(_shared, 10, 10, 10, true);

            return vec![]; // No need to render anything immediately, the Playing state will handle it
            
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

                    let text = "Press OK to start";

                    let text_width = (text.len() as u16) * eadkp::LARGE_FONT.width;
                    let x = eadkp::SCREEN_RECT.width / 2 - text_width / 2;
                    let y = eadkp::SCREEN_RECT.height / 2 - eadkp::LARGE_FONT.height / 2;

                    eadkp::display::draw_string(
                        text,
                        eadkp::Point { x, y },
                        true,
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