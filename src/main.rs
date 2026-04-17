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
mod grid;
mod render;

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
        running: true, // Le jeu court ? Hmm... interesting.

        state: StateEnum::MainMenu,
        grid: vec![0; 0], // Empty grid
        width: 0,
        height: 0,
        start_x: 0,
        start_y: 0,
        cursor_x: 0,
        cursor_y: 0,
        need_redraw: true,
        first_action: false,
        num_mines: 0,
        remaining_safe_cells: 0,
        theoretical_remaining_mines: 0,
        large_cells: true,

        time_base: 0,
        time_started: 0,
        time_stoped: 0,
        time_to_next_update: 0,

        wined: false,

        asset_dirt_large: eadkp::ImageLoader::from_flash(eadkp::include_image!("dirt_large.png")).expect("Failed to load dirt large image"),
        asset_dirt_small: eadkp::ImageLoader::from_flash(eadkp::include_image!("dirt_small.png")).expect("Failed to load dirt small image"),

        asset_flag_large: eadkp::ImageLoader::from_flash(eadkp::include_image!("flag_large.png")).expect("Failed to load dirt large image"),
        asset_flag_small: eadkp::ImageLoader::from_flash(eadkp::include_image!("flag_small.png")).expect("Failed to load dirt small image"),

        asset_mine_large: eadkp::ImageLoader::from_flash(eadkp::include_image!("mine_large.png")).expect("Failed to load mine large image"),
        asset_mine_small: eadkp::ImageLoader::from_flash(eadkp::include_image!("mine_small.png")).expect("Failed to load mine small image"),
    };

    let mut prev = eadkp::input::KeyboardState::scan(); // Initial keyboard state

    while shared.running {
        let now = eadkp::input::KeyboardState::scan(); // Scan the current keyboard state
        let just = now.get_just_pressed(prev); // Get keys that were just pressed
        if just.key_down(eadkp::input::Key::Home) { shared.running = false; }; // Exit if Home key is pressed

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
            StateEnum::EndGame => {
                let cmds = EndGame::update(&mut shared, now, prev);
                EndGame::render(&mut shared, cmds);
            },
        };

        // Update previous keyboard state
        prev = now;
    }

    return 0;
}