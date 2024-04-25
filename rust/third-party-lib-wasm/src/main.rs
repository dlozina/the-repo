pub mod wasm;

fn main() {
    unsafe {
        let result = wasm::add(2, 2);
        println!("2 + 2 = {}", result);
    }
}


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
