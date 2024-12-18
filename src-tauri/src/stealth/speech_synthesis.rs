use std::fmt::Write;
use rand::Rng;
use rand::prelude::SliceRandom;

use crate::utils::browser::is_chromium_based;
use crate::data::speech_synthesis::SPEECH_SYNTHESIS_VOICES;

/// Generates a mock speech synthesis script with random voices.
/// If the browser is Chromium-based, it shuffles and selects a random number of voices.
/// Otherwise, it uses predefined voices.
pub fn generate_speech_synthesis_script(browser_name: &str) -> String {
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
