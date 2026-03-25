use crate::common::*;
use alloc::vec::Vec;


pub struct MainMenu;

impl StateRuntime for MainMenu {
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