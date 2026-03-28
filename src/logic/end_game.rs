#[allow(unused_imports)]
use crate::{common::*, logic::*};
use alloc::{vec::Vec, vec, string::ToString};

pub fn init_end_game(shared: &mut SharedState, wined: bool) {
    shared.wined = wined;
    shared.need_redraw = true;
    shared.state = StateEnum::EndGame
}

pub struct EndGame;

impl StateRuntime for EndGame {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState, _keyboard: eadkp::input::KeyboardState, _old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand> {
        
        // Rerendre que le titre pour laisser le joueur voir ça partie
        if _shared.need_redraw {
            _shared.need_redraw = false;

            let (text, background) = if _shared.wined {
                (TITLE_TEXT_WIN, TITLE_BACKGROUND_COLOR_ENDGAME_WIN)
            } else {
                (TITLE_TEXT_LOSE, TITLE_BACKGROUND_COLOR_ENDGAME_LOSE)
            };

            return vec![
                RenderCommand::TitleBackground { color: background },
                RenderCommand::TitleText { text: text.to_string(), color: TITLE_COLOR, background }
            ];
        };

        // Générer les entrées clavier
        let just = _keyboard.get_just_pressed(_old_keyboard);
        
        if just.key_down(eadkp::input::Key::Ok) || just.key_down(eadkp::input::Key::Back) {
            _shared.state = StateEnum::MainMenu;
            _shared.need_redraw = true;
            return vec![];
        }

        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        for cmd in _to_render {
            match cmd {
                RenderCommand::TitleBackground { color } => {
                    eadkp::display::push_rect_uniform(TITLEBAR_RECT, color);
                }
                RenderCommand::TitleText { text, color, background } => {
                    let point = title_text_to_point(&text, TITLE_FONT);
                    
                    eadkp::display::draw_string(&text, point, TITLE_FONT_IS_LARGE, color, background);
                },
                _ => {},
            }
        }
    }
}