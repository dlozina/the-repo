mod functions;

// External calls
#[no_mangle]
pub extern "C" fn hello_world() -> &'static str {
    functions::great()
}

#[no_mangle]
pub extern "C" fn hello_user(name: &str, surname: &str) -> String {
    functions::great_user(name, surname)
}

#[no_mangle]
pub extern "C" fn company_greeting(name: &str) -> String {
    functions::great_company(name)
}

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    add(left, right)
}

// Complex types with C ABI interface
// https://stackoverflow.com/questions/75183630/does-rust-allow-distributing-closed-source-libraries

