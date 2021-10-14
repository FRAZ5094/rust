pub fn run() {
    //let hello = "Hello "; //immutable
    let mut hello = String::from("Hello "); //mutable

    hello.push('w'); //can only push char (single letter)

    hello.push_str("orld! some more text"); //can push strings

    //Capacity in bytes
    println!("Capacity {}", hello.capacity());

    println!("Length: {}", hello.len());

    println!("Is empty?: {}", hello.is_empty());

    println!("Contains world?: {}", hello.contains("World"));

    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through whitespace

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10); //string that can grow to length 10
    s.push('a');
    s.push('b');

    //assertion testing
    assert_eq!(2, s.len()); //will through an error if not equal

    println!("{}", hello);
}
