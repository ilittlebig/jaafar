use rand::thread_rng;
use rand::prelude::SliceRandom;

use crate::utils::browser::is_chromium_based;

pub fn generate_hardware_concurrency_and_memory_script(browser_name: &str) -> String {
    let is_chromium = is_chromium_based(browser_name);

    let mut rng = thread_rng();
    let hardware_concurrency = *[4, 4, 6, 6, 6, 8, 8, 10, 12]
        .choose(&mut rng)
        .expect("No hardware concurrency values available");
    let device_memory = *[2, 4, 8, 8]
        .choose(&mut rng)
        .expect("No device memory values available");

    let device_memory = value_or_undefined!(is_chromium, device_memory);
    format!(r#"
        Object.defineProperty(navigator, 'hardwareConcurrency', {{
            get: () => {hardware_concurrency}
        }});
        Object.defineProperty(navigator, 'deviceMemory', {{
            get: () => {device_memory}
        }});
    "#)
}
