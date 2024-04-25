pub mod wasm;

fn main() {
    unsafe {
        let result = wasm::add(2, 2);
        println!("2 + 2 = {}", result);
    }
}

// Note: This code needs to be compiled with the following command:
// cargo build --release --target wasm32-wasi
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
