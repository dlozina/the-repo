#[no_mangle]
pub extern "C" fn hello_world() -> &'static str {
    "Hello, world!"
}

#[no_mangle]
pub extern "C" fn hello_user(name: &str, surname: &str) -> String {
    format!("Hello {} {}!", name, surname)
}

#[no_mangle]
pub extern "C" fn company_greeting(name: &str) -> String {
    format!("You are working for {}", name)
}

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

// Complex types with C ABI interface
// https://stackoverflow.com/questions/75183630/does-rust-allow-distributing-closed-source-libraries
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let result = hello_world();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_hello_user() {
        let result = hello_user("Dino", "Lozina");
        assert_eq!(result, "Hello Dino Lozina!");
    }

    #[test]
    fn test_company_greeting() {
        let result = company_greeting("Macrometa");
        assert_eq!(result, "You are working for Macrometa");
    }

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
