use crate::common::*;
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

            return if _shared.wined { // Player won
                vec![
                    RenderCommand::TitleBackground { color: TITLE_COLOR_ENDGAME_WIN },
                    RenderCommand::TitleText { text: TITLE_TEXT_WIN.to_string(), color: eadkp::COLOR_WHITE },
                ]
            } else { // Player lost
                vec![
                    RenderCommand::TitleBackground { color: TITLE_COLOR_ENDGAME_LOSE },
                    RenderCommand::TitleText { text: TITLE_TEXT_LOSE.to_string(), color: eadkp::COLOR_WHITE },
                ]
            };
        };

        // Générer les entrées clavier

        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        // 
    }
}