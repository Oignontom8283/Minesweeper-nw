#[allow(unused_imports)]
use crate::{common::*, logic::*};
use alloc::{format, vec, vec::Vec, string::ToString};

pub fn init_end_game(shared: &mut SharedState, wined: bool) {
    shared.wined = wined;
    shared.need_redraw = true;
    shared.time_stoped = eadkp::timing::millis(); // moment de fin
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
                RenderCommand::TitleText { text: text.to_string(), color: TITLE_COLOR, background },
                RenderCommand::SubTitleText { text: vec![
                    format!("Time: {}", time_to_string((_shared.time_stoped - _shared.time_started) + _shared.time_base)),
                    "Press OK or BACK to return to menu".to_string(),
                ], color: TITLE_COLOR, background },
            ];
        };

        // Générer les entrées clavier
        let just = _keyboard.get_just_pressed(_old_keyboard);
        
        if just.key_down(eadkp::input::Key::Ok) || just.key_down(eadkp::input::Key::Back) {
            init_main_menu(_shared);
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
                RenderCommand::SubTitleText { text, color, background} => {
                    let font = if SUBTITLE_ENDGAME_IS_LAGE { eadkp::LARGE_FONT } else { eadkp::SMALL_FONT };
                    let x_base = eadkp::SCREEN_RECT.width / 2;

                    for (i, line) in text.iter().enumerate() {
                        
                        let x = x_base - (line.len() as u16 * font.width) / 2;
                        let y = eadkp::SCREEN_RECT.height - ((i as u16 + 1) * (font.height + SUBTITLE_ENDGAME_MARGIN));

                        let point = eadkp::Point { x, y };

                        eadkp::display::draw_string(line, point, SUBTITLE_ENDGAME_IS_LAGE, color, background);
                    }
                }
                _ => {},
            }
        }
    }
}