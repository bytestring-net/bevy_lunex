#![allow(dead_code)]

use std::borrow::Borrow;

use bevy::{prelude::*, utils::HashMap};
//use bevy_lunex::prelude::*;

macro_rules! attribute {
    ($x:ident) => {
        pub fn $x(mut self) -> Self {
            self.$x = true;
            self
        }
    };
}

macro_rules! ucolor_implement {
    ($x:ident) => {
        impl UColor for $x {
            fn primary(mut self) -> Self {
                self.color = ThemeColor::Primary;
                self
            }
            fn secondary(mut self) -> Self {
                self.color = ThemeColor::Secondary;
                self
            }
            fn accent(mut self) -> Self {
                self.color = ThemeColor::Accent;
                self
            }
            fn neutral(mut self) -> Self {
                self.color = ThemeColor::Neutral;
                self
            }
            fn base(mut self, b: u8) -> Self {
                self.color = ThemeColor::Base (b);
                self
            }
            fn base100(mut self) -> Self {
                self.color = ThemeColor::Base (100);
                self
            }
            fn info(mut self) -> Self {
                self.color = ThemeColor::Info;
                self
            }
            fn success(mut self) -> Self {
                self.color = ThemeColor::Success;
                self
            }
            fn warning(mut self) -> Self {
                self.color = ThemeColor::Warning;
                self
            }
            fn error(mut self) -> Self {
                self.color = ThemeColor::Error;
                self
            }
            fn color(mut self, c: Color) -> Self {
                self.color = ThemeColor::Custom (c);
                self
            }
        }
    };
}

// LAYOUT
/* 
pub fn layout(div: &mut Div) {
    // Spawn the layout here all divs and custom components etc.
    Div::Relative().into_layout(Button)


    Button::new(Div::)

}

// LOGIC

struct Div {

}

impl Div {
}

*/

pub enum Unit {
    Em (f32),
    Sec (f32),
    Abs (f32),
    Rel (f32),
}

#[derive(Clone, Debug, Default)]
pub struct Em (f32);
impl Into<Unit> for Em {
    fn into(self) -> Unit {
        Unit::Em(self.0)
    }
}
#[derive(Clone, Debug, Default)]
pub struct Sec(f32);
impl Into<Unit> for Sec {
    fn into(self) -> Unit {
        Unit::Sec(self.0)
    }
}
#[derive(Clone, Debug, Default)]
pub struct Abs(f32);
impl Into<Unit> for Abs {
    fn into(self) -> Unit {
        Unit::Abs(self.0)
    }
}
#[derive(Clone, Debug, Default)]
pub struct Rel(f32);
impl Into<Unit> for Rel {
    fn into(self) -> Unit {
        Unit::Rel(self.0)
    }
}


#[derive(Clone, Debug, Default)]
pub struct ThemeColorPack {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub neutral: Color,
    pub base: Color,
    pub info: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}
#[derive(Clone, Debug, Default)]
pub struct ThemeFontPack {
    pub primary: Handle<Font>,
    pub secondary: Handle<Font>,

    pub btn_lowercase: bool,
    pub btn_uppercase: bool,
}
#[derive(Clone, Debug, Default)]
pub struct ThemeVisualPack {
    pub box_rounding: Em,
    pub btn_rounding: Em,
    pub tab_rounding: Em,
    pub badge_rounding: Em,

    pub btn_border: Em,
    pub tab_border: Em,

    pub btn_outline: Em,
}
#[derive(Clone, Debug, Default)]
pub struct ThemeReactivityPack {
    pub btn_animation: Sec,
    pub btn_focused_scale: f32,
    pub animation_input: Sec,
}

#[derive(Clone, Debug, Default)]
pub struct Theme {
    pub color: ThemeColorPack,
    pub font: ThemeFontPack,
    pub visual: ThemeVisualPack,
    pub reactivity: ThemeReactivityPack,
}


#[derive(Resource)]
pub struct ThemeManager {
    current_theme: (String, Theme),
    themes: HashMap<String, Theme>,
}
impl ThemeManager {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("dracula".into(), Theme::default());

        let default = "dracula";
        ThemeManager {
            current_theme: (default.into(), map.get(default).unwrap().clone()),
            themes: map,
        }
    }
    pub fn get(&self) -> &Theme {
        &self.current_theme.1
    }
    pub fn get_mut(&mut self) -> &mut Theme {
        &mut self.current_theme.1
    }
    pub fn set_theme(&mut self, name: impl Borrow<str>) {
        if self.themes.contains_key(name.borrow()) {
            self.current_theme = (name.borrow().into(), self.themes.get(name.borrow()).unwrap().clone())
        }
    }
    pub fn add(&mut self, name: impl Borrow<str>, theme: Theme) -> Option<Theme> {
        self.themes.insert(name.borrow().into(), theme)
    }
}


#[derive(Clone, Debug, Default)]
pub enum ThemeColor {
    #[default]
    Primary,
    Secondary,
    Accent,
    Neutral,
    Base (u8),
    Info,
    Success,
    Warning,
    Error,
    Custom (Color),
}
impl ThemeColor {
    pub fn get(&self, theme: &Theme) -> Color {
        match self {
            Self::Primary => theme.color.primary,
            Self::Secondary => theme.color.secondary,
            Self::Accent => theme.color.accent,
            Self::Neutral => theme.color.neutral,
            Self::Base (b) => theme.color.base,
            Self::Info => theme.color.info,
            Self::Success => theme.color.success,
            Self::Warning => theme.color.warning,
            Self::Error => theme.color.error,
            Self::Custom (c) => *c,
        }
    }
}


fn theme_setup(mut commands: Commands) {
    let mut manager = ThemeManager::new();
    manager.add("cyberpunk", Theme {
        color: ThemeColorPack {
            primary: Color::YELLOW,
            ..default()
        },
        ..default()
    });
    manager.set_theme("cyberpunk");
    commands.insert_resource(manager);
}
pub struct ThemePlugin;
impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, theme_setup)
            .add_systems(Update, button_test);
    }
}


//EVERYTHIGN WILL BE DIV AND ALL ELEMENTS WILL BE TRAITS! ??? Opposite? Every element will be struct and Div will be trait????

/// When applied to element, it will use the color
pub trait UColor {
    fn primary(self) -> Self;
    fn secondary(self) -> Self;
    fn accent(self) -> Self;
    fn neutral(self) -> Self;
    fn base(self, b: u8) -> Self;
    fn base100(self) -> Self;
    fn info(self) -> Self;
    fn success(self) -> Self;
    fn warning(self) -> Self;
    fn error(self) -> Self;
    fn color(self, c: Color) -> Self;
}

#[derive(Component, Clone, Debug, Default)]
pub struct UButton {
    color: ThemeColor,

    outline: bool,
    active: bool,
    disabled: bool,
    no_animation: bool,
    large: bool,
    medium: bool,
    small: bool,
    tiny: bool,
    wide: bool,
    block: bool,
    circle: bool,
    square: bool,
}
impl UButton {
    pub fn new() -> Self {
        UButton::default()
    }
    attribute!(outline);
    attribute!(active);
    attribute!(disabled);
    attribute!(no_animation);
    attribute!(large);
    attribute!(medium);
    attribute!(small);
    attribute!(tiny);
    attribute!(wide);
    attribute!(block);
    attribute!(circle);
    attribute!(square);
}
ucolor_implement!(UButton);


fn button_test(theme: Res<ThemeManager>, query: Query<&UButton>) {
    //let manager = ThemeManager::new();
    //commands.spawn(manager);
    for btn in &query {
        println!("C: {:?}", btn.color.get(theme.get()));
    }
}