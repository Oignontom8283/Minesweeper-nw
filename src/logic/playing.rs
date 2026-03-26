use crate::common::*;
use alloc::vec::Vec;


pub struct Playing;

impl StateRuntime for Playing {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState, _keyboard: eadkp::input::KeyboardState) -> Vec<RenderCommand> {
        // 
        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        // 
    }
}