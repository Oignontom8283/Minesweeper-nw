use alloc::{borrow::Cow};

fn get_font_size(is_large: bool) -> eadkp::FontSize {
    if is_large { eadkp::LARGE_FONT } else { eadkp::SMALL_FONT }
}

#[allow(dead_code)]
pub enum HorizontalAlign { Left, Center, Right }
#[allow(dead_code)]
pub enum VerticalAlign { Top, Center, Bottom }

pub struct TextStyle<'a> {
    pub text: Cow<'a, str>,
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


pub struct TextLayout<'a> {
    pub lines: &'a [TextStyle<'a>],
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
    pub spacing: u16,
}

impl<'a> TextLayout<'a> {
    pub fn total_height(&self) -> u16 {
        if self.lines.is_empty() { return 0; }

        let h: u16 = self.lines.iter().map(|l| l.height()).sum();
        let s = (self.lines.len() as u16 - 1) * self.spacing;
        h + s
    }

    pub fn get_start_y(&self, anchor_y: u16) -> u16 {
        let total = self.total_height() as i32;
        let ay = anchor_y as i32;
        
        let y =match self.v_align {
            VerticalAlign::Top => total,
            VerticalAlign::Center => ay - (total / 2),
            VerticalAlign::Bottom => ay - total,
        };

        y.max(0) as u16
    }

    pub fn get_line_x(&self, line:&TextStyle, anchor_x: u16) -> u16 {
        let lw = line.width() as i32;
        let ax = anchor_x as i32;

        let x = match self.h_align {
            HorizontalAlign::Left => ax,
            HorizontalAlign::Center => ax - (lw / 2),
            HorizontalAlign::Right => ax - lw,
        };

        x.max(0) as u16
    }
}



pub fn draw_texts(layout: &TextLayout, anchor: eadkp::Point) {
    let mut current_y = layout.get_start_y(anchor.y);

    for line in layout.lines {
        let x = layout.get_line_x(line, anchor.x);

        eadkp::display::draw_string(
            &line.text, 
            eadkp::Point { x, y: current_y}, 
            line.is_large, 
            line.color, 
            line.bg_color
        );
        
        current_y += line.height() + layout.spacing;
    }
}