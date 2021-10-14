pub fn run() {
    greeting("Hello", "Fraser");

    let get_sum = add(123, 2323);

    //closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; //can use n3 because in this scope
    println!("C sum {}", add_nums(3, 3));

    //https://youtu.be/zF34dRivLOw?t=4755
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    // -> i32 means this function returns i32
    n1 + n2 //no ; means that this is the return value
}
