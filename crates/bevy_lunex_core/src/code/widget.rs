use std::borrow::Borrow;

use bevy::prelude::*;

use crate::{LunexError, UiTree, UiBranch, LayoutPackage, UiT, UiD};

// ===========================================================
// === WIDGET ===

/// # Widget
/// A smart pointer for [`UiBranch`] located inside [`UiTree`].
/// 
/// This is a component and can be spawned as entity.
#[derive(Component, Default, Clone, Debug, PartialEq)]
pub struct Widget {
    path: String,
    name: String,
}
impl Widget {
    // ===========================================================
    // === CREATION ===

    /// # New
    /// This function by itself does NOTHING except creating a smart pointer from provided path.
    /// It does NOT SYNCHRONIZE with any tree and doesn't change anything.
    ///
    /// If you want to actually create new widget, use `Widget::Create`
    ///
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::Widget;
    /// let button = Widget::new("Button");
    /// let setting_button = Widget::new("Settings/Button");
    /// ```
    pub fn new(path: impl Borrow<str>) -> Widget {
        Widget {
            path: path.borrow().to_owned(),
            name: path.borrow().rsplit_once('/').unwrap_or(("", path.borrow())).1.to_owned(),
        }
    }
    
    /// # Create
    /// This function creates new [`UiBranch`] inside of provided [`UiTree`] and returns a smart pointer to it ([`Widget`]).
    /// 
    /// The location and structure is defined by paths:
    /// * `Menu` -> Create `Menu` in root directory
    /// * `Menu/Settings` -> Create `Settings` inside `Menu`
    /// * `Menu/Settings/Button` -> Create `Button` inside `Settings` which is inside `Menu`
    ///
    /// # Simple example
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let menu_button = Widget::create(&mut tree, "Menu/Button", LayoutPackage::default()).unwrap();
    /// ```
    ///
    /// # Recommended example
    /// It is best to use so called `dynamic paths` to define the path rather than hardcoding it.
    /// The method `Widget::end()` returns the current path at that point in time ending with custom string.
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let menu_button = Widget::create(&mut tree, menu.end("Button"), LayoutPackage::default()).unwrap();
    /// ```
    /// 
    pub fn create<T:Default>(tree: &mut UiTree<T>, path: impl Borrow<str>, position: impl Into<LayoutPackage>) -> Result<Widget, LunexError> {
        let (_path, _name) = path.borrow().rsplit_once('/').unwrap_or((".", path.borrow()));
        match tree.borrow_branch_mut(_path) {
            Ok(borrowed_branch) => match borrowed_branch.create_branch(_name, position) {
                Ok(_) => Ok(Widget::new(path)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// # End
    /// This method is used to create `dynamic paths` from [`Widget`].
    /// 
    /// It returns the path stored inside the struct and adds custom string to the end.
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let settings = Widget::create(&mut tree, menu.end("Settings"), LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, settings.end("Button"), LayoutPackage::default()).unwrap();
    ///
    /// assert_eq!("Menu/Settings/Button", settings.end("Button"));
    /// assert_eq!("Menu/Settings/Button/Foo", button.end("Foo"));
    /// ```
    pub fn end(&self, s: impl Borrow<str>) -> String {
        format!("{}/{}", self.path, s.borrow())
    }

    // ===========================================================
    // === REMOVAL ===

    /// # Drop Branch
    /// This function will try to fetch itself and then drop a specified sub-branch
    /// # Examples
    /// ```should_panic
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let settings = Widget::create(&mut tree, menu.end("Settings"), LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, settings.end("Button"), LayoutPackage::default()).unwrap();
    /// 
    /// settings.drop_branch(&mut tree,"Button").unwrap();
    /// button.fetch(&mut tree).unwrap(); // This will panic, because the fetching will return error type that will get unwrapped
    ///
    /// ```
    pub fn drop_branch<T:Default>(&self, tree: &mut UiTree<T>, path: impl Borrow<str>) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => branch.drop_branch(path),
            Err(e) => Err(e),
        }
    }

    // ===========================================================
    // === UTIL ===

    /// # Path
    /// This method returns the path stored inside the struct.
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let settings = Widget::create(&mut tree, menu.end("Settings"), LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, settings.end("Button"), LayoutPackage::default()).unwrap();
    ///
    /// assert_eq!("Menu/Settings", settings.path());
    /// assert_eq!("Menu/Settings/Button", button.path());
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }

    /// # Name
    /// This method returns the name stored inside the struct.
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let settings = Widget::create(&mut tree, menu.end("Settings"), LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, settings.end("Button"), LayoutPackage::default()).unwrap();
    ///
    /// assert_eq!("Settings", settings.name());
    /// assert_eq!("Button", button.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    // ===========================================================
    // === FETCHING ===

    /// # Fetch
    /// This function will try to return &[`UiBranch`], located inside [`UiTree`] based on the widgets path.
    ///
    /// If you want to interact with the UiTree, you use this to get a borrow.
    ///
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage, UiBranch};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, menu.end("Button"), LayoutPackage::default()).unwrap();
    ///
    /// let _menu: &UiBranch = menu.fetch(&tree).unwrap();
    /// let _button: &UiBranch = button.fetch(&tree).unwrap();
    /// ```
    pub fn fetch<'a, T:Default> (&'a self, tree: &'a UiTree<T>) -> Result<&UiBranch<T>, LunexError> {
        match tree.borrow_branch(self.path.borrow()) {
            Ok(branch) => Ok(branch),
            Err(cause) => Err(LunexError::FetchError {
                path: self.path.to_string(),
                cause: Box::new(cause),
            }),
        }
    }

    /// # Fetch Extended
    /// This function will try to return &[`UiBranch`], located inside [`UiTree`] based on the widgets path.
    ///
    /// If you want to interact with the UiTree, you use this to get a borrow.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead. 
    ///
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage, UiBranch};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, menu.end("Button"), LayoutPackage::default()).unwrap();
    ///
    /// let _button: &UiBranch = menu.fetch_ext(&tree, "Button").unwrap();
    /// let _button: &UiBranch = button.fetch_ext(&tree, "").unwrap();
    /// ```
    pub fn fetch_ext<'a, T:Default>(&'a self, tree: &'a UiTree<T>, path: impl Borrow<str>) -> Result<&UiBranch<T>, LunexError> {
        let mut extra_path = String::from(&self.path);
        if !path.borrow().is_empty() {
            extra_path += "/";
            extra_path += path.borrow();
        }
        match tree.borrow_branch(extra_path.borrow()) {
            Ok(branch) => Ok(branch),
            Err(cause) => Err(LunexError::FetchError {
                path: extra_path,
                cause: Box::new(cause),
            }),
        }
    }

    /// # Fetch Mutable
    /// This function will try to return &mut [`UiBranch`], located inside [`UiTree`] based on the widgets path.
    ///
    /// If you want to interact with the UiTree, you use this to get a borrow.
    ///
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage, UiBranch};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, menu.end("Button"), LayoutPackage::default()).unwrap();
    ///
    /// let _menu: &mut UiBranch = menu.fetch_mut(&mut tree).unwrap();
    /// let _button: &mut UiBranch = button.fetch_mut(&mut tree).unwrap();
    /// ```
    pub fn fetch_mut<'a, T:Default>(&'a self, tree: &'a mut UiTree<T>) -> Result<&mut UiBranch<T>, LunexError> {
        match tree.borrow_branch_mut(self.path.borrow()) {
            Ok(branch) => Ok(branch),
            Err(cause) => Err(LunexError::FetchError {
                path: self.path.to_string(),
                cause: Box::new(cause),
            }),
        }
    }

    /// # Fetch Mutable Extended
    /// This function will try to return &mut [`UiBranch`], located inside [`UiTree`] based on the widgets path.
    ///
    /// If you want to interact with the UiTree, you use this to get a borrow.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead. 
    ///
    /// # Examples
    /// ```
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, LayoutPackage, UiBranch};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", LayoutPackage::default()).unwrap();
    /// let button = Widget::create(&mut tree, menu.end("Button"), LayoutPackage::default()).unwrap();
    ///
    /// let _button: &mut UiBranch = menu.fetch_mut_ext(&mut tree, "Button").unwrap();
    /// let _button: &mut UiBranch = button.fetch_mut_ext(&mut tree, "").unwrap();
    /// ```
    pub fn fetch_mut_ext<'a, T:Default>(&'a self, tree: &'a mut UiTree<T>, path: impl Borrow<str>) -> Result<&mut UiBranch<T>, LunexError> {
        let mut extra_path = String::from(&self.path);
        if !path.borrow().is_empty() {
            extra_path += "/";
            extra_path += path.borrow();
        }
        match tree.borrow_branch_mut(extra_path.borrow()) {
            Ok(branch) => Ok(branch),
            Err(cause) => Err(LunexError::FetchError {
                path: extra_path,
                cause: Box::new(cause),
            }),
        }
    }

    // ===========================================================
    // === ACTION ===

    /// # Contains Position
    /// This function will fetch the widget and will do a lookup to check if the point provided is within the calculated position of the container.
    /// # Examples
    /// ```
    /// # use bevy::prelude::Vec2;
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, RelativeLayout, UiBranch};
    /// # let mut tree = UiTree::new("Ui");
    /// let button = Widget::create(&mut tree, "Button", RelativeLayout::new()).unwrap();
    ///
    /// tree.compute(Vec2::splat(0.0), 100.0, 100.0); //This is automatically called by default LunexUI plugin
    /// 
    /// assert_eq!(true, button.contains_position(&mut tree, Vec2::new(50.0, 50.0)).unwrap());
    /// ```
    pub fn contains_position<T:Default>(&self, tree: &UiTree<T>, point: impl Borrow<Vec2>) -> Result<bool, LunexError> {
        match self.fetch(&tree) {
            Ok(branch) => {
                let position = branch.get_container().get_position();
                let p = point.borrow();
                Ok(
                    (p.x > position.point_1.x && p.x < position.point_2.x)
                        && (p.y > position.point_1.y && p.y < position.point_2.y),
                )
            }
            Err(e) => Err(e),
        }
    }

    /// # Contains Position
    /// This function will fetch the widget and will do a lookup to check if the point provided is within the calculated position of the container.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    /// # Examples
    /// ```
    /// # use bevy::prelude::Vec2;
    /// # use bevy_lunex_core::{UiTree, UiT, Widget, RelativeLayout, UiBranch};
    /// # let mut tree = UiTree::new("Ui");
    /// let menu = Widget::create(&mut tree, "Menu", RelativeLayout::new()).unwrap();
    /// let button = Widget::create(&mut tree, menu.end("Button"), RelativeLayout::new()).unwrap();
    ///
    /// tree.compute(Vec2::splat(0.0), 100.0, 100.0); //This is automatically called by default LunexUI plugin
    /// 
    /// assert_eq!(true, menu.contains_position_ext(&mut tree, "Button", Vec2::new(50.0, 50.0)).unwrap());
    /// ```
    pub fn contains_position_ext<T:Default>(&self, tree: &UiTree<T>, path: impl Borrow<str>, point: impl Borrow<Vec2>) -> Result<bool, LunexError> {
        match self.fetch_ext(&tree, path) {
            Ok(branch) => {
                let position = branch.get_container().get_position();
                let p = point.borrow();
                Ok(
                    (p.x > position.point_1.x && p.x < position.point_2.x)
                        && (p.y > position.point_1.y && p.y < position.point_2.y),
                )
            }
            Err(e) => Err(e),
        }
    }

    /* 
    /// # Fetch Data
    /// This function will try to return &option with [`Data`].
    ///
    /// This struct holds any data you need to recursively share between widgets.
    pub fn fetch_data<'a>(
        &'a self,
        tree: &'a UiTree,
    ) -> Result<&Option<Data>, LunexError> {
        match self.fetch(tree) {
            Ok(branch) => Ok(branch.data_get()),
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Extended
    /// This function will try to return &option with [`Data`].
    ///
    /// This struct holds any data you need to recursively share between widgets.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_ext<'a>(
        &'a self,
        tree: &'a UiTree,
        path: &str,
    ) -> Result<&Option<Data>, LunexError> {
        match self.fetch_ext(tree, path) {
            Ok(branch) => Ok(branch.data_get()),
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Mutable
    /// This function will try to return &mut option with [`Data`].
    ///
    /// This struct holds any data you need to recursively share between widgets.
    pub fn fetch_data_mut<'a>(
        &'a self,
        tree: &'a mut UiTree,
    ) -> Result<&mut Option<Data>, LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => Ok(branch.data_get_mut()),
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Mutable Extended
    /// This function will try to return &mut option with [`Data`].
    ///
    /// This struct holds any data you need to recursively share between widgets.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_mut_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
    ) -> Result<&mut Option<Data>, LunexError> {
        match self.fetch_mut_ext(tree, path) {
            Ok(branch) => Ok(branch.data_get_mut()),
            Err(e) => Err(e),
        }
    }
    */
    /*
    /// # Fetch Data Set f32
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    pub fn fetch_data_set_f32<'a>(
        &'a self,
        tree: &'a mut UiTree,
        key: &str,
        value: f32,
    ) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.f32s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.f32s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set f32 Extended
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_set_f32_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
        key: &str,
        value: f32,
    ) -> Result<(), LunexError> {
        match self.fetch_mut_ext(tree, path) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.f32s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.f32s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set String
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    pub fn fetch_data_set_string<'a>(
        &'a self,
        tree: &'a mut UiTree,
        key: &str,
        value: String,
    ) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.strings.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.strings.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set String Extended
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_set_string_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
        key: &str,
        value: String,
    ) -> Result<(), LunexError> {
        match self.fetch_mut_ext(tree, path) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.strings.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.strings.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set bool
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    pub fn fetch_data_set_bool<'a>(
        &'a self,
        tree: &'a mut UiTree,
        key: &str,
        value: bool,
    ) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.bools.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.bools.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set bool Extended
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_set_bool_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
        key: &str,
        value: bool,
    ) -> Result<(), LunexError> {
        match self.fetch_mut_ext(tree, path) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.bools.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.bools.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set Vec2
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    pub fn fetch_data_set_vec2<'a>(
        &'a self,
        tree: &'a mut UiTree,
        key: &str,
        value: Vec2,
    ) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.vec2s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.vec2s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set Vec2 Extended
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_set_vec2_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
        key: &str,
        value: Vec2,
    ) -> Result<(), LunexError> {
        match self.fetch_mut_ext(tree, path) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.vec2s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.vec2s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set Vec3
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    pub fn fetch_data_set_vec3<'a>(
        &'a self,
        tree: &'a mut UiTree,
        key: &str,
        value: Vec3,
    ) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.vec3s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.vec3s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set Vec3 Extended
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_set_vec3_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
        key: &str,
        value: Vec3,
    ) -> Result<(), LunexError> {
        match self.fetch_mut_ext(tree, path) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.vec3s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.vec3s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set Vec4
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    pub fn fetch_data_set_vec4<'a>(
        &'a self,
        tree: &'a mut UiTree,
        key: &str,
        value: Vec4,
    ) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.vec4s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.vec4s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    /// # Fetch Data Set Vec4 Extended
    /// This function will try to fetch [`Data`] and create a value on the branch.
    ///
    /// If there is no [`Data`] it will create one and set the value anyway.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_data_set_vec4_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
        key: &str,
        value: Vec4,
    ) -> Result<(), LunexError> {
        match self.fetch_mut_ext(tree, path) {
            Ok(branch) => {
                let data_option = branch.data_get_mut();
                match data_option {
                    Some(data) => {
                        data.vec4s.insert(key.to_string(), value);
                        Ok(())
                    }
                    None => {
                        let mut data = Data::new();
                        data.vec4s.insert(key.to_string(), value);
                        *data_option = Some(data);
                        Ok(())
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
*/
}