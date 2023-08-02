use rand::{thread_rng, Rng};
use std::io::Stdin;

pub fn makes10(int1: i64, int2: i64) -> bool {
    int1 + int2 == 10
}

fn get_number(stdin: &Stdin) -> i64 {
    // get a number and convert it to i64
    let input = {
        let buf = &mut String::new();
        stdin.read_line(buf).expect("Could not read from stdin.");
        String::from(buf.trim())
    };

    if input == "" {
        let num:i64 = thread_rng().gen_range(1..20);
        println!("Using random number {num}");
        return num;
    }
    match input.parse::<i64>() {
        Ok(num) => {num}
        Err(_) => {
            println!("Provided input ({input}) is not a valid i64, using a random number instead");
            thread_rng().gen_range(1..20)
        }
    }
}

pub fn makes10_main(stdin: &Stdin) {
    println!("Pick first number or press enter for a random number");
    let num1 = get_number(stdin);

    println!("Pick second number or press enter for a random number");
    let num2 = get_number(stdin);

    let res = makes10(num1,num2);

    if res {
        println!("{num1} plus {num2} is equal to ten");
        return;
    }

    println!("{num1} plus {num2} is not equal to ten");
}
