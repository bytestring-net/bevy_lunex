#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::HashMap;

pub struct MString {}
impl MString {
    pub fn construct_from (template: &str, data: HashMap<String, String>) -> Result<String, String> {
        let mut level = 0;
        let mut name = String::new();
        let mut result = String::new();
        for character in template.chars() {
            if character == '}' {
                level -= 1;
                match data.get(&name){
                    None => return Err(String::from("Error while constructing MString - '") + &name + "' is not defined!"),
                    Some (value) => result += value,
                }
                name.clear();
            }
            if level == 1 {name.push(character);}
            if character == '{' {level += 1;}

            if level == 0 && character != '}'{
                result.push(character);
            }
        }
        if level != 0 {return Err(String::from("Error while constructing MString - wrong use of brackets!"));}
        Ok(result)
    }
    pub fn split_last (string: &str, delimiter: &str ) -> (String, String) {
        let str_list: Vec<&str> =  string.split(delimiter).collect();
        let mut output = String::new();
        let mut is_first = true;
        for x in str_list.iter().take(str_list.len() - 1){
            if !is_first {output += delimiter} else {is_first = false};
            output += x;
        }
        (output, String::from(str_list[str_list.len() - 1]))
    }
    pub fn subtract (string: &str, substring: &str) -> String {             // ABCDE - ABG = CDE
        let mut strip = string.chars();
        let mut substrip = substring.chars();
        for i in 0..strip.clone().count() {
            let char = strip.next();
            if char != substrip.next() {
                return String::from(char.unwrap_or('\0')) + strip.as_str();
            }
        }
        return String::from(strip.as_str());
    }
    pub fn subtract_void (string: &str, substring: &str) -> String {   // ABCDE - ABG = DE
        let mut strip = string.chars();
        let mut substrip = substring.chars();
        for i in 0..strip.clone().count() {
            if strip.next() != substrip.next() {
                return String::from(strip.as_str());
            }
        }
        return String::from(strip.as_str());
    }
}