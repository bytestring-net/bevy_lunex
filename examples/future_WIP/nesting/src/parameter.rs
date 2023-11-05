use std::borrow::Borrow;

#[derive(Debug)]
pub enum Parameter {
    Simple (String),                    //Arg0
    Defined ((String, String)),         //Arg1
    Complex ((String, Vec<String>)),    //Arg2 ... (,,,) (,,,,)
}
impl Parameter {
    pub fn parse(input: impl Borrow<str>) -> Vec<Parameter> {
        let mut vec = Vec::new();
        let text_vec = split_parameter(input);
        for text in text_vec {
            vec.push(parse_parameter(text));
        }
        vec
    }
}

fn split_parameter(input: impl Borrow<str>) -> Vec<String> {
    let input: &str = input.borrow();
    let mut result = Vec::new();
    let mut start = 0;
    let mut inside_brackets = false;

    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => inside_brackets = true,
            ')' => inside_brackets = false,
            ',' if !inside_brackets => {
                let x = input[start..idx].trim();
                if !x.is_empty() { result.push(x.into()) }
                start = idx + 1;
            }
            _ => (),
        }
    }

    let x = input[start..].trim();
    if !x.is_empty() { result.push(x.into()) }
    result
}

fn parse_parameter(input: String) -> Parameter {
    match input.split_once(':') {
        Some ((str1, str2)) => {
            if !str2.contains(',') {
                if str2.trim().is_empty() {
                    Parameter::Simple(trim(str1))
                } else {
                    Parameter::Defined((trim(str1), trim(str2)))
                }
            } else {
                let mut vec = Vec::new();
                let spl: Vec<&str> = str2.trim()[1..str2.len()-2].split_terminator(',').collect();
                for i in spl {
                    let x = i.trim();
                    if !x.is_empty() { vec.push(x.into()) }
                }
                Parameter::Complex((trim(str1), vec))
            }
        },
        None => Parameter::Simple(input),
    }
}

fn trim(input: &str) -> String {
    let mut x: &str = input.trim();
    if x.starts_with('(') { x = x[1..].into() }
    if x.ends_with(')') { x = x[..x.len()-1].into() }
    x.into()
}
