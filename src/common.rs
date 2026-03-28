use alloc::{vec::Vec, string::String};

pub const CELL_SMALL: u16 = 16;
pub const CELL_LARGE: u16 = 20;
pub const CELL_MARGIN: u16 = 1;

pub const CURSOR_COLOR: eadkp::Color = eadkp::COLOR_MAGENTA;
pub const FRAME_COLOR: eadkp::Color = eadkp::Color::from_888(135, 135, 135); // gray
pub const FRAME_THICKNESS: u8 = 1;

pub const BACKGROUND_PLAYING_COLOR: eadkp::Color = eadkp::COLOR_WHITE;

pub const TITLE_FONT_IS_LARGE: bool = true;
pub const TITLE_FONT: eadkp::FontSize = if TITLE_FONT_IS_LARGE { eadkp::LARGE_FONT } else { eadkp::SMALL_FONT };
pub const TITLE_COLOR: eadkp::Color = eadkp::COLOR_WHITE;
pub const TITLEBAR_RECT: eadkp::Rect = eadkp::Rect { x: 0, y:0, width: eadkp::SCREEN_RECT.width, height: eadkp::LARGE_FONT.height };
pub const PLAY_AREA_RECT: eadkp::Rect = eadkp::Rect { x: 0, y: TITLEBAR_RECT.height, width: eadkp::SCREEN_RECT.width, height: eadkp::SCREEN_RECT.height - TITLEBAR_RECT.height };

pub const TITLE_BACKGROUND_COLOR_PLAYING: eadkp::Color = eadkp::Color::from_888(255, 181, 49);
pub const TITLE_BACKGROUND_COLOR_MENU: eadkp::Color = eadkp::Color::from_888(255, 181, 49);
pub const TITLE_BACKGROUND_COLOR_ENDGAME_WIN: eadkp::Color = eadkp::Color::from_888(50, 168, 82);
pub const TITLE_BACKGROUND_COLOR_ENDGAME_LOSE: eadkp::Color = eadkp::Color::from_888(255, 104, 74);

pub const TITLE_TEXT_MAIN_MENU: &str = "Minesweeper";
pub const TITLE_TEXT_WIN: &str = "You win !";
pub const TITLE_TEXT_LOSE: &str = "You lose !";

pub enum StateEnum {
    MainMenu,
    Playing,
    EndGame,
}

pub struct SharedState {
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
    pub large_cells: bool,

    pub time_base: u64,
    pub time_started: u64,
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

    // menu
    Instruction

    // end game

}

pub trait StateRuntime {
    #[allow(dead_code)]
    fn enter(shared: &mut SharedState);
    fn update(shared: &mut SharedState, new_keyboard: eadkp::input::KeyboardState, old_keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand>;
    fn render(shared: &mut SharedState, to_render: Vec<RenderCommand>);
}



pub fn title_text_to_point(text: &str, font_size: eadkp::FontSize) -> eadkp::Point {
    let x = TITLEBAR_RECT.width / 2 - ((text.len() * font_size.width as usize) / 2) as u16;
    let y = TITLEBAR_RECT.height / 2 - font_size.height / 2;

    eadkp::Point { x, y }
}