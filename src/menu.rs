
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
    pub large_font: bool,
}


pub fn draw_text_configured(config: TextConfig) {

    let font = if config.large_font { eadkp::LARGE_FONT } else { eadkp::SMALL_FONT };


    // Calculer la largeur
    let total_width = config.text.len() as u16 * font.width;

    // Calcule du X
    let final_x = match config.h_align {
        HorizontalAlign::Left => config.pos.x,
        HorizontalAlign::Center => config.pos.x - ( total_width / 2),
        HorizontalAlign::Right => config.pos.x - total_width,
    };

    // Calcule du Y
    let final_y = match config.v_align {
        VerticalAlign::Top => config.pos.y,
        VerticalAlign::Center => config.pos.y - (font.height / 2),
        VerticalAlign::Botton => config.pos.y - font.height,
    };

    
    // Afficher le texte
    eadkp::display::draw_string(
        config.text,
        eadkp::Point { x: final_x, y: final_y },
        config.large_font,
        config.color,
        config.bg_color
    )
}