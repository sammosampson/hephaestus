use std::num::*;

pub fn string(value: &str) -> String {
    value.to_string()
}

type ParseUnsignedIntResult = Result<usize, ParseIntError>;
type ParseFloatResult = Result<f64, ParseFloatError>;

pub fn parse_number_from_string(from: &str) -> ParseUnsignedIntResult {
    from.parse::<usize>()
}

pub fn parse_float_from_string(from: &str) -> ParseFloatResult {
    from.parse::<f64>()
}


pub fn parse_signed_int_32_from_number(from: usize, is_negative: bool) -> i32 {
    let result = from as i32;

    if is_negative {
        return -result;
    }
    
    result
}

pub fn parse_signed_int_64_from_number(from: usize, is_negative: bool) -> i64 {
    let result = from as i64;

    if is_negative {
        return -result;
    }
    
    result
}


pub fn parse_float_32_from_number(from: f64, is_negative: bool) -> f32 {
    let result = from as f32;

    if is_negative {
        return -result;
    }
    
    result
}