use rand::thread_rng;
use rand::prelude::SliceRandom;

/// Generates a JavaScript snippet that mocks desktop capabilities, including
/// media queries, color depth (24 or 30), and disabling touch support.
pub fn generate_desktop_capabilities_script() -> String {
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
