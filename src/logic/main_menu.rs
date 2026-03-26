use crate::common::*;
use alloc::vec::Vec;
use alloc::vec;

pub struct MainMenu;

impl StateRuntime for MainMenu {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState, _keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand> {
        
        if _shared.need_redraw {
            _shared.need_redraw = false;
            return vec![RenderCommand::Instruction];
        }

        if _keyboard.key_down(eadkp::input::Key::Ok) {
            
            if cfg!(target_os = "none") {
                // Code pour nw
            } else {
                #[cfg(not(target_os = "none"))]
                println!("Switching to Playing state");
            }
        }

        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        
        for cmd in _to_render {
            match cmd {
                RenderCommand::Instruction => {

                    let text = "Press OK to start";

                    let x =eadkp::SCREEN_RECT.width / 2 - (text.len() as u16) / 2;
                    let y = eadkp::SCREEN_RECT.height / 2 - eadkp::LARGE_FONT.height / 2;

                    eadkp::display::draw_string(
                        text,
                        eadkp::Point { x, y },
                        true,
                        eadkp::COLOR_BLACK,
                        eadkp::COLOR_WHITE
                    );
                    
                },
                _ => {}
            }
        }
    }
}