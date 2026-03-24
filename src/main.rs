#![cfg_attr(target_os = "none", no_std)]
#![no_main]

/*!
Simple base code including only the strict necessary using the eadkp library
and displaying "Hello, world!" on the screen.
*/

// Import the eadkp crate with macros
#[macro_use]
extern crate eadkp;

// Setup the NWA environment.
eadk_setup!(name = "Minesweeper");

#[unsafe(no_mangle)]
pub fn main() -> isize {

    // Initialize the heap allocator
    _eadk_init_heap();

    let dirt_texture = eadkp::ImageLoader::from_flash(include_image!("dirt.png")).expect("Failed to load dirt texture");

    // Drawing closure. Called before looping.
    let first_drawing = || {
        eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, eadkp::COLOR_WHITE); // Fill the entire screen with white

        eadkp::display::push_image(&dirt_texture, eadkp::Point {x:5, y:5});
    };

    let drawing = || {
        // Currently empty, but can be used for dynamic drawing in the main loop
    };
    

    // Initial keyboard state
    let mut prev = eadkp::input::KeyboardState::scan();

    first_drawing(); // Initial drawing

    loop {
        let now = eadkp::input::KeyboardState::scan(); // Scan the current keyboard state
        let just = now.get_just_pressed(prev); // Get keys that were just pressed
        if just.key_down(eadkp::input::Key::Back) { break 0; }; // Exit if Back key is pressed

        // Clear the screen to white
        eadkp::display::wait_for_vblank(); // Wait for VBlank before updating the display

        drawing(); // Call the drawing closures

        // Update previous keyboard state
        prev = now;
    }
}