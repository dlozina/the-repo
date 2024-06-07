fn main() {

    // Example of match guard
    let num = Some(6);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even!"),
        Some(x) => println!("The number {x} is odd!"),
        None => ()
    }

    // Example of @Bindings
    enum Message {
        Hello { id: i32}
    }

    let msg = Message::Hello {id: 4};

    match msg {
        Message::Hello { id: id_variable @ 3..=7} => println!("Found an id in rage: {id_variable}"),
        Message::Hello { id: 10..=12}=> println!("Found an id in another range"),
        Message::Hello { id } => println!("Some other id: {id}")
    }

}
