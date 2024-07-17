use crate::lang::types::Literal;

/// Gets f32 vector from the command parameter literals
pub fn get_f32_param_values(params: Vec<Literal>) -> Result<Vec<f32>, String> {
    let mut result = vec![];

    for (indx, param) in params.iter().enumerate() {
        match param {
            Literal::Number(num) => { result.push(num.clone()); }
            _ => {
                return Err(format!(
                    "Unexpected value, expecting number at position: {}",
                    indx,
                ));
            }
        }
    }

    Ok(result)
}

/// Gets string vector from the command parameter literals
pub fn get_string_param_values(params: Vec<Literal>) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = vec![];

    for (indx, param) in params.iter().enumerate() {
        match param {
            Literal::String(s) => { result.push(s.to_owned()) },
            _ => {
                return Err(format!(
                    "Unexpected value, expecting number at position: {}",
                    indx,
                ));
            }
        }
    }

    Ok(result)
}

pub fn get_string_param_value(params: &Vec<Literal>, indx: usize) -> Result<String, String> {
    match params.get(indx) {
        Some(param) => {
            match param {
                Literal::String(s) => Ok(s.to_owned()),
                _ => Err(String::from("String not found where expected.")),
            }
        }

        None => Err(String::from("String not found where expected.")),
    }
}

pub fn get_f32_param_value(params: &Vec<Literal>, indx: usize) -> Result<f32, String> {
    match params.get(indx) {
        Some(param) => {
            match param {
                Literal::Number(s) => Ok(s.to_owned()),
                _ => Err(String::from("String not found where expected.")),
            }
        }

        None => Err(String::from("String not found where expected.")),
    }
}

