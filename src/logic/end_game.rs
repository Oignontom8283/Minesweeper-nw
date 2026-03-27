use crate::common::*;
use alloc::{vec::Vec, vec, string::ToString};

const TITLE_TEXT_COLOR: eadkp::Color = eadkp::COLOR_WHITE;

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
                (TITLE_TEXT_WIN, TITLE_COLOR_ENDGAME_WIN)
            } else {
                (TITLE_TEXT_LOSE, TITLE_COLOR_ENDGAME_LOSE)
            };

            return vec![
                RenderCommand::TitleBackground { color: background },
                RenderCommand::TitleText { text: text.to_string(), color: TITLE_TEXT_COLOR, background }
            ];
        };

        // Générer les entrées clavier

        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        // 
    }
}