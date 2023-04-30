#![allow(non_snake_case)]

use utils::read_line;

struct Person {
    age: i32,
}

impl Person {
    fn new(initial_age: i32) -> Person {
        let age = if initial_age < 0 {
            println!("Age is not valid, setting age to 0.");
            0
        } else {
            initial_age
        };

        return Person { age };
    }

    fn amIOld(&self) {
        if self.age < 13 {
            println!("You are young.")
        } else if self.age < 18 {
            println!("You are a teenager.")
        } else {
            println!("You are old.")
        }
    }

    fn yearPasses(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let T: i32 = read_line().trim().parse().unwrap();

    for _ in 0..T {
        let age = read_line().trim().parse().unwrap();
        let mut p = Person::new(age);

        p.amIOld();

        for _ in 0..3 {
            p.yearPasses();
        }

        p.amIOld();
        println!("");
    }
}
