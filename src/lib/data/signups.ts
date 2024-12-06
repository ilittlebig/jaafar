/**
 *
 *
 * Author: Elias Sjödin
 * Created: 2024-12-03
 */

export const signupCommands: { [key: string]: string } = {
	"sabrina-hallenstadion": "sabrina_hallenstadion",
	"sabrina-unity-arena": "sabrina_unity_arena",
	"sabrina-avicii-arena": "sabrina_avicii_arena",
	"random-unicorn-arena": "random_unicorn_arena",
}

export const signupProducts = [
	{
		group: "Sabrina Carpenter",
		options: [
			{ value: "sabrina-hallenstadion", label: "Sabrina Carpenter - 3/27 - Hallenstadion" },
			{ value: "sabrina-unity-arena", label: "Sabrina Carpenter - 3/30 - Unity Arena" },
			{ value: "sabrina-royal-arena", label: "Sabrina Carpenter - 3/31 - Royal Arena" },
			{ value: "sabrina-avicii-arena", label: "Sabrina Carpenter - 4/3 - Avicii Arena" },
		],
	},
	{ value: "random-unicorn-arena", label: "Random Artist - 24/24 - Unicorn Arena" },
];

export const signupModes = [
	{ value: "manual_confirm", label: "Manual Confirmation"},
	{ value: "auto_confirm", label: "Auto Confirmation" },
];
