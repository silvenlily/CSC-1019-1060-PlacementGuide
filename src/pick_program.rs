use std::env;
use std::io::Stdin;

pub enum Program {
    PrintMessage,
    ChooseBeverage,
    Makes10,
    PrintNumbers,
    SumInput,
    Exit,
}

fn match_program(input: &String) -> Option<Program> {
    match input.to_lowercase().trim_end() {
        "1" | "printmessage" => Some(Program::PrintMessage),
        "2" | "choosebeverage" => Some(Program::ChooseBeverage),
        "3" | "makes10" => Some(Program::Makes10),
        "4" | "printnumbers" => Some(Program::PrintNumbers),
        "5" | "suminput" => Some(Program::SumInput),
        "" | "exit" => Some(Program::Exit),
        _ => None,
    }
}

fn prompt_program(stdin: &Stdin) -> Program {
    loop {
        // if Program to run is not provided prompt user for Program
        println!("Select Program to run (1-5) or press enter to exit");
        let input = &mut String::new();
        stdin.read_line(input).expect("Could not read from stdin.");
        // check what program user selected

        if let Some(p) = match_program(input) {
            return p;
        }

        println!("Invalid input, valid inputs are any number one to five or: 'PrintMessage', 'ChooseBeverage', 'Makes10', 'PrintNumbers', 'SumInput' and 'Exit'.");
    }
}

pub fn pick_program(stdin: &Stdin) -> Program {
    // get args as vec
    let args: Vec<String> = env::args().collect();

    // check if Program to run was provided as an argument
    match args.get(1) {
        None => {
            // if not then prompt for program
            prompt_program(stdin)
        }
        Some(arg) => {
            // if yes check if its we can match the user input to a program
            if let Some(program) = match_program(arg) {
                return program;
            }

            // if we cant, say so and prompt for input
            println!("Unknown argument: {arg}.");
            prompt_program(stdin)
        }
    }
}
