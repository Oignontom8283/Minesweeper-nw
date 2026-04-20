
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


