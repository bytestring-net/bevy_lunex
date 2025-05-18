use std::hash::{DefaultHasher, Hash, Hasher};
use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::*;

#[derive(Component, Reflect, Clone, PartialEq, Debug)]
enum DurationMode {
    CharSpeed(f32),
    AnimDuration(f32,)
}

/// This component modifies attached [`Text2d`] with a modified string outputted from a time dependant function.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct TextAnimator {
    string: String,
    function: fn(t: f32, text: &str) -> String,
    counter: f32,
    mode: DurationMode,
}
impl Default for TextAnimator {
    fn default() -> Self {
        Self {
            string: String::new(),
            function: typing_animation_underscore,
            counter: 0.0,
            mode: DurationMode::AnimDuration(5.0),
        }
    }
}
impl TextAnimator {
    /// Creates new instance
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            string: text.into(),
            ..Default::default()
        }
    }
    /// Replace the text with a new one and resets the animation.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.string = text.into();
        self.counter = 0.0;
    }
    /// Replace the text with a new one while not reseting the animation.
    pub fn set_text_quiet(&mut self, text: impl Into<String>) {
        self.string = text.into();
        match self.mode {
            DurationMode::AnimDuration(duration) => {
                self.counter = duration;
            },
            DurationMode::CharSpeed(_) => {
                self.counter = 1.0 + self.string.trim().chars().count() as f32
            }
        }
    }
    /// Replace the default function with a new one. The function provided takes time as input and original string and outputs modified string.
    pub fn function(mut self, function: fn(t: f32, text: &str) -> String) -> Self {
        self.function = function;
        self
    }
    /// Replace the default speed in seconds with a new one.
    pub fn speed(mut self, speed: f32) -> Self {
        self.mode = DurationMode::CharSpeed(speed);
        self
    }
    /// Replace the default duration in seconds with a new one.
    pub fn duration(mut self, duration: f32) -> Self {
        self.mode = DurationMode::AnimDuration(duration);
        self
    }
    /// This system takes care of updating the TextAnimator in time.
    pub(crate) fn system_2d(mut query: Query<(&mut Text2d, &mut TextAnimator)>, time: Res<Time>, mut commads: Commands) {
        for (mut text, mut animator) in &mut query {
            match animator.mode {
                DurationMode::CharSpeed(speed) => {
                    let chars = 1.0 + animator.string.trim().chars().count() as f32;

                    // Increment the time counter
                    let mut modified = false;
                    if animator.counter < chars { animator.counter += time.delta_secs() * speed; modified = true; }
                    animator.counter = animator.counter.min(chars);

                    // Continue if not changed
                    if !modified { continue; }
                   
                    // Check if the new string will get changed somehow
                    let new_text = (animator.function)(animator.counter/chars, &animator.string);
                    if new_text.as_str() != text.as_str() {
                        
                        // Change the target string
                        text.0 = new_text;
                        commads.trigger(RecomputeUiLayout);
                    }
                    
                }
                DurationMode::AnimDuration(duration) => {
                    // Increment the time counter
                    let mut modified = false;
                    if animator.counter < duration { animator.counter += time.delta_secs(); modified = true; }
                    animator.counter = animator.counter.min(duration);

                    // Continue if not changed
                    if !modified { continue; }

                    // Check if the new string will get changed somehow
                    let new_text = (animator.function)(animator.counter/duration, &animator.string);
                    if new_text.as_str() != text.as_str() {
                        
                        // Change the target string
                        text.0 = new_text;
                        commads.trigger(RecomputeUiLayout);
                    }
                },
            }
        }
    }
    /// This system takes care of updating the TextAnimator in time.
    #[cfg(feature = "text3d")]
    pub(crate) fn system_3d(mut query: Query<(&mut Text3d, &mut TextAnimator)>, time: Res<Time>, mut commads: Commands) {
        for (mut text, mut animator) in &mut query {
            match animator.mode {
                DurationMode::CharSpeed(speed) => {
                    let chars = 1.0 + animator.string.trim().chars().count() as f32;

                    // Increment the time counter
                    let mut modified = false;
                    if animator.counter < chars { animator.counter += time.delta_secs() * speed; modified = true; }
                    animator.counter = animator.counter.min(chars);

                    // Continue if not changed
                    if !modified { continue; }
                   
                    // Check if the new string will get changed somehow
                    let new_text = (animator.function)(animator.counter/chars, &animator.string);
                    if new_text != text.get_single().expect("Multisegment 3D text not supported, make a PR to Lunex if you need it") {
                        
                        // Change the target string
                        let text = text.get_single_mut().expect("Multisegment 3D text not supported, make a PR to Lunex if you need it");
                        *text = new_text;
                        commads.trigger(RecomputeUiLayout);
                    }
                    
                }
                DurationMode::AnimDuration(duration) => {
                    // Increment the time counter
                    let mut modified = false;
                    if animator.counter < duration { animator.counter += time.delta_secs(); modified = true; }
                    animator.counter = animator.counter.min(duration);

                    // Continue if not changed
                    if !modified { continue; }

                    // Check if the new string will get changed somehow
                    let new_text = (animator.function)(animator.counter/duration, &animator.string);
                    if new_text != text.get_single().expect("Multisegment 3D text not supported, make a PR to Lunex if you need it") {
                        
                        // Change the target string
                        let text = text.get_single_mut().expect("Multisegment 3D text not supported, make a PR to Lunex if you need it");
                        *text = new_text;
                        commads.trigger(RecomputeUiLayout);
                    }
                },
            }
        }
    }
}

/// Simulates typing animation with an underscore cursor
pub fn typing_animation_underscore(t: f32, text: &str) -> String {
    let visible_chars = (t * text.len() as f32).floor() as usize;
    let visible_chars = visible_chars.min(text.len());

    if visible_chars < text.len() {
        // Show typed characters plus cursor
        format!("{}{}", &text[..visible_chars], "_")
    } else {
        // All characters visible, show cursor at end
        text.to_string()
    }
}

/// Simulates typing animation with an vertical line cursor
pub fn typing_animation_cursor(t: f32, text: &str) -> String {
    let visible_chars = (t * text.len() as f32).floor() as usize;
    let visible_chars = visible_chars.min(text.len());

    if visible_chars < text.len() {
        // Show typed characters plus cursor
        format!("{}{}", &text[..visible_chars], "|")
    } else {
        // All characters visible, show cursor at end
        text.to_string()
    }
}

/// Creates a decryption effect where random symbols gradually become the actual text
pub fn decryption_animation(t: f32, text: &str) -> String {

    // Hash input data into unique seed
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let seed: u64 = hasher.finish();

    // Create unique reproducible RNG from time
    let mut rng = StdRng::seed_from_u64(seed + (t*60.0).round() as u64);

    // Define symbols used
    let symbols = "!@#$%^&*()_+-=[]{}|;:'\",.<>/?`~";
    let mut result = String::with_capacity(text.len());

    for (i, c) in text.chars().enumerate() {
        let char_progress = (t * text.len() as f32) - i as f32;

        if char_progress < 0.0 {
            // Not yet started decrypting this character
            result.push(symbols.chars().nth(rng.random_range(0..symbols.len())).unwrap());
        } else if char_progress >= 1.0 {
            // This character is fully decrypted
            result.push(c);
        } else {
            // This character is in the process of being decrypted
            // 80% chance of showing the real character as we get closer to 1.0
            if rng.random::<f32>() < char_progress {
                result.push(c);
            } else {
                result.push(symbols.chars().nth(rng.random_range(0..symbols.len())).unwrap());
            }
        }
    }

    result
}

/// Creates a slide-in effect where characters come in from the sides
pub fn slide_in_animation(t: f32, text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let center = text.len() / 2;

    for (i, c) in text.chars().enumerate() {
        let distance_from_center = center.abs_diff(i);

        let char_progress = t * 2.0 - (distance_from_center as f32 / center as f32);

        if char_progress >= 1.0 {
            // Character is fully visible
            result.push(c);
        } else if char_progress > 0.0 {
            // Character is sliding in
            result.push('_');
        } else {
            // Character hasn't started appearing yet
            result.push(' ');
        }
    }

    result
}

/// Reveals characters in a scrambled order
pub fn scrambled_reveal_animation(t: f32, text: &str) -> String {
    // Create a seeded RNG for consistent scrambling
    let mut indices: Vec<usize> = (0..text.len()).collect();
    let seed = 42; // Fixed seed for consistent scrambling
    let mut rng = StdRng::seed_from_u64(seed);

    // Shuffle indices to determine reveal order
    use rand::seq::SliceRandom;
    indices.shuffle(&mut rng);

    let chars_to_reveal = (t * text.len() as f32).floor() as usize;
    let mut result = vec![' '; text.len()];

    // Reveal characters in scrambled order
    for i in indices.iter().take(chars_to_reveal.min(text.len())) {
        result[*i] = text.chars().nth(*i).unwrap();
    }

    result.into_iter().collect()
}



/// This plugin is used for the main logic.
#[derive(Debug, Default, Clone)]
pub struct UiLunexAnimPlugin;
impl Plugin for UiLunexAnimPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Update, TextAnimator::system_2d);

        // Add text 3d support
        #[cfg(feature = "text3d")] {
            app.add_systems(Update, TextAnimator::system_3d);
        }
    }
}