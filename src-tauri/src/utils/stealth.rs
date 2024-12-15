use crate::utils::plugins::generate_hide_plugins_script;
use crate::data::profiles::BrowserProfile;

///
///
/// # Arguments
/// -
///
/// # Returns
///
pub fn build_stealth_script(browser_profile: &BrowserProfile) -> String {
    let plugins_script = generate_hide_plugins_script();
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
