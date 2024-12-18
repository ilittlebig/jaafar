use rand::{Rng, thread_rng};
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::distributions::Alphanumeric;

use crate::data::plugins::PLUGINS_SET;

/// Generates a JavaScript snippet to spoof the browser's plugin information.
///
/// # Returns
/// A JavaScript string that sets `navigator.plugins` to 2 real and 2 random plugin filenames.
pub fn generate_hide_plugins_script() -> String {
    let plugins = get_random_plugins();
    let formatted_plugins = plugins
        .iter()
        .map(|plugin| format!("{{ filename: '{}' }}", plugin))
        .collect::<Vec<String>>()
        .join(", ");

    let mime_types = plugins
        .iter()
        .map(|plugin| format!("{{ type: 'application/x-{}' }}", plugin))
        .collect::<Vec<String>>()
        .join(", ");

    let plugin_script = format!(r#"
        Object.defineProperty(navigator, 'plugins', {{
            get: () => [{formatted_plugins}]
        }});

        Object.defineProperty(navigator, 'mimeTypes', {{
            get: () => [{mime_types}]
        }});
    "#);
    plugin_script
}

fn generate_random_plugin_filename() -> String {
    let mut rng = thread_rng();
    let random_string: String = (0..10)
        .map(|_| rng.sample(Alphanumeric))
        .map(char::from)
        .collect();
    format!("{}.plugin", random_string)
}

/// Generates 2 real plugins, and 2 completely random plugins
fn get_random_plugins() -> Vec<String> {
    let mut plugins: Vec<String> = PLUGINS_SET
        .iter()
        .choose_multiple(&mut thread_rng(), 2)
        .into_iter()
        .map(|p| p.to_string())
        .collect();

    for _ in 0..2 {
        let random_plugin_name = generate_random_plugin_filename();
        plugins.push(random_plugin_name);
    }

    plugins.shuffle(&mut thread_rng());
    plugins
}
