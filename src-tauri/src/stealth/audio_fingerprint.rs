use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;

/// Generates a JavaScript snippet to spoof the audio fingerprint.
///
/// # Returns
/// A JavaScript snippet to spoof the audio fingerprint object.
pub fn generate_audio_fingerprint_script() -> String {
    let mut rng = thread_rng();

    let base_latency = rng.gen_range(0.004..0.008);
    let sample_rate = *[44100, 48000].choose(&mut rng).unwrap();
    let smoothing_time_constant = 0.8; //*[0.7, 0.8, 0.8, 0.8, 0.8, 0.9].choose(&mut rng).unwrap();
    let fft_size = 2048; //*[1024, 2048, 4096].choose(&mut rng).unwrap();
    let frequency_bin_count = fft_size / 2;

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
