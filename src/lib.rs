pub mod common;
pub mod ctos;
pub mod stoc;
pub mod utils;


#[cfg(test)]
mod tests {
    pub(crate) fn cast_hex_stream_to_c_array(stream: &str) -> Vec<u8> {
        let bytes: Vec<u8>  = (0..stream.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&stream[i..i + 2], 16))
            .collect::<Result<Vec<_>, _>>().unwrap();
        bytes
    }
}