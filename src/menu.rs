
fn get_font_size(is_large: bool) -> eadkp::FontSize {
    return if is_large { eadkp::LARGE_FONT } else { eadkp::SMALL_FONT };
}

pub enum HorizontalAlign { Left, Center, Right }
pub enum VerticalAlign { Top, Center, Bottom }

pub struct TextStyle<'a> {
    pub text: &'a str,
    pub color: eadkp::Color,
    pub bg_color: eadkp::Color,
    pub is_large: bool,
}

impl<'a> TextStyle<'a> {
    pub fn width(&self) -> u16 {
        self.text.len() as u16 * get_font_size(self.is_large).width
    }

    pub fn height(&self) -> u16 {
        get_font_size(self.is_large).height
    }
}

