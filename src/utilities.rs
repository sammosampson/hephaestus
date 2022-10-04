use std::{num::*, ops::Neg};

pub fn string(value: &str) -> String {
    value.to_string()
}

type ParseUnsignedIntResult = Result<usize, ParseIntError>;
type ParseFloat64Result = Result<f64, ParseFloatError>;
type ParseFloat32Result = Result<f32, ParseFloatError>;

pub fn parse_number_from_string(from: &str) -> ParseUnsignedIntResult {
    from.parse::<usize>()
}

pub fn parse_float_32_from_string(from: &str) -> ParseFloat32Result {
    from.parse::<f32>()
}

pub fn parse_float_64_from_string(from: &str) -> ParseFloat64Result {
    from.parse::<f64>()
}

pub fn try_parse_signed_number_from_number<TFrom, TTo: TryFrom<TFrom> + Neg<Output = TTo>>(from: TFrom, is_negative: bool) -> Option<TTo> {
    if let Ok(mut converted_number) = TTo::try_from(from) {
        if is_negative {
            converted_number = converted_number.neg();
        }
        return Some(converted_number)
    }
    None
}

pub fn try_parse_unsigned_number_from_number<TFrom, TTo: TryFrom<TFrom>>(from: TFrom) -> Option<TTo> {
    if let Ok(converted_number) = TTo::try_from(from) {
        return Some(converted_number);
    }
    None
}