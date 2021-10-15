use std::env;

pub fn run() {
    //cli args

    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();

    println!("Args: {:?}", args);

    let name = "Fraser";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
