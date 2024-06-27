use encoding_rs::UTF_8;

pub fn cast_to_string(array: &[u16]) -> Option<String> {
    let mut str = array;
    if let Some(index) = array.iter().position(|&i| i == 0) {
        str = &str[0..index as usize];
    }
    else { return None }
    let body = unsafe { std::slice::from_raw_parts(str.as_ptr() as *const u8, str.len() * 2) };
    let (cow, _, had_errors) = encoding_rs::UTF_16LE.decode(&body);
    if had_errors { None }
    else { Some(cow.to_string()) }
}

pub fn cast_to_c_array(message: &str) -> Vec<u16> {
    let mut vector: Vec<u16> = message.encode_utf16().collect();
    vector.push(0);
    vector
}

pub fn cast_to_fix_length_array<const N: usize>(message: &str) -> [u16; N] {
    let mut data = [0u16; N];
    for (index, chr) in message.encode_utf16().enumerate() {
        data[index] = chr;
    }
    data
}

pub fn cast_to_utf8_string(array: &[u8]) -> Option<String> {
    let (cow, _, had_errors) = encoding_rs::UTF_8.decode(array);
    if had_errors { None }
    else { Some(cow.to_string()) }
}

/// Transform string to \[u16\] without length limit but a \0 in the end.
pub fn cast_to_utf8_c_array(message: &str) -> Vec<u8> {
    let mut vector: Vec<u8> = UTF_8.encode(message).0.to_vec();
    vector.push(0);
    vector
}

/// Transform string to \[u16\] with a fixed size. \
/// Differennt from ygopro, it will keeps 0 for residual part.
pub fn cast_to_utf8_fix_length_array<const N: usize>(message: &str) -> [u8; N] {
    let mut data = [0u8; N];
    for (index, chr) in UTF_8.encode(message).0.iter().enumerate() {
        data[index] = *chr;
    }
    data
}