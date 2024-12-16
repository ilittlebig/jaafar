use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fmt::Write;
use user_agent_parser::UserAgentParser;

use crate::utils::plugins::generate_hide_plugins_script;
use crate::data::profiles::BrowserProfile;
use crate::data::speech_synthesis::SPEECH_SYNTHESIS_VOICES;

fn is_chromium_based(browser_name: &str) -> bool {
    false//matches!(browser_name, "Chrome" | "Google Chrome" | "Microsoft Edge")
}

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
    let hardware_concurrency = browser_profile.hardware_concurrency;

    let plugins_script = generate_hide_plugins_script();
    let user_agent_data_script = generate_user_agent_data_string(browser_profile, browser_name, browser_version);
    let speech_synthesis_script = generate_speech_synthesis_script(browser_name);
    let desktop_capabilities_script = generate_desktop_capabilities_script();
    let audio_fingerprint_script = generate_audio_fingerprint_script();

    let main_script = format!(r#"
        Object.defineProperty(navigator, 'platform', {{ get: () => '{platform}' }});
        Object.defineProperty(navigator, 'vendor', {{ get: () => '{webgl_vendor}' }});
        Object.defineProperty(navigator, 'hardwareConcurrency', {{ get: () => {hardware_concurrency} }});
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

/// Spoofs browser characteristics, such as:
/// - platform
/// - architecture
/// - bitness
/// - platform version
///
/// If the browser is Chromium-based, it populates these values;
/// otherwise, it assigns undefined for non-Chromium browsers.
fn generate_user_agent_data_string(browser_profile: &BrowserProfile, browser_name: &str, browser_version: &str) -> String {
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

/// Generates a mock speech synthesis script with random voices.
/// If the browser is Chromium-based, it shuffles and selects a random number of voices.
/// Otherwise, it uses predefined voices.
fn generate_speech_synthesis_script(browser_name: &str) -> String {
    let is_chromium = is_chromium_based(browser_name);
    let mut rng = rand::thread_rng();
    let mut speech_synthesis = String::new();

    let mut shuffled_voices = SPEECH_SYNTHESIS_VOICES.to_vec();
    shuffled_voices.shuffle(&mut rng);

    let total_voices = rng.gen_range(19..=209);
    for (i, voice) in shuffled_voices.iter().enumerate() {
        if i >= total_voices {
            break;
        }

        let voice_uri = voice.voice_uri;
        let name = voice.name;
        let lang = voice.lang;
        let local_service = voice.local_service;
        let default = i == 0;

        write!(speech_synthesis, r#"
            {{
                voiceURI: "{voice_uri}",
                name: "{name}",
                lang: "{lang}",
                localService: {local_service},
                default: {default}
            }},
        "#).unwrap();
    }

    let non_chromium_voices = r#"
        { voiceURI: "Samantha", name: "Samantha", lang: "en-US", localService: true, default: false },
        { voiceURI: "Aaron", name: "Aaron", lang: "en-US", localService: true, default: false },
    "#;

    let speech_synthesis = if is_chromium {
        speech_synthesis
    } else {
        non_chromium_voices.to_string()
    };

    format!(r#"
        Object.defineProperty(window, 'speechSynthesis', {{
            value: {{
                getVoices: () => [{speech_synthesis}]
            }}
        }});
    "#)
}

/// Generates a JavaScript snippet that mocks desktop capabilities, including
/// media queries, color depth (24 or 30), and disabling touch support.
fn generate_desktop_capabilities_script() -> String {
    let mut rng = thread_rng();
    let color_depth_options = [24, 30];
    let color_depth = *color_depth_options.choose(&mut rng).unwrap();

    format!(r#"
        const originalMatchMedia = window.matchMedia;
        const mediaQueryMatches = {{
            'any-pointer: fine': true,
            'any-pointer: any': true,
            'any-pointer: coarse': false,
            'pointer: fine': true,
            'pointer: any': true,
            'pointer: coarse': false,
            'any-hover: hover': true,
            'any-hover: any': true,
            'any-hover: none': false,
            'hover: hover': true,
            'hover: any': true,
            'hover: none': false
        }};

        const parseMediaQuery = query => {{
            query = query.trim();

            const conditions = query.match(/\([^)]+\)/g);
            if (!conditions) {{
                return originalMatchMedia.call(window, query).matches;
            }}

            for (let condition of conditions) {{
                condition = condition.slice(1, -1).trim();
                if (condition in mediaQueryMatches) {{
                    if (!mediaQueryMatches[condition]) {{
                        return false;
                    }}
                }} else {{
                    return originalMatchMedia.call(window, query).matches;
                }}
            }}

            return true;
        }}

        window.matchMedia = function(query) {{
            const matches = parseMediaQuery(query);
            return {{
                matches: matches,
                media: query,
                onchange: null,
                addListener: function() {{}},
                removeListener: function() {{}},
                addEventListener: function() {{}},
                removeEventListener: function() {{}},
                dispatchEvent: function() {{ return false; }}
            }};
        }};

        Object.defineProperty(screen, 'colorDepth', {{
            get: function() {{ return {color_depth}; }},
            configurable: false
        }});

        Object.defineProperty(navigator, 'maxTouchPoints', {{
            value: 0,
            writable: false,
            configurable: false
        }});
    "#)
}

///
///
///
fn generate_audio_fingerprint_script() -> String {
    let mut rng = rand::thread_rng();

    let base_latency = rng.gen_range(0.004..0.008);
    let sample_rate = *[44100, 48000, 32000, 22050, 16000].choose(&mut rng).unwrap();
    let fft_size = *[1024, 2048, 4096].choose(&mut rng).unwrap();
    let frequency_bin_count = fft_size / 2;
    let smoothing_time_constant = rng.gen_range(0.7..0.9);

    let desired_pxi_output = rng.gen_range(100.0..150.0);
    let spoofed_sample_value = desired_pxi_output / 500.0;

    format!(r#"
        const spoofedValues = {{
            baseLatency: {base_latency},
            outputLatency: 0,
            sampleRate: {sample_rate},
            fftSize: {fft_size},
            frequencyBinCount: {frequency_bin_count},
            minDecibels: -100.0,
            maxDecibels: -30.0,
            smoothingTimeConstant: {smoothing_time_constant}
        }};

        const defineSpoofedProperty = (target, property, value) => {{
            Object.defineProperty(target, property, {{
                get: () => value,
                set: () => {{}},
                configurable: false,
                enumerable: true
            }});
        }}

        const OriginalAudioContext = window.AudioContext || window.webkitAudioContext;
        window.AudioContext = new Proxy(OriginalAudioContext, {{
            construct(target, args) {{
                const instance = new target(...args);
                defineSpoofedProperty(instance, 'baseLatency', spoofedValues.baseLatency);
                defineSpoofedProperty(instance, 'outputLatency', spoofedValues.outputLatency);
                defineSpoofedProperty(instance, 'sampleRate', spoofedValues.sampleRate);
                return instance;
            }}
        }});

        const OriginalOfflineAudioContext = window.OfflineAudioContext || window.webkitOfflineAudioContext;
        window.OfflineAudioContext = new Proxy(OriginalOfflineAudioContext, {{
            construct(target, args) {{
                const instance = new target(...args);
                const originalCreateAnalyser = instance.createAnalyser.bind(instance);

                instance.createAnalyser = function() {{
                    const analyser = originalCreateAnalyser();
                    defineSpoofedProperty(analyser, 'fftSize', spoofedValues.fftSize);
                    defineSpoofedProperty(analyser, 'frequencyBinCount', spoofedValues.frequencyBinCount);
                    defineSpoofedProperty(analyser, 'minDecibels', spoofedValues.minDecibels);
                    defineSpoofedProperty(analyser, 'maxDecibels', spoofedValues.maxDecibels);
                    defineSpoofedProperty(analyser, 'smoothingTimeConstant', spoofedValues.smoothingTimeConstant);
                    return analyser;
                }};
                return instance;
            }}
        }});

        window.webkitAudioContext = window.AudioContext;
        window.webkitOfflineAudioContext = window.OfflineAudioContext;

        if (window.AnalyserNode) {{
            defineSpoofedProperty(AnalyserNode.prototype, 'fftSize', spoofedValues.fftSize);
            defineSpoofedProperty(AnalyserNode.prototype, 'frequencyBinCount', spoofedValues.frequencyBinCount);
            defineSpoofedProperty(AnalyserNode.prototype, 'minDecibels', spoofedValues.minDecibels);
            defineSpoofedProperty(AnalyserNode.prototype, 'maxDecibels', spoofedValues.maxDecibels);
            defineSpoofedProperty(AnalyserNode.prototype, 'smoothingTimeConstant', spoofedValues.smoothingTimeConstant);
        }}

        const OriginalGetChannelData = AudioBuffer.prototype.getChannelData;
        AudioBuffer.prototype.getChannelData = function(channel) {{
            const data = OriginalGetChannelData.call(this, channel);
            for (let i = 4500; i < 5000 && i < data.length; i++) {{
                data[i] = {spoofed_sample_value};
            }}
            return data;
        }};

        Object.freeze(spoofedValues);
    "#)
}
