pub fn run() {
    let x = 1;

    let y = 2.5;

    let z: i64 = 3123123;

    println!("{}", std::i32::MAX);
    println!("{}", std::u128::MAX);

    let my_array = [0..100];

    println!("{:#?}", (x, y, z));
    println!("{:?}", my_array[5]);
}
