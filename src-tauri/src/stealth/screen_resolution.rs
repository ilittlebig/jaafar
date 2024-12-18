use rand::thread_rng;
use rand::prelude::SliceRandom;

/// Generates a JavaScript snippet to spoof screen resolution values.
///
/// # Returns
/// A String with JavaScript snippet to spoof screen resolution values.
pub fn generate_screen_resolution_script() -> String {
    let mut rng = thread_rng();
    let screen_sizes = [
        (1366, 768),  // Standard Laptop
        (1280, 800),  // Smaller Laptop
        (1440, 900),  // 16:10 Laptop
        (1600, 900),  // Wide HD
        (1920, 1080), // Full HD
        (1920, 1080),
        (1920, 1080),
        (1920, 1080),
        (1920, 1080),
        (1920, 1080),
        (2560, 1440), // QHD
        (3840, 2160), // 4K UHD
    ];

    let (width, height) = *screen_sizes.choose(&mut rng).unwrap();
    let avail_width = width;
    let avail_height = height;

    format!(r#"
        Object.defineProperty(screen, 'width', {{
            get: () => {width}
        }});
        Object.defineProperty(screen, 'height', {{
            get: () => {height}
        }});
        Object.defineProperty(screen, 'availWidth', {{
            get: () => {avail_width}
        }});
        Object.defineProperty(screen, 'availHeight', {{
            get: () => {avail_height}
        }});
    "#)
}
