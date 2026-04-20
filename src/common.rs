use alloc::{format, string::String, vec::Vec};
use serde::{Serialize, Deserialize};

pub const MINES_DENSITY_NORMALE: f32 = 0.10; // 10% des cellules contiennent des mines
pub const MINES_DENSITY_HARD: f32 = 0.15; // 15% des cellules contiennent des mines
pub const CELL_SMALL: u16 = 16;
pub const CELL_LARGE: u16 = 20;
pub const CELL_MARGIN: u16 = 1;

pub const CURSOR_COLOR: eadkp::Color = eadkp::COLOR_MAGENTA;
pub const FRAME_COLOR: eadkp::Color = eadkp::Color::from_888(135, 135, 135); // gray
pub const FRAME_THICKNESS: u8 = 1;
pub const UPDATE_TIME_INTERVAL: u64 = 1000; // en milliseconds

pub const BACKGROUND_PLAYING_COLOR: eadkp::Color = eadkp::COLOR_WHITE;

pub const TITLE_FONT_IS_LARGE: bool = true;
pub const TITLE_FONT: eadkp::FontSize = if TITLE_FONT_IS_LARGE { eadkp::LARGE_FONT } else { eadkp::SMALL_FONT };
pub const TITLE_COLOR: eadkp::Color = eadkp::COLOR_WHITE; // coleur du texte du titre
pub const SUBTITLE_ENDGAME_IS_LAGE: bool = false;
pub const SUBTITLE_ENDGAME_MARGIN: u16 = 2;
pub const TITLEBAR_RECT: eadkp::Rect = eadkp::Rect { x: 0, y:0, width: eadkp::SCREEN_RECT.width, height: eadkp::LARGE_FONT.height };
pub const PLAY_AREA_RECT: eadkp::Rect = eadkp::Rect { x: 0, y: TITLEBAR_RECT.height, width: eadkp::SCREEN_RECT.width, height: eadkp::SCREEN_RECT.height - TITLEBAR_RECT.height };

pub const TITLE_BACKGROUND_COLOR_PLAYING: eadkp::Color = eadkp::Color::from_888(255, 181, 49);
pub const TITLE_BACKGROUND_COLOR_MENU: eadkp::Color = eadkp::Color::from_888(255, 181, 49);
pub const TITLE_BACKGROUND_COLOR_ENDGAME_WIN: eadkp::Color = eadkp::Color::from_888(50, 168, 82);
pub const TITLE_BACKGROUND_COLOR_ENDGAME_LOSE: eadkp::Color = eadkp::Color::from_888(255, 104, 74);

pub const TITLE_TEXT_MAIN_MENU: &str = "Minesweeper";
pub const TITLE_TEXT_WIN: &str = "You win !";
pub const TITLE_TEXT_LOSE: &str = "You lose !";

pub const SAVE_GAME_FILE_NAME: &str = "minesweeper_game.sav";
pub const SAVE_SCORE_FILE_NAME: &str = "minesweeper_score.sav";

pub const KEY_UP: eadkp::input::Key = eadkp::input::Key::Up;
pub const KEY_DOWN: eadkp::input::Key = eadkp::input::Key::Down;
pub const KEY_LEFT: eadkp::input::Key = eadkp::input::Key::Left;
pub const KEY_RIGHT: eadkp::input::Key = eadkp::input::Key::Right;
pub const KEY_FLAG: eadkp::input::Key = eadkp::input::Key::Ok;
pub const KEY_REVEAL: eadkp::input::Key = eadkp::input::Key::Back;
pub const KEY_NORMALE_MODE: eadkp::input::Key = eadkp::input::Key::Ok;
pub const KEY_HARD_MODE: eadkp::input::Key = eadkp::input::Key::Back;
pub const KEY_EXIT: eadkp::input::Key = eadkp::input::Key::Backspace;
pub const KEY_MENU: eadkp::input::Key = eadkp::input::Key::Toolbox; // abandon

#[derive(PartialEq)]
pub enum StateEnum {
    MainMenu,
    Playing,
    EndGame,
}

pub struct SharedState {
    pub running: bool,

    pub state: StateEnum,
    pub need_redraw: bool,

    pub grid: Vec<u8>,
    pub width: u8,
    pub height: u8,
    pub start_x: u16,
    pub start_y: u16,
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub first_action: bool,
    pub num_mines: usize,
    pub remaining_safe_cells: usize,
    pub theoretical_remaining_mines: i32,
    pub large_cells: bool,

    pub time_base: u64,
    pub time_started: u64, // Mment de chargement de l'instance, pas du start !
    pub time_stoped: u64,
    pub time_to_next_update: u64,

    pub wined: bool, // true if the player has won, false if they lost (used for end game screen)

    pub asset_dirt_large: eadkp::Image,
    pub asset_dirt_small: eadkp::Image,
    pub asset_flag_large: eadkp::Image,
    pub asset_flag_small: eadkp::Image,
    pub asset_mine_large: eadkp::Image,
    pub asset_mine_small: eadkp::Image,
}

pub enum RenderCommand {

    // Common
    Background { color: eadkp::Color },
    TitleBackground { color: eadkp::Color },
    TitleText { text: String, color: eadkp::Color, background: eadkp::Color },

    // jeu
    Cell { x: u8, y: u8 },
    Cursor { x: u8, y: u8 },
    Frame { color: eadkp::Color },
    TitleTime { time: String, color: eadkp::Color, background: eadkp::Color },
    TitleMines { mines: String, color: eadkp::Color, background: eadkp::Color },

    // menu
    Instruction,

    // end game
    SubTitleText { text: Vec<String>, color: eadkp::Color, background: eadkp::Color },
}

pub trait StateRuntime {
    #[allow(dead_code)]
    fn enter(shared: &mut SharedState);
    fn update(shared: &mut SharedState, new_keyboard: eadkp::input::KeyboardState, old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand>;
    fn render(shared: &mut SharedState, to_render: Vec<RenderCommand>);
}

#[derive(Serialize, Deserialize)]
pub struct GameSave {
    pub grid: Vec<u8>,
    pub width: u8,
    pub height: u8,
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub first_action: bool,
    pub num_mines: usize,
    pub remaining_safe_cells: usize,
    pub theoretical_remaining_mines: i32,
    pub large_cells: bool,
    pub time_base: u64,
}


pub fn title_text_to_point(text: &str, font_size: eadkp::FontSize) -> eadkp::Point {
    let x = TITLEBAR_RECT.width / 2 - ((text.len() * font_size.width as usize) / 2) as u16;
    let y = TITLEBAR_RECT.height / 2 - font_size.height / 2;

    eadkp::Point { x, y }
}

pub fn title_text_to_point_pourcent(text: &str, font_size: eadkp::FontSize, pourcent: f32) -> eadkp::Point {
    let x = (TITLEBAR_RECT.width as f32 * pourcent) as u16 - ((text.len() * font_size.width as usize) / 2) as u16;
    let y = TITLEBAR_RECT.height / 2 - font_size.height / 2;

    eadkp::Point { x, y }
}

pub fn time_to_string(time: u64) -> String {
    let seconds = (time / 1000) % 60;
    let minutes  =(time / (60 * 1000)) % 60;
    let hours = time / (60 *60 *1000);

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}