use crate::{common::*, scenes::*, menu};
use alloc::{format, string::ToString, vec::Vec, vec};

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
                RenderCommand::Background { color: BACKGROUND_COLOR },
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
            
            init_playing(_shared, &DifficultyEnum::Normale, true); // +0.5 pour arrondir correctement a l'entier le plus proche
        }

        else if just_pressed.key_down(KEY_HARD_MODE) {
            // Start une nouvelle partie en mode difficile

            #[cfg(not(target_os = "none"))]
            println!("Switching to Playing state with small cells");

            init_playing(_shared, &DifficultyEnum::Hard, false);
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
                        menu::TextStyle { text: "Press OK to normal game".into(), color: eadkp::COLOR_WHITE, bg_color:eadkp::COLOR_BLACK , is_large: true },
                        menu::TextStyle { text: "Press BACK to hard game".into(), color: eadkp::COLOR_WHITE, bg_color:eadkp::COLOR_BLACK, is_large: true },
                    ];

                    menu::draw_texts(&menu::TextLayout {
                        lines: &menu_lines,
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Center,
                        spacing: 5
                    }, eadkp::Point { x: width / 2, y: height - (height / 3) });
                    

                    let footer_lines = [
                        menu::TextStyle { text: "Press DEL to exit; Don't use HOME !".into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: false },
                        menu::TextStyle { text: "TOOL to back to menu (abort)".into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: false },
                    ];

                    menu::draw_texts(&menu::TextLayout {
                        lines: &footer_lines,
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Bottom,
                        spacing: 1
                    }, eadkp::Point { x: width / 2, y: height - 1});

                    // Scores

                    let is_large:bool = false;
                    let marge_h = 2;
                    let marge_v = 1;
                    let spacing = 0;

                    let score_normal = [
                        menu::TextStyle { text: "Normal:".into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: !is_large },
                        menu::TextStyle { text: format!("Played: {}", _shared.score.normal.games_played).into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large },
                        menu::TextStyle { text: format!("Wins: {}", _shared.score.normal.wins).into(), color: eadkp::COLOR_BLACK, bg_color: TITLE_BACKGROUND_COLOR_ENDGAME_WIN, is_large },
                        menu::TextStyle { text: format!("Losses: {}", _shared.score.normal.losses).into(), color: eadkp::COLOR_BLACK, bg_color: TITLE_BACKGROUND_COLOR_ENDGAME_LOSE, is_large },
                        menu::TextStyle { text: format!("Mean Playtime: {}", time_to_string(_shared.score.normal.avg_playtime())).into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large },
                        menu::TextStyle { text: format!("Mean Win: {:.2}%", _shared.score.normal.win_rate() * 100.0).into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large },
                    ];

                    menu::draw_texts(&menu::TextLayout {
                        lines: &score_normal,
                        h_align: menu::HorizontalAlign::Left,
                        v_align: menu::VerticalAlign::Top,
                        spacing,
                    }, eadkp::Point { x: marge_h, y: eadkp::LARGE_FONT.height + marge_v});


                    let score_hard = [
                        menu::TextStyle { text: "Hard:".into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large: !is_large },
                        menu::TextStyle { text: format!("Played: {}", _shared.score.hard.games_played).into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large },
                        menu::TextStyle { text: format!("Wins: {}", _shared.score.hard.wins).into(), color: eadkp::COLOR_BLACK, bg_color: TITLE_BACKGROUND_COLOR_ENDGAME_WIN, is_large },
                        menu::TextStyle { text: format!("Losses: {}", _shared.score.hard.losses).into(), color: eadkp::COLOR_BLACK, bg_color: TITLE_BACKGROUND_COLOR_ENDGAME_LOSE, is_large },
                        menu::TextStyle { text: format!("Mean Playtime: {}", time_to_string(_shared.score.hard.avg_playtime())).into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large },
                        menu::TextStyle { text: format!("Mean Win: {:.2}%", _shared.score.hard.win_rate() * 100.0).into(), color: eadkp::COLOR_BLACK, bg_color: BACKGROUND_COLOR, is_large },
                    ];

                    menu::draw_texts(&menu::TextLayout {
                        lines: &score_hard,
                        h_align: menu::HorizontalAlign::Right,
                        v_align: menu::VerticalAlign::Top,
                        spacing,
                    }, eadkp::Point { x: eadkp::SCREEN_RECT.width - marge_h, y: eadkp::LARGE_FONT.height + marge_v});
                },
                RenderCommand::TitleBackground { color } => {
                  eadkp::display::push_rect_uniform(TITLEBAR_RECT, color);  
                },
                RenderCommand::TitleText { text, color, background } => {
                    menu::draw_texts(&menu::TextLayout {
                        lines: &[menu::TextStyle { text:text.as_str().into(), color: color, bg_color: background, is_large: TITLE_FONT_IS_LARGE }],
                        h_align: menu::HorizontalAlign::Center,
                        v_align: menu::VerticalAlign::Center,
                        spacing: 0
                    }, title_point());

                },
                _ => {}
            }
        }
    }
}