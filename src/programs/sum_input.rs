use std::io::Stdin;

pub fn sum_input(stdin:&Stdin) {
    let mut num: i64 = 1;
    let mut sum: i64 = 0;

    while num != 0 {
        // prompt user
        println!("Enter a number (current sum: {sum}) or 0 to finish");
        // get user input
        let next_num_str = &mut String::new();
        stdin.read_line(next_num_str).expect("Could not read from stdin.");

        // convert to number
        let input = next_num_str
            .trim()
            .parse::<i64>();

        match input {
            Ok(next_num) => {
                num = next_num;
                sum += next_num;
            }
            Err(_) => {
                println!("Provided number is not a valid i64");
            }
        }
    }
    println!("{sum}")
}