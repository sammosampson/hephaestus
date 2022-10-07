use std::{
    *,
    time::*,
    num::*
};

pub fn string(value: &str) -> String {
    value.to_string()
}

pub fn is_int_string(to_check: &str) -> bool{
    parse_signed_64_from_string(to_check).is_ok() || parse_unsigned_64_from_string(to_check).is_ok()
}

pub fn is_float_string(to_check: &str) -> bool{
    parse_float_64_from_string(to_check).is_ok()
}

pub fn parse_unsigned_64_from_string(from: &str) -> Result<u64, ParseIntError> {
    from.parse::<u64>()
}

pub fn parse_signed_64_from_string(from: &str) -> Result<i64, ParseIntError> {
    from.parse::<i64>()
}

pub fn parse_unsigned_32_from_string(from: &str) -> Result<u32, ParseIntError> {
    from.parse::<u32>()
}

pub fn parse_signed_32_from_string(from: &str) -> Result<i32, ParseIntError> {
    from.parse::<i32>()
}

pub fn parse_unsigned_16_from_string(from: &str) -> Result<u16, ParseIntError> {
    from.parse::<u16>()
}

pub fn parse_signed_16_from_string(from: &str) -> Result<i16, ParseIntError> {
    from.parse::<i16>()
}

pub fn parse_unsigned_8_from_string(from: &str) -> Result<u8, ParseIntError> {
    from.parse::<u8>()
}

pub fn parse_signed_8_from_string(from: &str) -> Result<i8, ParseIntError> {
    from.parse::<i8>()
}

pub fn parse_float_32_from_string(from: &str) -> Result<f32, ParseFloatError> {
    from.parse::<f32>()
}

pub fn parse_float_64_from_string(from: &str) -> Result<f64, ParseFloatError> {
    from.parse::<f64>()
}

pub fn add_negative_if_needed(alphanumeric_string: &str, is_negative: bool) -> String {
    if is_negative {
        return format!("-{}", alphanumeric_string);
    }
    string(alphanumeric_string)
}

pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { 
        slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
}

pub fn get_8_padded_u8_array_from_string(from: &str) -> [u8; 8] {
    assert!(from.len() <= 8);
    
    let mut to = [0; 8];
    to[..from.len()].copy_from_slice(&from.as_bytes());
    to
}

pub fn get_truncated_18_padded_u8_array_from_string(from: &str) -> [u8; 18] {
    let from_len = if from.len() < 18 { from.len() } else { 18 };
    
    let mut to = [0; 18];
    to[..from_len].copy_from_slice(&from[..from_len].as_bytes());
    to
}

pub fn string_to_bytes_zero_terminated(entry: &str) -> Vec<u8> {
    let mut new_string = string_to_bytes(entry);
    new_string.push(0x0);
    new_string
}

pub fn string_to_bytes(entry: &str) -> Vec<u8> {
    entry.as_bytes().into()
}

pub fn u32_to_bytes(entry: &u32) -> Vec<u8> {
    any_as_u8_slice(entry).into()
}

pub fn get_current_timestamp() -> u32 {
    // seconds since 1970-01-01 00:00:00 GMT
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => return n.as_secs() as u32,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}