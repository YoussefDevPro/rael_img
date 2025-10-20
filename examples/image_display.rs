use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rael::{Canvas, Color};
use rael_img::load_image;
use std::io::{stdout, Write};
use std::time::Duration;

// A simple struct to ensure cleanup is performed when it's dropped.
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        _ = execute!(stdout(), LeaveAlternateScreen, Show);
        _ = disable_raw_mode();
    }
}

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let (width, height) = crossterm::terminal::size()?;
    let mut canvas = Canvas::new(
        width as usize,
        height as usize,
        Color { r: 0, g: 0, b: 0 }, // Default background color
    );

    // --- Load image data ---
    // IMPORTANT: Replace "path/to/your/image.webp" with the actual path to your image.
    let image_pixels = match load_image("hehe.png", None, None, (10, 5), false, 0.1) {
        Ok(pixels) => pixels,
        Err(e) => {
            // Handle error, e.g., print a message and exit
            eprintln!("Error loading image: {}", e);
            return Ok(());
        }
    };

    // --- Main Game Loop ---
    loop {
        // --- Input Handling ---
        if poll(Duration::from_millis(0))? {
            if let Ok(Event::Key(key_event)) = read() {
                if key_event.code == KeyCode::Char('q') {
                    break; // Exit loop on 'q'
                }
            }
        }

        // --- Update and Draw Scene ---
        canvas.clear(); // Clear canvas to default color

        // Draw the image
        for (x, y, color) in &image_pixels {
            canvas.set_pixel(*x as usize, *y as usize, 1, *color);
        }

        // Render the canvas to the terminal
        let output = canvas.render();
        execute!(stdout, Print(output))?;
        stdout.flush()?;

        std::thread::sleep(Duration::from_millis(100)); // Aim for ~60 FPS
    }

    Ok(())
}
