use crate::common::*;
use alloc::vec::Vec;


pub struct GameOver;

impl StateRuntime for GameOver {
    fn enter(_shared: &mut SharedState) {
        //
    }

    fn update(_shared: &mut SharedState) -> Vec<RenderCommand> {
        // 
        Vec::new()
    }

    fn render(_shared: &mut SharedState, _to_render: Vec<RenderCommand>) {
        // 
    }
}