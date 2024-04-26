extern "C" {
    #[doc = " @Add Function"]
    pub fn add(left: usize, right: usize) -> usize;
}

extern "C" {
    #[doc = " @Hello World Function"]
    pub fn hello_world() -> &'static str;
}

extern "C" {
    #[doc = " @add function"]
    pub fn hello_user(name: &str, surname: &str) -> String;
}