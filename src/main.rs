#![cfg_attr(target_os = "none", no_std)]
#![no_main]

/*!
Simple base code including only the strict necessary using the eadkp library
and displaying "Hello, world!" on the screen.
*/

use eadkp::ImageFormat;

// Import the eadkp crate with macros
#[macro_use]
extern crate eadkp;

// Setup the NWA environment.
eadk_setup!(name = "Eadkp template");

#[unsafe(no_mangle)]
pub fn main() -> isize {

    // Initialize the heap allocator
    _eadk_init_heap();

    // Load image
    // IMPORTANT: When you add an image, use `just clean` before recompiling so that the Rust compiler takes the new image into account.
    // ::from_flash - Create an image object from flash without copying it to RAM (slow reading, RAM savings)
    // ::to_ram - Create an image object by copying data to RAM (fast reading, high RAM consumption)
    let eadkp_logo = eadkp::ImageLoader::from_flash(eadkp::include_image!("eadkp_logo_64.png")).expect("Failed to load eadkp logo image");

    // Drawing closure. Called before looping.
    let first_drawing = || {
        eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, eadkp::COLOR_WHITE); // Fill the entire screen with white


        // DISPLAY EADKP LOGO

        let height_center = (eadkp::SCREEN_RECT.height as f32 / 2.9) as u16;

        let image_position = eadkp::Point {
            x: eadkp::SCREEN_RECT.width / 2 - eadkp_logo.width() / 2,
            y: height_center - eadkp_logo.height() / 2,
        };

        // Draw the loaded image at position (50, 50)
        eadkp::display::push_image(&eadkp_logo, image_position);

        // DISPLAY TITLE TEXT
        
        let title_text = "Eadkp Template";
        let title_text_len = (title_text.len() * (eadkp::LARGE_FONT.width as usize)) as u16;

        let title_position = eadkp::Point {
            x: eadkp::SCREEN_RECT.width / 2 - (title_text_len / 2),
            y: height_center + (eadkp_logo.height() / 2) + 20 - (eadkp::LARGE_FONT.height / 2),
        };

        eadkp::display::draw_string(
            title_text,
            title_position,
            true,
            eadkp::COLOR_BLACK,
            eadkp::COLOR_WHITE
        );


        // DISPLAY SUBTITLE TEXT

        let subtitle_text = "A simple template to start a new project.";
        let subtitle_text_len = (subtitle_text.len() * eadkp::SMALL_FONT.width as usize) as u16;

        let subtitle_position = eadkp::Point {
            x: eadkp::SCREEN_RECT.width / 2 - (subtitle_text_len / 2),
            y: height_center + (eadkp_logo.height() / 2) + 40 - (eadkp::SMALL_FONT.height / 2),
        };
        
        eadkp::display::draw_string(
            subtitle_text,
            subtitle_position,
            false,
            eadkp::COLOR_BLACK,
            eadkp::COLOR_WHITE
        );


        // DISPLAY FOOTER TEXT

        let footer_text = "Press BACK to exit.";
        let footer_text_padding = 5;

        let footer_position = eadkp::Point {
            x: footer_text_padding,
            y: eadkp::SCREEN_RECT.height - eadkp::SMALL_FONT.height - footer_text_padding,
        };

        eadkp::display::draw_string(
            footer_text,
            footer_position,
            false,
            eadkp::Color::from_888(128, 128, 128), // Gray color
            eadkp::COLOR_WHITE
        );
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