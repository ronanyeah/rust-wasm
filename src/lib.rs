use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn string_to_bytes(val: &str) -> Vec<u8> {
    val.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = string_to_bytes("🦀");
        assert_eq!(result.len(), 4);
    }
}
