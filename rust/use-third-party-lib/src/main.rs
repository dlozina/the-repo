use libloading::{Library, Symbol};

type AddFunc = fn(usize, usize) -> usize;

type HelloFunc = fn() -> &'static str;

type HelloUserFunc = fn(name: &str, surname: &str) -> String;

fn main() {
    const LIBRARY_PATH: &str = "/Users/dlozina/workspace/my/the-repo/rust/use-third-party-lib/src/lib/libthird_party_lib.dylib";

    unsafe {
        let lib = Library::new(LIBRARY_PATH).unwrap();

        let func_add: Symbol<AddFunc> = lib.get(b"add").unwrap();
        let function_add_result = func_add(1, 2);
        println!("1 + 2 = {}", function_add_result);

        let func_hello_world: Symbol<HelloFunc> = lib.get(b"hello_world").unwrap();
        let function_hello_result = func_hello_world();
        println!("{}", function_hello_result);

        let func_hello_user: Symbol<HelloUserFunc> = lib.get(b"hello_user").unwrap();
        let function_hello_user_result = func_hello_user("Dino", "Lozina");
        println!("{}", function_hello_user_result);
    }
}
