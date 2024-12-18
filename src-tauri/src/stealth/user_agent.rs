use crate::utils::browser::is_chromium_based;
use crate::data::profiles::BrowserProfile;

/// Spoofs browser characteristics, such as:
/// - platform
/// - architecture
/// - bitness
/// - platform version
///
/// If the browser is Chromium-based, it populates these values;
/// otherwise, it assigns undefined for non-Chromium browsers.
pub fn generate_user_agent_data_script(browser_profile: &BrowserProfile, browser_name: &str, browser_version: &str) -> String {
    let is_chromium = is_chromium_based(browser_name);

    let user_agent_data_platform = browser_profile.ua_data_platform;
    let architecture = browser_profile.architecture;
    let bitness = browser_profile.bitness;
    let platform_version = browser_profile.platform_version;

    let brands = if is_chromium {
        generate_brands(browser_name, browser_version)
    } else {
        "undefined".to_string()
    };

    let platform = if is_chromium {
        format!("'{}'", user_agent_data_platform)
    } else {
        "undefined".to_string()
    };

    let is_mobile = if is_chromium {
        "false".to_string()
    } else {
        "undefined".to_string()
    };

    if is_chromium {
        format!(r#"
            Object.defineProperty(navigator, 'userAgentData', {{
                get: () => ({{
                    brands: {brands},
                    platform: {platform},
                    mobile: {is_mobile},
                    getHighEntropyValues: keys => {{
                        return new Promise(resolve => {{
                            const highEntropyValues = {{
                                architecture: "{architecture}",
                                bitness: "{bitness}",
                                model: "",
                                platformVersion: "{platform_version}"
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
    } else {
        r#"
            Object.defineProperty(navigator, 'userAgentData', {
                get: () => undefined
            });
        "#.to_string()
    }
}

fn generate_brands(browser_name: &str, browser_version: &str) -> String {
    match browser_name {
        "Chrome" | "Google Chrome" => format!(r#"
            [
                {{ brand: "Google Chrome", version: "{browser_version}" }},
                {{ brand: "Chromium", version: "{browser_version}" }},
                {{ brand: "Not_A Brand", version: "24" }}
            ]
        "#),
        "Microsoft Edge" => format!(r#"
            [
                {{ brand: "Microsoft Edge", version: "{browser_version}" }},
                {{ brand: "Chromium", version: "{browser_version}" }},
                {{ brand: "Not_A Brand", version: "24" }}
            ]
        "#),
        _ => panic!("Unsupported browser for brands generation: {}", browser_name),
    }
}
