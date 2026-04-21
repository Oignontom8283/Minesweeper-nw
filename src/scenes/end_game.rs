use crate::{menu, save::save_score};
#[allow(unused_imports)]
use crate::{common::*, scenes::*};
use alloc::{format, vec, vec::Vec, string::ToString};

pub fn init_end_game(shared: &mut SharedState, wined: bool) {
    shared.wined = wined;
    shared.need_redraw = true;
    shared.time_stoped = eadkp::timing::millis(); // moment de fin
    shared.time_elapsed = (shared.time_stoped - shared.time_started) + shared.time_base; // temps écoulé total

    calculate_score(shared); // calculer et appliquer le score
    save_score(SAVE_SCORE_FILE_NAME, &shared.score); // sauvegarder le score

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
                    format!("Time: {}", time_to_string(_shared.time_elapsed)),
                    "Press OK or BACK to return to menu".to_string(),
                ], color: TITLE_COLOR, background },
            ];
        };

        // Générer les entrées clavier
        let just = _keyboard.get_just_pressed(_old_keyboard);
        
        if just.key_down(KEY_REVEAL) || just.key_down(KEY_FLAG) || just.key_down(KEY_MENU) {
            init_main_menu(_shared);
        }

        // exit
        if just.key_down(KEY_EXIT) {
            _shared.running = false;
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
                    menu::draw_texts(&menu::TextLayout {
                        lines: &[menu::TextStyle { text: text.as_str().into(), color, bg_color: background, is_large:TITLE_FONT_IS_LARGE }],
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Center,
                        spacing: 0
                    }, title_point());
                },
                RenderCommand::SubTitleText { text, color, background} => {
                    let lines: Vec<menu::TextStyle> = text.iter().map(|t| {
                        menu::TextStyle { text: t.as_str().into(), color, bg_color: background, is_large: SUBTITLE_ENDGAME_IS_LAGE }
                    }).collect();

                    menu::draw_texts(&menu::TextLayout {
                        lines: &lines,
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Bottom,
                        spacing: SUBTITLE_ENDGAME_MARGIN
                    }, eadkp::Point { x: eadkp::SCREEN_RECT.width / 2, y: eadkp::SCREEN_RECT.height - SUBTITLE_ENDGAME_MARGIN });
                }
                _ => {},
            }
        }
    }
}