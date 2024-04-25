pub mod wasm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let result = wasm::add(2, 2);
            assert_eq!(result, 4);
        }
    }
}
