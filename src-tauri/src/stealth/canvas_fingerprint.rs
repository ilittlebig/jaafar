pub fn generate_canvas_fingerprint_script() -> String {
    r#"
        const originalToBlob = HTMLCanvasElement.prototype.toBlob;
        const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
        const originalGetImageData = CanvasRenderingContext2D.prototype.getImageData;

        const clamp = (value, min, max) => Math.max(min, Math.min(max, value));

        function mulberry32(a) {
            return function() {
                var t = a += 0x6D2B79F5;
                t = Math.imul(t ^ t >>> 15, t | 1);
                t ^= t + Math.imul(t ^ t >>> 7, t | 61);
                return ((t ^ t >>> 14) >>> 0) / 4294967296;
            }
        }

        const seed = Array.from(navigator.userAgent).reduce((acc, char) => acc + char.charCodeAt(0), 0);
        const rng = mulberry32(seed);

        const NOISE_MIN = -2;
        const NOISE_MAX = 2;

        const generateNoise = () => Math.floor(rng() * (NOISE_MAX - NOISE_MIN + 1)) + NOISE_MIN;

        const applyNoise = (imageData) => {
            const data = imageData.data;
            const length = data.length;

            for (let i = 0; i < length; i += 4) {
                data[i]     = clamp(data[i]     + generateNoise(), 0, 255); // R
                data[i + 1] = clamp(data[i + 1] + generateNoise(), 0, 255); // G
                data[i + 2] = clamp(data[i + 2] + generateNoise(), 0, 255); // B
            }

            return imageData;
        };

        const noisify = function(canvas, ctx) {
            if (!ctx) {
                console.warn('No 2D context available for this canvas.');
                return;
            }

            const width = canvas.width;
            const height = canvas.height;

            try {
                const imageData = originalGetImageData.call(ctx, 0, 0, width, height);
                const noisedImageData = applyNoise(imageData);
                ctx.putImageData(noisedImageData, 0, 0);
            } catch (error) {
                console.error('Error during noisify:', error);
            }
        };

        Object.defineProperty(HTMLCanvasElement.prototype, "toBlob", {
            value: function(callback, type, quality) {
                const ctx = this.getContext("2d", { willReadFrequently: true });
                noisify(this, ctx);
                originalToBlob.call(this, callback, type, quality);
            },
            writable: true,
            configurable: true
        });

        Object.defineProperty(HTMLCanvasElement.prototype, "toDataURL", {
            value: function(type, encoderOptions) {
                const ctx = this.getContext("2d", { willReadFrequently: true });
                noisify(this, ctx);
                return originalToDataURL.call(this, type, encoderOptions);
            },
            writable: true,
            configurable: true
        });

        Object.defineProperty(CanvasRenderingContext2D.prototype, "getImageData", {
            value: function(sx, sy, sw, sh) {
                noisify(this.canvas, this);
                return originalGetImageData.call(this, sx, sy, sw, sh);
            },
            writable: true,
            configurable: true
        });
    "#.to_string()
}
