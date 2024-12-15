pub struct Voice {
    pub voice_uri: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub local_service: bool,
    pub default: bool,
}

pub static SPEECH_SYNTHESIS_VOICES: &[Voice] = &[
    Voice {
        voice_uri: "Samantha",
        name: "Samantha",
        lang: "en-US",
        local_service: true,
        default: true
    },
    Voice {
        voice_uri: "Aaron",
        name: "Aaron",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Albert",
        name: "Albert",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Alice",
        name: "Alice",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Alva",
        name: "Alva",
        lang: "sv-SE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Amélie",
        name: "Amélie",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Amira",
        name: "Amira",
        lang: "ms-MY",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Anna",
        name: "Anna",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Arthur",
        name: "Arthur",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Bad News",
        name: "Bad News",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Bahh",
        name: "Bahh",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Bells",
        name: "Bells",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Boing",
        name: "Boing",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Bubbles",
        name: "Bubbles",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Carmit",
        name: "Carmit",
        lang: "he-IL",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Catherine",
        name: "Catherine",
        lang: "en-AU",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Cellos",
        name: "Cellos",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Damayanti",
        name: "Damayanti",
        lang: "id-ID",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Daniel (English (United Kingdom))",
        name: "Daniel (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Daniel (French (France))",
        name: "Daniel (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Daria",
        name: "Daria",
        lang: "bg-BG",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (German (Germany))",
        name: "Eddy (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (English (United Kingdom))",
        name: "Eddy (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (English (United States))",
        name: "Eddy (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Spanish (Spain))",
        name: "Eddy (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Spanish (Mexico))",
        name: "Eddy (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Finnish (Finland))",
        name: "Eddy (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (French (Canada))",
        name: "Eddy (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (French (France))",
        name: "Eddy (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Italian (Italy))",
        name: "Eddy (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Japanese (Japan))",
        name: "Eddy (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Korean (South Korea))",
        name: "Eddy (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Portuguese (Brazil))",
        name: "Eddy (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Chinese (China mainland))",
        name: "Eddy (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Eddy (Chinese (Taiwan))",
        name: "Eddy (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Ellen",
        name: "Ellen",
        lang: "nl-BE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (German (Germany))",
        name: "Flo (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (English (United Kingdom))",
        name: "Flo (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (English (United States))",
        name: "Flo (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Spanish (Spain))",
        name: "Flo (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Spanish (Mexico))",
        name: "Flo (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Finnish (Finland))",
        name: "Flo (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (French (Canada))",
        name: "Flo (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (French (France))",
        name: "Flo (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Italian (Italy))",
        name: "Flo (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Japanese (Japan))",
        name: "Flo (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Korean (South Korea))",
        name: "Flo (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Portuguese (Brazil))",
        name: "Flo (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Chinese (China mainland))",
        name: "Flo (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Flo (Chinese (Taiwan))",
        name: "Flo (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Fred",
        name: "Fred",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Good News",
        name: "Good News",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Gordon",
        name: "Gordon",
        lang: "en-AU",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (German (Germany))",
        name: "Grandma (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (English (United Kingdom))",
        name: "Grandma (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (English (United States))",
        name: "Grandma (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Spanish (Spain))",
        name: "Grandma (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Spanish (Mexico))",
        name: "Grandma (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Finnish (Finland))",
        name: "Grandma (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (French (Canada))",
        name: "Grandma (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (French (France))",
        name: "Grandma (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Italian (Italy))",
        name: "Grandma (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Japanese (Japan))",
        name: "Grandma (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Korean (South Korea))",
        name: "Grandma (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Portuguese (Brazil))",
        name: "Grandma (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Chinese (China mainland))",
        name: "Grandma (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandma (Chinese (Taiwan))",
        name: "Grandma (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (German (Germany))",
        name: "Grandpa (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (English (United Kingdom))",
        name: "Grandpa (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (English (United States))",
        name: "Grandpa (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Spanish (Spain))",
        name: "Grandpa (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Spanish (Mexico))",
        name: "Grandpa (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Finnish (Finland))",
        name: "Grandpa (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (French (Canada))",
        name: "Grandpa (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (French (France))",
        name: "Grandpa (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Italian (Italy))",
        name: "Grandpa (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Japanese (Japan))",
        name: "Grandpa (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Korean (South Korea))",
        name: "Grandpa (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Portuguese (Brazil))",
        name: "Grandpa (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Chinese (China mainland))",
        name: "Grandpa (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Grandpa (Chinese (Taiwan))",
        name: "Grandpa (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Hattori",
        name: "Hattori",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Helena",
        name: "Helena",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Ioana",
        name: "Ioana",
        lang: "ro-RO",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Jacques",
        name: "Jacques",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Jester",
        name: "Jester",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Joana",
        name: "Joana",
        lang: "pt-PT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Junior",
        name: "Junior",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Kanya",
        name: "Kanya",
        lang: "th-TH",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Karen",
        name: "Karen",
        lang: "en-AU",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Kathy",
        name: "Kathy",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Kyoko",
        name: "Kyoko",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Lana",
        name: "Lana",
        lang: "hr-HR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Laura",
        name: "Laura",
        lang: "sk-SK",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Lekha",
        name: "Lekha",
        lang: "hi-IN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Lesya",
        name: "Lesya",
        lang: "uk-UA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Li-Mu",
        name: "Li-Mu",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Linh",
        name: "Linh",
        lang: "vi-VN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Luciana",
        name: "Luciana",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Majed",
        name: "Majed",
        lang: "ar-001",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Marie",
        name: "Marie",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Martha",
        name: "Martha",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Martin",
        name: "Martin",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Meijia",
        name: "Meijia",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Melina",
        name: "Melina",
        lang: "el-GR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Milena",
        name: "Milena",
        lang: "ru-RU",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Moira",
        name: "Moira",
        lang: "en-IE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Montse",
        name: "Montse",
        lang: "ca-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Mónica",
        name: "Mónica",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Nicky",
        name: "Nicky",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Nora",
        name: "Nora",
        lang: "nb-NO",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "O-Ren",
        name: "O-Ren",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Organ",
        name: "Organ",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Paulina",
        name: "Paulina",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Ralph",
        name: "Ralph",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (German (Germany))",
        name: "Reed (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (English (United Kingdom))",
        name: "Reed (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (English (United States))",
        name: "Reed (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Spanish (Spain))",
        name: "Reed (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Spanish (Mexico))",
        name: "Reed (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Finnish (Finland))",
        name: "Reed (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (French (Canada))",
        name: "Reed (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Italian (Italy))",
        name: "Reed (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Japanese (Japan))",
        name: "Reed (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Korean (South Korea))",
        name: "Reed (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Portuguese (Brazil))",
        name: "Reed (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Chinese (China mainland))",
        name: "Reed (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Reed (Chinese (Taiwan))",
        name: "Reed (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rishi",
        name: "Rishi",
        lang: "en-IN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (German (Germany))",
        name: "Rocko (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (English (United Kingdom))",
        name: "Rocko (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (English (United States))",
        name: "Rocko (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Spanish (Spain))",
        name: "Rocko (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Spanish (Mexico))",
        name: "Rocko (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Finnish (Finland))",
        name: "Rocko (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (French (Canada))",
        name: "Rocko (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (French (France))",
        name: "Rocko (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Italian (Italy))",
        name: "Rocko (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Japanese (Japan))",
        name: "Rocko (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Korean (South Korea))",
        name: "Rocko (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Portuguese (Brazil))",
        name: "Rocko (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Chinese (China mainland))",
        name: "Rocko (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Rocko (Chinese (Taiwan))",
        name: "Rocko (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (German (Germany))",
        name: "Sandy (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (English (United Kingdom))",
        name: "Sandy (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (English (United States))",
        name: "Sandy (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Spanish (Spain))",
        name: "Sandy (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Spanish (Mexico))",
        name: "Sandy (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Finnish (Finland))",
        name: "Sandy (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (French (Canada))",
        name: "Sandy (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (French (France))",
        name: "Sandy (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Italian (Italy))",
        name: "Sandy (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Japanese (Japan))",
        name: "Sandy (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Korean (South Korea))",
        name: "Sandy (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Portuguese (Brazil))",
        name: "Sandy (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Chinese (China mainland))",
        name: "Sandy (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sandy (Chinese (Taiwan))",
        name: "Sandy (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sara",
        name: "Sara",
        lang: "da-DK",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Satu",
        name: "Satu",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (German (Germany))",
        name: "Shelley (German (Germany))",
        lang: "de-DE",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (English (United Kingdom))",
        name: "Shelley (English (United Kingdom))",
        lang: "en-GB",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (English (United States))",
        name: "Shelley (English (United States))",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Spanish (Spain))",
        name: "Shelley (Spanish (Spain))",
        lang: "es-ES",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Spanish (Mexico))",
        name: "Shelley (Spanish (Mexico))",
        lang: "es-MX",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Finnish (Finland))",
        name: "Shelley (Finnish (Finland))",
        lang: "fi-FI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (French (Canada))",
        name: "Shelley (French (Canada))",
        lang: "fr-CA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (French (France))",
        name: "Shelley (French (France))",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Italian (Italy))",
        name: "Shelley (Italian (Italy))",
        lang: "it-IT",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Japanese (Japan))",
        name: "Shelley (Japanese (Japan))",
        lang: "ja-JP",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Korean (South Korea))",
        name: "Shelley (Korean (South Korea))",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Portuguese (Brazil))",
        name: "Shelley (Portuguese (Brazil))",
        lang: "pt-BR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Chinese (China mainland))",
        name: "Shelley (Chinese (China mainland))",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Shelley (Chinese (Taiwan))",
        name: "Shelley (Chinese (Taiwan))",
        lang: "zh-TW",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Sinji",
        name: "Sinji",
        lang: "zh-HK",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Superstar",
        name: "Superstar",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Tessa",
        name: "Tessa",
        lang: "en-ZA",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Thomas",
        name: "Thomas",
        lang: "fr-FR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Tina",
        name: "Tina",
        lang: "sl-SI",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Tingting",
        name: "Tingting",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Trinoids",
        name: "Trinoids",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Tünde",
        name: "Tünde",
        lang: "hu-HU",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Whisper",
        name: "Whisper",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Wobble",
        name: "Wobble",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Xander",
        name: "Xander",
        lang: "nl-NL",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Yelda",
        name: "Yelda",
        lang: "tr-TR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Yu-shu",
        name: "Yu-shu",
        lang: "zh-CN",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Yuna",
        name: "Yuna",
        lang: "ko-KR",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Zarvox",
        name: "Zarvox",
        lang: "en-US",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Zosia",
        name: "Zosia",
        lang: "pl-PL",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Zuzana",
        name: "Zuzana",
        lang: "cs-CZ",
        local_service: true,
        default: false
    },
    Voice {
        voice_uri: "Google Deutsch",
        name: "Google Deutsch",
        lang: "de-DE",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google US English",
        name: "Google US English",
        lang: "en-US",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google UK English Female",
        name: "Google UK English Female",
        lang: "en-GB",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google UK English Male",
        name: "Google UK English Male",
        lang: "en-GB",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google español",
        name: "Google español",
        lang: "es-ES",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google español de Estados Unidos",
        name: "Google español de Estados Unidos",
        lang: "es-US",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google français",
        name: "Google français",
        lang: "fr-FR",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google हिन्दी",
        name: "Google हिन्दी",
        lang: "hi-IN",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google Bahasa Indonesia",
        name: "Google Bahasa Indonesia",
        lang: "id-ID",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google italiano",
        name: "Google italiano",
        lang: "it-IT",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google 日本語",
        name: "Google 日本語",
        lang: "ja-JP",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google 한국의",
        name: "Google 한국의",
        lang: "ko-KR",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google Nederlands",
        name: "Google Nederlands",
        lang: "nl-NL",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google polski",
        name: "Google polski",
        lang: "pl-PL",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google português do Brasil",
        name: "Google português do Brasil",
        lang: "pt-BR",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google русский",
        name: "Google русский",
        lang: "ru-RU",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google 普通话（中国大陆）",
        name: "Google 普通话（中国大陆）",
        lang: "zh-CN",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google 粤語（香港）",
        name: "Google 粤語（香港）",
        lang: "zh-HK",
        local_service: false,
        default: false
    },
    Voice {
        voice_uri: "Google 國語（臺灣）",
        name: "Google 國語（臺灣）",
        lang: "zh-TW",
        local_service: false,
        default: false
    },
];
