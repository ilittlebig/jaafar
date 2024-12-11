pub struct BrowserProfile {
    pub ua: &'static str,
    pub vendor: &'static str,
    pub webgl_renderer: &'static str,
    pub platform: &'static str,
    pub language: &'static str,
    pub languages: &'static [&'static str],
    pub hardware_concurrency: usize,
    pub device_memory: usize,
}

pub static PROFILES: &[BrowserProfile] = &[
    BrowserProfile {
        ua: "abc",
        vendor: "lol",
        webgl_renderer: "ehm",
        platform: "wtf",
        language: "en-US",
        languages: &["en-US", "en"],
        hardware_concurrency: 8,
        device_memory: 8,
    },
];
