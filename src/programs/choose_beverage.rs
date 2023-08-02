use std::io::Stdin;

pub fn choose_beverage(stdin:&Stdin) {
    loop {
        println!("Enter a number to choose one of following three beverages. The choices are: (1) Water, (2) Coke, (3) Coffee.");
        let input = &mut String::new();
        stdin.read_line(input).expect("Could not read from stdin.");

        match input.trim() {
            "1" => {
                println!("Water");
                break;
            }
            "2" => {
                println!("Coke");
                break;
            }
            "3" => {
                println!("Coffee");
                break;
            }
            _ => {
                println!("Invalid choice.")
            }
        }
    }

}