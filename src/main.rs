#![cfg_attr(target_os = "none", no_std)]
#![no_main]

/*!
Simple base code including only the strict necessary using the eadkp library
and displaying "Hello, world!" on the screen.
*/

// Import the eadkp crate with macros
#[macro_use]
extern crate eadkp;

mod common;
mod logic;
pub mod grid;

use alloc::vec;
use common::*;
use logic::*;

// Setup the NWA environment.
eadk_setup!(name = "Minesweeper");

#[unsafe(no_mangle)]
pub fn main() -> isize {

    // Initialize the heap allocator
    _eadk_init_heap();

    // Initialize the shared state

    let mut shared = SharedState {
        state: StateEnum::MainMenu,
        grid: vec![0; 0], // Empty grid
        width: 0,
        height: 0,
        cursor_x: 0,
        cursor_y: 0,
        need_redraw: true,
        first_click: false,
        num_mines: 0,
        large_cells: false,

        asset_dirt: eadkp::ImageLoader::from_flash(eadkp::include_image!("dirt.png")).expect("Failed to load dirt image"),
    };

    let mut prev = eadkp::input::KeyboardState::scan(); // Initial keyboard state

    loop {
        let now = eadkp::input::KeyboardState::scan(); // Scan the current keyboard state
        let just = now.get_just_pressed(prev); // Get keys that were just pressed
        if just.key_down(eadkp::input::Key::Back) { break 0; }; // Exit if Back key is pressed

        // Clear the screen to white
        eadkp::display::wait_for_vblank(); // Wait for VBlank before updating the display

        match shared.state {
            StateEnum::MainMenu => {
                let cmds = MainMenu::update(&mut shared, now, prev);
                MainMenu::render(&mut shared, cmds);
            },
            StateEnum::Playing => {
                let cmds = Playing::update(&mut shared, now, prev);
                Playing::render(&mut shared, cmds);
            },
            StateEnum::GameOver => {
                let cmds = GameOver::update(&mut shared, now, prev);
                GameOver::render(&mut shared, cmds);
            },
        };

        // Update previous keyboard state
        prev = now;
    }
}