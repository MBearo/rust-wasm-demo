use md5;
use wasm_bindgen::prelude::*; // 用于加载 Prelude（预导入）模块

#[wasm_bindgen]
pub fn string_to_md5(input: &str) -> String {
    let mut input_bytes = input.to_string().into_bytes();
    let encrypted_appendix = vec![0x2c, 0x2c, 0x31, 0x26];
    let appendix_bytes: Vec<u8> = encrypted_appendix.iter().map(|b| b ^ 0x55).collect();

    input_bytes.extend(appendix_bytes);
    let digest = md5::compute(&input_bytes);
    format!("{:x}", digest)
}