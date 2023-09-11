use std::borrow::Borrow;

use bevy::prelude::*;

use crate::{LunexError, UiTree, UiBranch, Data, LayoutPackage, Position};
use crate::{is_numerical_id, split_last};

// ===========================================================
// === WIDGET DEFINITION ===

#[derive(Component, Default, Clone, Debug, PartialEq)]
pub struct Widget {
    path: String,
    name: String,
}
impl Widget {
    /*
    pub fn fetch_layout<'a> (&'a self, system: &'a  UITree, key: &str) -> Result<&PositionLayout, String> {
        match self.fetch(system, key){
            Ok (branch) => Ok(branch.container_get().position_layout_get()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_layout_mut<'a> (&'a self, system: &'a mut UITree, key: &str) -> Result<&mut PositionLayout, String> {
        match self.fetch_mut(system, key){
            Ok (branch) => Ok(branch.container_get_mut().position_layout_get_mut()),
            Err (message) => Err(message),
        }
    }
    */
    //add is cursor_within + depth

    // ===========================================================
    // === FETCHING ===

    /// # Fetch
    /// This function will try to return &[`UiBranch`], located inside [`UiTree`] based on the widgets path.
    ///
    /// If you want to interact with the UiTree, you use this to get a borrow.
    ///
    /// # Examples
    /// ```
    /// let tree = UiTree::new();
    ///
    /// //This is only a pointer
    /// let menu_pointer = Widget::create(&mut tree, "Menu", PositionLayout::Default)?;
    /// let button_pointer = Widget::create(&mut tree, &menu_pointer.end("Button"), PositionLayout::Default)?;
    ///
    /// //This is the actual 'widget' that you can manipulate
    /// let menu: &UiBranch = menu_pointer.fetch(&tree)?;
    /// let button: &UiBranch = button_pointer.fetch(&tree)?;
    ///
    /// ```
    pub fn fetch<'a>(&'a self, tree: &'a UiTree) -> Result<&UiBranch, LunexError> {
        match tree.main_branch().borrow_linked_checked(&self.path) {
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
    /// let tree = UiTree::new();
    ///
    /// //This is only a pointer
    /// let menu_pointer = Widget::create(&mut tree, "Menu", PositionLayout::Default)?;
    /// let button_pointer = Widget::create(&mut tree, &menu_pointer.end("Button"), PositionLayout::Default)?;
    ///
    /// //This is the actual 'widget' that you can manipulate
    /// let menu: &UiBranch = menu_pointer.fetch_ext(&tree, "")?;   //Leave blank for self
    /// let button: &UiBranch = menu_pointer.fetch_ext(&tree, "Button")?; //You can locate sub-widgets
    ///
    /// ```
    pub fn fetch_ext<'a>(&'a self, tree: &'a UiTree, path: &str) -> Result<&UiBranch, LunexError> {
        let mut extra_path = String::from(&self.path);
        if !path.is_empty() {
            extra_path += "/";
            extra_path += path;
        }
        match tree.main_branch().borrow_linked_checked(&extra_path) {
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
    /// If you want to interact with the UITree, you use this to get a borrow.
    ///
    /// # Examples
    /// ```
    /// let tree = UiTree::new();
    ///
    /// //This is only a pointer
    /// let menu_pointer = Widget::create(&mut tree, "Menu", PositionLayout::Default)?;
    /// let button_pointer = Widget::create(&mut tree, &menu_pointer.end("Button"), PositionLayout::Default)?;
    ///
    /// //This is the actual 'widget' that you can manipulate
    /// let menu: &mut UiBranch = menu_pointer.fetch_mut(&mut tree)?;
    /// let button: &mut UiBranch = button_pointer.fetch_mut(&mut tree)?;
    ///
    /// ```
    pub fn fetch_mut<'a>(
        &'a self,
        tree: &'a mut UiTree,
    ) -> Result<&mut UiBranch, LunexError> {
        match tree.main_branch_mut().borrow_linked_checked_mut(&self.path) {
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
    /// If you want to interact with the UITree, you use this to get a borrow.
    ///
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead. 
    ///
    /// # Examples
    /// ```
    /// let tree = UiTree::new();
    ///
    /// //This is only a pointer
    /// let menu_pointer = Widget::create(&mut tree, "Menu", PositionLayout::Default)?;
    /// let button_pointer = Widget::create(&mut tree, &menu_pointer.end("Button"), PositionLayout::Default)?;
    ///
    /// //This is the actual 'widget' that you can manipulate
    /// let menu: &mut UiBranch = menu_pointer.fetch_mut_ext(&mut tree, "")?;   //Leave blank for self
    /// let button: &mut UiBranch = menu_pointer.fetch_mut_ext(&mut tree, "Button")?; //You can locate sub-widgets
    ///
    /// ```
    pub fn fetch_mut_ext<'a>(
        &'a self,
        tree: &'a mut UiTree,
        path: &str,
    ) -> Result<&mut UiBranch, LunexError> {
        let mut extra_path = String::from(&self.path);
        if !path.is_empty() {
            extra_path += "/";
            extra_path += path;
        }
        match tree.main_branch_mut().borrow_linked_checked_mut(&extra_path) {
            Ok(branch) => Ok(branch),
            Err(cause) => Err(LunexError::FetchError {
                path: extra_path,
                cause: Box::new(cause),
            }),
        }
    }

    /// # Fetch Position
    /// This function will try to return &[`Position`].
    ///
    /// This struct is output of the calculated layout data.
    pub fn fetch_position<'a>(
        &'a self,
        tree: &'a UiTree,
    ) -> Result<&Position, LunexError> {
        match self.fetch(tree) {
            Ok(branch) => Ok(branch.container_get().position_get()),
            Err(e) => Err(e),
        }
    }

    /// # Fetch Position Extended
    /// This function will try to return &[`Position`].
    ///
    /// This struct is output of the calculated layout data.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn fetch_position_ext<'a>(
        &'a self,
        tree: &'a UiTree,
        path: &str,
    ) -> Result<&Position, LunexError> {
        match self.fetch_ext(tree, path) {
            Ok(branch) => Ok(branch.container_get().position_get()),
            Err(e) => Err(e),
        }
    }

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


    // ===========================================================
    // === CREATION ===

    /// # New
    /// This function by itself does NOTHING except creating a pointer from provided path.
    /// It does NOT SYNCHRONIZE with any hierarchy and doesn't change anything.
    ///
    /// If you want to actually create new widget, use ``Widget::Create``
    ///
    /// This is just a pointer to call on more advanced methods later.
    ///
    /// # Examples
    /// ```
    /// let button = Widget::new("Button");
    /// let setting_button = Widget::new("Settings/Button");
    /// ```
    pub fn new(path: &str) -> Widget {
        Widget {
            path: path.to_string(),
            name: split_last(path, "/").1,
        }
    }

    /// # Create
    /// This function is the one you create new widgets with. It creates a [`Widget`] on the path specified inside the hierarchy.
    ///
    /// Paths:
    /// * ``Menu`` -> Create widget ``Menu`` in ``#ROOT``
    /// * ``Menu/Category`` -> Create widget ``Category`` in ``Menu``
    /// * ``Menu/Category/Button`` -> Create widget ``Button`` in ``Category`` (Located at ``Menu/Category``)
    ///
    /// # Example
    /// ```
    /// let menu_pointer = Widget::create(&mut system, "Menu", PositionLayout::Default);
    /// let button_pointer = Widget::create(&mut system, "Menu/Button", PositionLayout::Default); //Not recommended way of defining path
    /// ```
    ///
    /// The string after the last '/' is the name of the [`Widget`] and the rest is path to the parent [`Widget`].
    /// Note that manually setting path is bad practice, it is recommended to use `.end()` method on [`Widget`]
    ///
    /// # Nameless widgets
    /// You are also able to create 'nameless' widgets. They are good for non-important widgets like
    /// one time use or used only for layout purposes. (Don't require interactivity).
    /// Leave an empty string for name to create nameless [`Widget`]. Their name will be generated.
    ///
    /// ```
    /// let menu_pointer = Widget::create(&mut system, "", PositionLayout::Default);
    /// let button_pointer = Widget::create(&mut system, &menu_pointer.end(""), PositionLayout::Default);
    /// ```
    /// In this case the path of ``button_pointer`` is `` #0/#0 `` (The number stands for an order they were created in)
    ///
    pub fn create(tree: &mut UiTree, path: impl Borrow<str>, position: impl Into<LayoutPackage>) -> Result<Widget, LunexError> {
        let path = path.borrow();

        let str_list: Vec<&str> = path.split('/').collect();
        let str_list_len = str_list.len();

        let mut parent_path = String::new();
        let name = String::from(str_list[str_list_len - 1]);

        let mut n = if str_list_len != 0 {
            str_list_len - 1
        } else {
            0
        };

        //# This will check for skippable paths (Menu/#0/#0/Display -> Menu/Display)
        let mut absolute = String::new(); // => #0/#0
        if !name.is_empty() && !is_numerical_id(&name) && str_list_len > 1 {
            let mut i = str_list_len - 2;
            while is_numerical_id(str_list[i]) && i > 0 {
                absolute = format!("{}/{}", str_list[i], absolute);
                i -= 1;
            }
            if absolute.contains("/") {
                absolute = absolute[..absolute.len() - 1].to_string()
            }
            n = i + 1;
        }

        //# Collect the remaining iterator into path
        for ii in 0..n {
            if ii != 0 {
                parent_path += "/"
            }
            parent_path += str_list[ii];
        }

        //# Create branch in ROOT
        if parent_path.is_empty() {
            match tree.main_branch_mut().create_linked(&name, position.into()) {
                Ok(absolute_key) => {
                    let widget = if name.is_empty() {
                        Widget::new(&absolute_key)
                    } else {
                        Widget::new(path)
                    };
                    Ok(widget)
                }
                Err(e) => Err(e),
            }

        //# Create branch in branch
        } else {
            match Widget::new(&parent_path).fetch_mut(tree) {
                Ok(parent_branch) => {
                    if !absolute.is_empty() == true {
                        //println!("Name: {}, Path: {}, PPath: {}, obs: {}/###", name, path, parent_path, absolute);

                        //# Create branch with skip
                        let absolute_key = match parent_branch.borrow_linked_checked_mut(&absolute)
                        {
                            Ok(nameless_branch) => {
                                match nameless_branch.create_linked(&name, position.into()) {
                                    Ok(absolute_key) => absolute_key,
                                    Err(message) => return Err(message),
                                }
                            }
                            Err(message) => return Err(message),
                        };
                        match parent_branch
                            .shortcut_add(name, format!("{}/{}", absolute, absolute_key))
                        {
                            Ok(..) => Ok(Widget::new(&format!(
                                "{}/{}",
                                parent_path, absolute_key
                            ))),
                            Err(message) => Err(message),
                        }
                    } else {
                        //# Create direct branch without skipping
                        match parent_branch.create_linked(&name, position.into()) {
                            Ok(absolute_key) => {
                                if name.is_empty() {
                                    Ok(Widget::new(&format!(
                                        "{}/{}",
                                        parent_path, absolute_key
                                    )))
                                } else {
                                    Ok(Widget::new(path))
                                }
                            }
                            Err(message) => Err(message),
                        }
                    }
                }
                Err(e) => Err(e),
            }
        }
    }


    // ===========================================================
    // === REMOVAL ===

    /// # Drop Extended
    /// This function will try to drop a sub-branch
    pub fn drop_ext (&self, tree: &mut UiTree, path : &str) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                branch.drop_linked_checked(path)
            }
            Err(e) => Err(e),
        }
    }

    /// # Remove
    /// This function will try to drop a sub-branch and remove it's shortcut
    pub fn remove (&self, tree: &mut UiTree, key : &str) -> Result<(), LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                branch.remove_simple_checked(key)
            }
            Err(e) => Err(e),
        }
    }

    /// # Remove Invalid
    /// This function will try to remove all shortcuts that are not valid
    pub fn remove_invalid (&self, tree: &mut UiTree) -> Result<usize, LunexError> {
        match self.fetch_mut(tree) {
            Ok(branch) => {
                Ok(branch.remove_invalid())
            }
            Err(e) => Err(e),
        }
    }


    // ===========================================================
    // === PATH SYSTEM ===

    /// # Add
    /// This method is used to create dynamic path to widgets.
    /// 
    /// It adds the widget's name to the path. Used in combination of `.add_str()` and `.end()`.
    /// # Examples
    /// ```
    /// let menu = Widget::create(&mut tree, "Menu", PositionLayout::Default)?;
    /// let category = Widget::create(&mut tree, &menu.end("Category"), PositionLayout::Default)?;
    ///
    /// let path = menu.add(&category).end("Button");
    /// assert_eq!("Menu/Category/Button", path);
    /// ```
    pub fn add(&self, w: impl Borrow<Widget>) -> Widget {
        Widget::new(&format!("{}/{}", self.path, w.borrow().name))
    }
    /// # Str
    /// This method is used to create dynamic path to widgets.
    /// 
    /// It adds the string to the path. Used in combination of `.add()` and `.end()`.
    /// # Examples
    /// ```
    /// let menu = Widget::create(&mut tree, "Menu", PositionLayout::Default)?;
    ///
    /// let path = menu.str("Category").end("Button");
    /// assert_eq!("Menu/Category/Button", path);
    /// ```
    pub fn str(&self, s: impl Borrow<str>) -> Widget {
        Widget::new(&format!("{}/{}", self.path, s.borrow()))
    }
    /// # End
    /// This method is used to create dynamic path to widgets.
    /// 
    /// It adds the string to the path and returns finished string. Used in combination of `.add()` and `.str()`.
    /// # Examples
    /// ```
    /// let menu = Widget::create(&mut tree, "Menu", PositionLayout::Default)?;
    /// let category = Widget::create(&mut tree, &menu.end("Category"), PositionLayout::Default)?;
    ///
    /// let path = category.end("Button");
    /// assert_eq!("Menu/Category/Button", path);
    /// ```
    pub fn end(&self, s: impl Borrow<str>) -> String {
        format!("{}/{}", self.path, s.borrow())
    }


    // ===========================================================
    // === FUNCTIONAL METHODS ===

    /// # Contains Position
    /// This function will fetch the widget and will do a lookup to check if the point provided is within the calculated position of the container.
    pub fn contains_position(&self, tree: &UiTree, point: &Vec2) -> Result<bool, LunexError> {
        match self.fetch(&tree) {
            Ok(branch) => {
                let position = branch.container_get().position_get();
                Ok(
                    (point.x > position.point_1.x && point.x < position.point_2.x)
                        && (point.y > position.point_1.y && point.y < position.point_2.y),
                )
            }
            Err(e) => Err(e),
        }
    }

    /// # Contains Position
    /// This function will fetch the widget and will do a lookup to check if the point provided is within the calculated position of the container.
    /// 
    /// In this extended function you can also specify path to sub-widgets which will be used as target instead.
    pub fn contains_position_ext(&self, tree: &UiTree, path: &str, point: &Vec2) -> Result<bool, LunexError> {
        match self.fetch_ext(&tree, path) {
            Ok(branch) => {
                let position = branch.container_get().position_get();
                Ok(
                    (point.x > position.point_1.x && point.x < position.point_2.x)
                        && (point.y > position.point_1.y && point.y < position.point_2.y),
                )
            }
            Err(e) => Err(e),
        }
    }
}

