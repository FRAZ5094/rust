pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers.push(5);

    println!("Vector: {:?}", numbers);

    numbers.pop();

    println!("Vector: {:?}", numbers);

    //loop though vector values

    for x in numbers.iter() {
        println!("Value: {}", x)
    }

    //loop though and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}
