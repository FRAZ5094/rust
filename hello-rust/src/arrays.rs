pub fn run() {
    let numbers = [1, 2, 3, 4];

    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    //get slice
    let slice = &numbers[0..2];

    println!("{:?}", slice);
}
