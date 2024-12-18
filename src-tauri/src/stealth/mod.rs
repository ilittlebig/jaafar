use user_agent_parser::UserAgentParser;

mod plugins;
mod user_agent;
mod speech_synthesis;
mod desktop_capabilities;
mod audio_fingerprint;
mod hardware_concurrency_and_memory;
mod screen_resolution;
mod canvas_fingerprint;

use plugins::generate_hide_plugins_script;
use user_agent::generate_user_agent_data_script;
use speech_synthesis::generate_speech_synthesis_script;
use desktop_capabilities::generate_desktop_capabilities_script;
use audio_fingerprint::generate_audio_fingerprint_script;
use hardware_concurrency_and_memory::generate_hardware_concurrency_and_memory_script;
use screen_resolution::generate_screen_resolution_script;
use canvas_fingerprint::generate_canvas_fingerprint_script;

use crate::data::profiles::BrowserProfile;

/// Generates a JavaScript snippet to spoof browser properties.
///
/// # Arguments
/// - `browser_profile`: Contains browser details to spoof.
///
/// # Returns
/// - A string with the JavaScript stealth script.
pub fn build_stealth_script(browser_profile: &BrowserProfile) -> String {
    let regexes_path = include_str!("../data/regexes.yaml");
    let ua_parser = UserAgentParser::from_str(regexes_path).unwrap();
    let product = ua_parser.parse_product(browser_profile.ua);

    let browser_name: &str = product.name.as_deref().unwrap_or("Unknown");
    let browser_version: &str = product.major.as_deref().unwrap_or("0");

    let platform = browser_profile.platform;
    let webgl_vendor = browser_profile.webgl_vendor;
    let webgl_renderer = browser_profile.webgl_renderer;

    let plugins_script = generate_hide_plugins_script();
    let user_agent_data_script = generate_user_agent_data_script(browser_profile, browser_name, browser_version);
    let speech_synthesis_script = generate_speech_synthesis_script(browser_name);
    let desktop_capabilities_script = generate_desktop_capabilities_script();
    let audio_fingerprint_script = generate_audio_fingerprint_script();
    let hardware_concurrency_and_memory_script = generate_hardware_concurrency_and_memory_script(browser_name);
    let screen_resolution_script = generate_screen_resolution_script();
    let canvas_fingerprint_script = generate_canvas_fingerprint_script();

    let main_script = format!(r#"
        Object.defineProperty(navigator, 'platform', {{ get: () => '{platform}' }});
        Object.defineProperty(navigator, 'vendor', {{ get: () => '{webgl_vendor}' }});
        Object.defineProperty(navigator, 'languages', {{ get: () => ['en-US', 'en'] }});

        Object.defineProperty(navigator, 'webdriver', {{
            get: () => undefined,
        }});

        const getParameter = WebGLRenderingContext.prototype.getParameter;
        WebGLRenderingContext.prototype.getParameter = function(parameter) {{
            if (parameter === 37445) return '{webgl_vendor}';
            if (parameter === 37446) return '{webgl_renderer}';
            return getParameter.call(this, parameter);
        }};

        const originalDefineProperty = Object.defineProperty;
        Object.defineProperty = function(target, prop, descriptor) {{
            if (target instanceof Error && prop === 'stack' && typeof descriptor.get === 'function') {{
                return originalDefineProperty(target, prop, {{
                    get: function() {{
                        return Error.prototype.stack.call(this);
                    }},
                    configurable: true,
                    enumerable: false
                }});
            }}
            return originalDefineProperty(target, prop, descriptor);
        }};

        {plugins_script}
        {user_agent_data_script}
        {speech_synthesis_script}
        {desktop_capabilities_script}
        {audio_fingerprint_script}
        {hardware_concurrency_and_memory_script}
        {screen_resolution_script}
        {canvas_fingerprint_script}
    "#);

    let worker_script = format!(r#"
        const originalWorker = Worker;
        Worker = function(script, options) {{
            const stealthScript = `{main_script}`;
            const blob = new Blob([
                stealthScript + '\n' + 'importScripts("' + script + '")'
            ], {{ type: 'application/javascript' }});
            const blobURL = URL.createObjectURL(blob);
            return new originalWorker(blobURL, options);
        }};
    "#);

    let stealth_script = format!("{main_script}{worker_script}");
    stealth_script
}
