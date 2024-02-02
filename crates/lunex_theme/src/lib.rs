use bevy::prelude::*;


pub mod prelude {
    pub use super::Theme;
}

// Theme Plugin (has system looping through all containers)


/// # Lunex Theme
/// Containes all ECS components for styling the container.
/// Can be compared to TailwindCSS classes.
pub mod ltm {
    use bevy::ecs::component::Component;

    /// # NodeData
    /// Marker component to apply container styling to the container
    #[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
    pub struct NodeData;


    /// # Base Color
    /// Color of the container elements
    #[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
    pub struct BaseColor(pub crate::ThemeColor);

    /// # Text Color
    /// Color of the container's text
    #[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
    pub struct TextColor(pub crate::ThemeColor);

    /// # Opacity
    /// Overall opacity of the container
    #[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
    pub struct Opacity(pub f32);

    /// # Base Opacity
    /// Opacity of the container elements
    #[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
    pub struct BaseOpacity(pub f32);

    /// # Font Opacity
    /// Opacity of the container's text
    #[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
    pub struct TextOpacity(pub f32);
}


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ColorPair {
    pub base_color: Color,
    pub text_color: Color,
}
impl ColorPair {
    pub fn v(self, l: f32) -> Color {
        self.base_color.with_l((500.0 - l)/500.0)
    }
}

/// # Mode
/// 
/// * [Mode::Light]
/// * [Mode::Dark]
/// * [Mode::Neutral]
pub enum Mode {
    /// Use colors offset for light mode
    Light,
    /// Use colors offset for dark mode
    Dark,
    /// Use colors as they are with no offset
    Neutral,
}

pub struct Theme {
    pub name: String,
    pub mode: Mode,         // Should be in CurrentTheme struct instead of the preset?

    pub primary   : ColorPair,
    pub secondary : ColorPair,
    pub tertiary  : ColorPair,
    pub quaternery: ColorPair,
    pub info      : ColorPair,
    pub warning   : ColorPair,
    pub success   : ColorPair,
    pub error     : ColorPair,
    pub surface   : ColorPair,

    pub font_base: Handle<Font>,
    pub font_heading: Handle<Font>,

    pub text_color: Color,

    pub rounding_container: f32,
    pub rounding_base     : f32,
    pub border_width      : f32,
    pub border_color      : ThemeColor,
}

pub enum Rounded {
    None,
    XS,
    SM,
    MD,
    LG,
    XL,
    XL2,
    XL3,
    XL4,
    XL5,
    XL6,
    XL7,
    Full,
    Custom(f32)
}

/// ## Theme Color
/// A specific color picked from predefined color pool.
/// Allows for easy color swapping.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeColor {
    Primary(f32),
    Secondary(f32),
    Tertiary(f32),
    Quaternery(f32),
    Info(f32),
    Warning(f32),
    Success(f32),
    Error(f32),
    Surface(f32),
    Neutral(f32),
    Custom(ColorPair, f32),
}
impl ThemeColor {
    pub const PRIMARY_50: ThemeColor = ThemeColor::Primary(50.0);
    pub const PRIMARY_100: ThemeColor = ThemeColor::Primary(100.0);
    pub const PRIMARY_200: ThemeColor = ThemeColor::Primary(200.0);
    pub const PRIMARY_300: ThemeColor = ThemeColor::Primary(300.0);
    pub const PRIMARY_400: ThemeColor = ThemeColor::Primary(400.0);
    pub const PRIMARY_500: ThemeColor = ThemeColor::Primary(500.0);
    pub const PRIMARY_600: ThemeColor = ThemeColor::Primary(600.0);
    pub const PRIMARY_700: ThemeColor = ThemeColor::Primary(700.0);
    pub const PRIMARY_800: ThemeColor = ThemeColor::Primary(800.0);
    pub const PRIMARY_900: ThemeColor = ThemeColor::Primary(900.0);

    pub const SECONDARY_50: ThemeColor = ThemeColor::Secondary(50.0);
    pub const SECONDARY_100: ThemeColor = ThemeColor::Secondary(100.0);
    pub const SECONDARY_200: ThemeColor = ThemeColor::Secondary(200.0);
    pub const SECONDARY_300: ThemeColor = ThemeColor::Secondary(300.0);
    pub const SECONDARY_400: ThemeColor = ThemeColor::Secondary(400.0);
    pub const SECONDARY_500: ThemeColor = ThemeColor::Secondary(500.0);
    pub const SECONDARY_600: ThemeColor = ThemeColor::Secondary(600.0);
    pub const SECONDARY_700: ThemeColor = ThemeColor::Secondary(700.0);
    pub const SECONDARY_800: ThemeColor = ThemeColor::Secondary(800.0);
    pub const SECONDARY_900: ThemeColor = ThemeColor::Secondary(900.0);

    pub const TERTIARY_50: ThemeColor = ThemeColor::Tertiary(50.0);
    pub const TERTIARY_100: ThemeColor = ThemeColor::Tertiary(100.0);
    pub const TERTIARY_200: ThemeColor = ThemeColor::Tertiary(200.0);
    pub const TERTIARY_300: ThemeColor = ThemeColor::Tertiary(300.0);
    pub const TERTIARY_400: ThemeColor = ThemeColor::Tertiary(400.0);
    pub const TERTIARY_500: ThemeColor = ThemeColor::Tertiary(500.0);
    pub const TERTIARY_600: ThemeColor = ThemeColor::Tertiary(600.0);
    pub const TERTIARY_700: ThemeColor = ThemeColor::Tertiary(700.0);
    pub const TERTIARY_800: ThemeColor = ThemeColor::Tertiary(800.0);
    pub const TERTIARY_900: ThemeColor = ThemeColor::Tertiary(900.0);

    pub const QUATERNARY_50: ThemeColor = ThemeColor::Quaternery(50.0);
    pub const QUATERNARY_100: ThemeColor = ThemeColor::Quaternery(100.0);
    pub const QUATERNARY_200: ThemeColor = ThemeColor::Quaternery(200.0);
    pub const QUATERNARY_300: ThemeColor = ThemeColor::Quaternery(300.0);
    pub const QUATERNARY_400: ThemeColor = ThemeColor::Quaternery(400.0);
    pub const QUATERNARY_500: ThemeColor = ThemeColor::Quaternery(500.0);
    pub const QUATERNARY_600: ThemeColor = ThemeColor::Quaternery(600.0);
    pub const QUATERNARY_700: ThemeColor = ThemeColor::Quaternery(700.0);
    pub const QUATERNARY_800: ThemeColor = ThemeColor::Quaternery(800.0);
    pub const QUATERNARY_900: ThemeColor = ThemeColor::Quaternery(900.0);

    pub const INFO_50: ThemeColor = ThemeColor::Info(50.0);
    pub const INFO_100: ThemeColor = ThemeColor::Info(100.0);
    pub const INFO_200: ThemeColor = ThemeColor::Info(200.0);
    pub const INFO_300: ThemeColor = ThemeColor::Info(300.0);
    pub const INFO_400: ThemeColor = ThemeColor::Info(400.0);
    pub const INFO_500: ThemeColor = ThemeColor::Info(500.0);
    pub const INFO_600: ThemeColor = ThemeColor::Info(600.0);
    pub const INFO_700: ThemeColor = ThemeColor::Info(700.0);
    pub const INFO_800: ThemeColor = ThemeColor::Info(800.0);
    pub const INFO_900: ThemeColor = ThemeColor::Info(900.0);

    pub const WARNING_50: ThemeColor = ThemeColor::Warning(50.0);
    pub const WARNING_100: ThemeColor = ThemeColor::Warning(100.0);
    pub const WARNING_200: ThemeColor = ThemeColor::Warning(200.0);
    pub const WARNING_300: ThemeColor = ThemeColor::Warning(300.0);
    pub const WARNING_400: ThemeColor = ThemeColor::Warning(400.0);
    pub const WARNING_500: ThemeColor = ThemeColor::Warning(500.0);
    pub const WARNING_600: ThemeColor = ThemeColor::Warning(600.0);
    pub const WARNING_700: ThemeColor = ThemeColor::Warning(700.0);
    pub const WARNING_800: ThemeColor = ThemeColor::Warning(800.0);
    pub const WARNING_900: ThemeColor = ThemeColor::Warning(900.0);

    pub const SUCCESS_50: ThemeColor = ThemeColor::Success(50.0);
    pub const SUCCESS_100: ThemeColor = ThemeColor::Success(100.0);
    pub const SUCCESS_200: ThemeColor = ThemeColor::Success(200.0);
    pub const SUCCESS_300: ThemeColor = ThemeColor::Success(300.0);
    pub const SUCCESS_400: ThemeColor = ThemeColor::Success(400.0);
    pub const SUCCESS_500: ThemeColor = ThemeColor::Success(500.0);
    pub const SUCCESS_600: ThemeColor = ThemeColor::Success(600.0);
    pub const SUCCESS_700: ThemeColor = ThemeColor::Success(700.0);
    pub const SUCCESS_800: ThemeColor = ThemeColor::Success(800.0);
    pub const SUCCESS_900: ThemeColor = ThemeColor::Success(900.0);

    pub const ERROR_50: ThemeColor = ThemeColor::Error(50.0);
    pub const ERROR_100: ThemeColor = ThemeColor::Error(100.0);
    pub const ERROR_200: ThemeColor = ThemeColor::Error(200.0);
    pub const ERROR_300: ThemeColor = ThemeColor::Error(300.0);
    pub const ERROR_400: ThemeColor = ThemeColor::Error(400.0);
    pub const ERROR_500: ThemeColor = ThemeColor::Error(500.0);
    pub const ERROR_600: ThemeColor = ThemeColor::Error(600.0);
    pub const ERROR_700: ThemeColor = ThemeColor::Error(700.0);
    pub const ERROR_800: ThemeColor = ThemeColor::Error(800.0);
    pub const ERROR_900: ThemeColor = ThemeColor::Error(900.0);

    pub const SURFACE_50: ThemeColor = ThemeColor::Surface(50.0);
    pub const SURFACE_100: ThemeColor = ThemeColor::Surface(100.0);
    pub const SURFACE_200: ThemeColor = ThemeColor::Surface(200.0);
    pub const SURFACE_300: ThemeColor = ThemeColor::Surface(300.0);
    pub const SURFACE_400: ThemeColor = ThemeColor::Surface(400.0);
    pub const SURFACE_500: ThemeColor = ThemeColor::Surface(500.0);
    pub const SURFACE_600: ThemeColor = ThemeColor::Surface(600.0);
    pub const SURFACE_700: ThemeColor = ThemeColor::Surface(700.0);
    pub const SURFACE_800: ThemeColor = ThemeColor::Surface(800.0);
    pub const SURFACE_900: ThemeColor = ThemeColor::Surface(900.0);

    pub const NEUTRAL_50: ThemeColor = ThemeColor::Neutral(50.0);
    pub const NEUTRAL_100: ThemeColor = ThemeColor::Neutral(100.0);
    pub const NEUTRAL_200: ThemeColor = ThemeColor::Neutral(200.0);
    pub const NEUTRAL_300: ThemeColor = ThemeColor::Neutral(300.0);
    pub const NEUTRAL_400: ThemeColor = ThemeColor::Neutral(400.0);
    pub const NEUTRAL_500: ThemeColor = ThemeColor::Neutral(500.0);
    pub const NEUTRAL_600: ThemeColor = ThemeColor::Neutral(600.0);
    pub const NEUTRAL_700: ThemeColor = ThemeColor::Neutral(700.0);
    pub const NEUTRAL_800: ThemeColor = ThemeColor::Neutral(800.0);
    pub const NEUTRAL_900: ThemeColor = ThemeColor::Neutral(900.0);
}
impl Default for ThemeColor {
    fn default() -> Self {
        ThemeColor::SURFACE_500
    }
}