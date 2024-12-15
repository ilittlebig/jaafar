pub struct BrowserProfile {
    pub ua: &'static str,
    pub ua_data_platform: &'static str,
    pub architecture: &'static str,
    pub bitness: &'static str,
    pub platform_version: &'static str,
    pub webgl_vendor: &'static str,
    pub webgl_renderer: &'static str,
    pub platform: &'static str,
    pub language: &'static str,
    pub languages: &'static [&'static str],
    pub hardware_concurrency: usize,
    pub device_memory: usize,
}

pub static PROFILES: &[BrowserProfile] = &[
    BrowserProfile {
        ua: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        ua_data_platform: "macOS",
        architecture: "arm",
        bitness: "64",
        platform_version: "10.15.7",
        webgl_vendor: "Google Inc. (Apple)",
        webgl_renderer: "ANGLE (Apple, ANGLE Metal Renderer: Apple M1 Pro, Unspecified Version)",
        platform: "MacIntel",
        language: "en-US",
        languages: &["en-US", "en"],
        hardware_concurrency: 10,
        device_memory: 8,
    },
];
