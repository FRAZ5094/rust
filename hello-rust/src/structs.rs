//traditional struct
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

//tuple struct
struct Colourtuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    //structs used to make custom data types

    let mut c = Colour {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    //println!("Colour: {} {} {}", c.red, c.green, c.blue);

    let mut c_tuple = Colourtuple(255, 0, 0);

    c_tuple.0 = 0;

    //println!("Colour: {} {} {}", c_tuple.0, c_tuple.1, c_tuple.2);

    let mut p = Person::new("Mary", "Doe");
    println!("Person {}", p.full_name());

    p.set_last_name("Williams");

    println!("Person {}", p.full_name());

    println!("Person Tuple {:?}", p.to_tuple());
}
