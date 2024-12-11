use rand::prelude::SliceRandom;
use crate::data::profiles::{PROFILES, BrowserProfile};

/// Selects a random browser profile from the predefined list of profiles.
///
/// This function ensures that a valid browser profile is always returned by randomly
/// selecting one from the `PROFILES` array. It is used to simulate different browser
/// environments for anti-bot systems.
///
/// # Returns
/// A reference to a randomly selected `BrowserProfile`.
pub fn get_random_profile() -> &'static BrowserProfile {
    let mut rng = rand::thread_rng();
    PROFILES.choose(&mut rng).expect("Profiles list should not be empty")
}
