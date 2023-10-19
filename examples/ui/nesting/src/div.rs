use paste::paste;


use std::borrow::Borrow;

use bevy::prelude::Vec2;


use super::parameter::Parameter;

#[macro_export]
macro_rules! attribute {
    ($x:ident) => {
        pub fn $x(mut self) -> Self {
            self.$x = true;
            self
        }

        paste!{
            pub fn [<set_ $x>](&mut self) {
                self.$x = true;
            }
        }

    };
}

#[macro_export]
macro_rules! attribute_value {
    ($x:ident, $y:ident) => {
        pub fn $x(mut self, v: $y) -> Self {
            self.$x = v;
            self
        }

        paste!{
            pub fn [<set_ $x>](&mut self, v: $y) {
                self.$x = v;
            }
        }

    };
}

#[macro_export]
macro_rules! attribute_flipflop {
    ($x:ident, $( $y:ident ),*) => {
        pub fn $x(mut self) -> Self {
            self.$x = true;
            $(
                self.$y = false;
            )*
            self
        }
        paste!{
            pub fn [<set_ $x>](& mut self) {
                self.$x = true;
                $(
                    self.$y = false;
                )*
            }
        }
    };
}



pub enum Placement {
    /// Placement variant which defines position based on properties, Divs with this type add to the parent content_size
    Flow {
        padding: [f32; 4],  //Option to get padding from Theme
        /// Horizontal alignment, range from `-1.0` to `1.0` (from left to right)
        h_align: f32,
        /// Vertical alignment, range from `-1.0` to `1.0` (from top to bottom)
        v_align: f32,
    },
    /// Placement variant which defines position based on direct position values, Divs with this type do not contribute to the parent content_size
    Fixed
}


pub enum DivClass {
    //Provided Defaults
    Alert,
    Avatar,
    Badge,
    BottomNavbar,
    Breadcrumbs,
    Button,
    Card,
    Carousel,
    ChatBubble,
    Checkbox,
    Collapse,
    Countdown,
    Divider,
    Drawer,
    Dropdown,
    FileInput,
    Footer,
    Hero,
    Indicator,
    InputGroup,
    TextInput,
    Join,
    Keyboard,
    Link,
    Loading,
    Mask,
    Menu,
    BrowserMockup,
    CodeMockup,
    PhoneMockup,
    WindowMockup,
    Modal,
    Navbar,
    Pagination,
    Progress,
    RadialProgress,
    Radio,
    RangeSlider,
    Rating,
    Select,
    Stack,
    Stat,
    Steps,
    Swap,
    Tabs,
    Table,
    TextArea,
    Toast,
    Toggle,
    Tooltip,

    // Basic elements
    Relative, // -> Provides their behaivour
    Solid,
    Window,

    // My defaults
    List,
    Cell,

    Break,
    Trait (Box<dyn DivTrait>)
}


pub struct Div {

    pub class : DivClass,

    pub placement: Placement,
    pub content_size: Vec2,

    pub nested_div: Vec<Div>,
}
impl Div {
    pub fn compute_content (&mut self) {

        let mut content_size = Vec2::ZERO;
        let mut line_size = Vec2::ZERO;

        for div in &self.nested_div {
            match &div.class {
                DivClass::Break => {
                    if line_size.x > content_size.x { content_size.x = line_size.x }
                    content_size.y += line_size.y;
                    line_size.x = 0.0;
                    line_size.y = 0.0;

                },
                DivClass::Trait(custom_div) => {
                    let div_size = custom_div.size();   //NEEDS TO INCLUDE PADDING
                    line_size.x += div_size.x;
                    if div_size.y > line_size.y { line_size.y = div_size.y }
                },
                _ => unreachable!(),
            }
        }
        if line_size != Vec2::ZERO {
            if line_size.x > content_size.x { content_size.x = line_size.x }
            content_size.y += line_size.y;
            line_size.x = 0.0;
            line_size.y = 0.0;
        }
        self.content_size = content_size;
    }
}

pub trait DivTrait {
    fn size(&self) -> Vec2;
}



#[derive(Default)]
pub struct TestBox {
    large: bool,
    medium: bool,
    tiny: bool,
}
impl TestBox {
    pub fn new() -> Self {
        TestBox::default()
    }
    attribute_flipflop!(large, tiny, medium);
    attribute_flipflop!(medium, large, tiny);
    attribute_flipflop!(tiny, large, medium);
}

impl DivTrait for TestBox {
    fn size(&self) -> Vec2 {
        if self.tiny { return Vec2::new(20.0, 10.0) }
        if self.large { return Vec2::new(60.0, 20.0) }
        Vec2::new(40.0, 15.0)
    }
}
impl Into<Div> for TestBox {
    fn into(self) -> Div {
        Div { class: DivClass::Trait(Box::new(self)), placement: Placement::Fixed, content_size: Vec2::ZERO, nested_div: Vec::new() }
    }
}








pub struct ParseError;


#[derive(Clone, Debug, Default)]
pub struct Button {
    large: bool,
    medium: bool,
    tiny: bool,
    number: f32,
}
impl Button {
    attribute_value!(number, f32);
    attribute_flipflop!(large, medium, tiny);
    attribute_flipflop!(medium, tiny, large);
    attribute_flipflop!(tiny, large, medium);

    pub fn call(&mut self, parameter: Parameter) -> Result<(), ParseError> {
        match parameter {
            Parameter::Simple(name) => {
                match name.as_str() {
                    "large" => self.set_large(),
                    "medium" => self.set_medium(),
                    "tiny" => self.set_tiny(),
                    _ => return Err(ParseError),
                }
            }
            Parameter::Defined((name, param)) => {
                let param: f32 = param.parse().unwrap();
                match name.as_str() {
                    "number" => self.set_number(param),
                    _ => return Err(ParseError),
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub fn new() -> Self {
        Button::default()
    }
    pub fn from(argument: impl Borrow<str>) -> Self {
        let vec = Parameter::parse(argument);
        let mut btn = Button::new();
        for param in vec {
            let _ = btn.call(param);
        }
        btn
    }
}
