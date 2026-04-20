
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

pub enum VerticalAlign {
    Top,
    Center,
    Botton,
}

pub struct TextConfig<'a> {
    pub text: &'a str,
    pub pos: eadkp::Point,
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
    pub color: eadkp::Color,
    pub bg_color: eadkp::Color,
    pub font: eadkp::FontSize,
}


pub fn draw_text_configured(config: TextConfig) {

    let font_width = config.font.width;
    let font_height = config.font.height;

    // Calculer la largeur
    let total_width = config.text.len() as u16 * font_width;


    // Calcule du X
    let final_x = match config.h_align {
        HorizontalAlign::Left => config.pos.x,
        HorizontalAlign::Center => config.pos.x - ( total_width / 2),
        HorizontalAlign::Right => config.pos.x - total_width,
    };

    // Calcule du Y
    let final_y = match config.v_align {
        VerticalAlign::Top => config.pos.y,
        VerticalAlign::Center => config.pos.y - (font_height / 2),
        VerticalAlign::Botton => config.pos.y - font_height,
    };

    let is_large = config.font == eadkp::LARGE_FONT;

    
    // Afficher le texte
    eadkp::display::draw_string(
        config.text,
        eadkp::Point { x: final_x, y: final_y },
        is_large,
        config.color,
        config.bg_color
    )
}