use user_agent_parser::UserAgentParser;

use crate::utils::plugins::generate_hide_plugins_script;
use crate::data::profiles::BrowserProfile;

/// Generates a JavaScript stealth script to spoof browser properties.
///
/// # Arguments
/// - `browser_profile`: Contains browser details to spoof.
///
/// # Returns
/// - A string with the JavaScript stealth script.
pub fn build_stealth_script(browser_profile: &BrowserProfile) -> String {
    let plugins_script = generate_hide_plugins_script();
    let user_agent_data_script = generate_user_agent_data_string(browser_profile);

    let platform = browser_profile.platform;
    let webgl_vendor = browser_profile.webgl_vendor;
    let webgl_renderer = browser_profile.webgl_renderer;
    let hardware_concurrency = browser_profile.hardware_concurrency;

    let main_script = format!(r#"
        Object.defineProperty(navigator, 'platform', {{ get: () => '{platform}' }});
        Object.defineProperty(navigator, 'vendor', {{ get: () => '{webgl_vendor}' }});
        Object.defineProperty(navigator, 'hardwareConcurrency', {{ get: () => {hardware_concurrency} }});
        Object.defineProperty(navigator, 'languages', {{ get: () => ['en-US', 'en'] }});

        Object.defineProperty(navigator, 'webdriver', {{
            get: () => undefined,
        }});

        {user_agent_data_script}

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
    "#);

    let worker_script = format!(r#"
        const originalWorker = Worker;
        Worker = function(script, options) {{
            const stealthScript = `{main_script}`;
            const blob = new Blob([stealthScript + '\n' + 'importScripts("' + script + '")'], {{ type: 'application/javascript' }});
            const blobURL = URL.createObjectURL(blob);
            return new originalWorker(blobURL, options);
        }};
    "#);

    let stealth_script = format!("{main_script}{worker_script}");
    stealth_script
}

fn generate_user_agent_data_string(browser_profile: &BrowserProfile) -> String {
    let regexes_path = include_str!("../data/regexes.yaml");
    let ua_parser = UserAgentParser::from_str(regexes_path).unwrap();
    let product = ua_parser.parse_product(browser_profile.ua);

    let browser_name: &str = product.name.as_deref().unwrap_or("Unknown");
    let browser_version: &str = product.major.as_deref().unwrap_or("0");

    let architecture = browser_profile.architecture;
    let bitness = browser_profile.bitness;
    let platform_version = browser_profile.platform_version;

    let is_chromium_based = false;//matches!(browser_name, "Chrome" | "Google Chrome" | "Microsoft Edge" | "Chromium");
    let brands = if is_chromium_based {
        generate_brands(browser_name, browser_version)
    } else {
        "undefined".to_string()
    };

    let platform = if is_chromium_based {
        format!("{}", browser_profile.ua_data_platform)
    } else {
        "undefined".to_string()
    };

    let is_mobile = if is_chromium_based {
        "false".to_string()
    } else {
        "undefined".to_string()
    };

    let high_entropy_values = if is_chromium_based {
        r#"
            architecture: "{architecture}",
            bitness: "{bitness}",
            model: "",
            platformVersion: "{platform_version}"
        "#.to_string()
    } else {
        r#"
            architecture: "NA",
            bitness: "NA",
            model: "NA",
            platformVersion: "NA"
        "#.to_string()
    };

    format!(r#"
        Object.defineProperty(navigator, 'userAgentData', {{
            get: () => ({{
                brands: {brands},
                platform: {platform},
                mobile: {is_mobile},
                getHighEntropyValues: keys => {{
                    return new Promise(resolve => {{
                        const highEntropyValues = {{
                            {high_entropy_values}
                        }};

                        const result = keys.reduce((obj, key) => {{
                            if (highEntropyValues[key] !== undefined) {{
                                obj[key] = highEntropyValues[key];
                            }}
                            return obj;
                        }}, {{}});
                        resolve(result);
                    }});
                }}
            }})
        }});
    "#)
}

fn generate_brands(browser_name: &str, browser_version: &str) -> String {
    match browser_name {
        "Chrome" | "Google Chrome" => format!(
            r#"
            [
                {{ brand: "Google Chrome", version: "{browser_version}" }},
                {{ brand: "Chromium", version: "{browser_version}" }},
                {{ brand: "Not_A Brand", version: "24" }}
            ]
            "#
        ),
        "Microsoft Edge" => format!(
            r#"
            [
                {{ brand: "Microsoft Edge", version: "{browser_version}" }},
                {{ brand: "Chromium", version: "{browser_version}" }},
                {{ brand: "Not_A Brand", version: "24" }}
            ]
            "#
        ),
        _ => panic!("Unsupported browser for brands generation: {}", browser_name),
    }
}