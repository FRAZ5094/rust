pub fn run() {
    //with primate arrays
    let array1 = [1, 2, 3];
    let array2 = array1;

    println!("Values: {:?}", (array1, array2));

    //for non-primatives (eg vectors)

    let vector1 = vec![1, 2, 3];
    let vector2 = &vector1;

    println!("Values: {:?}", (&vector1, vector2))
}
