// Implementation of the functions
pub fn great_company(name: &str) -> String {
    format!("You are working for {}", name)
}

pub fn great_user(name: &str, surname: &str) -> String {
    format!("Hello {} {}!", name, surname)
}

pub fn great() -> &'static str {
    "Hello, world!"
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let result = great();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_hello_user() {
        let result = great_user("Dino", "Lozina");
        assert_eq!(result, "Hello Dino Lozina!");
    }

    #[test]
    fn test_company_greeting() {
        let result = great_company("Macrometa");
        assert_eq!(result, "You are working for Macrometa");
    }

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}