use crate::pick_program::Program;
use std::io;

mod pick_program;
mod programs;

fn main() {
    println!("\n\n\n\n\n\n\n\n\n\n");
    // get stdin handle so we can reuse it
    let stdin = io::stdin();

    loop {
        println!("\n");
        // get program to run
        let program: Program = pick_program::pick_program(&stdin);

        println!("\n");

        match program {
            Program::PrintMessage => {
                programs::print_message::print_message();
            }
            Program::ChooseBeverage => {
                programs::choose_beverage::choose_beverage(&stdin);
            }
            Program::Makes10 => {
                programs::makes_10::makes10_main(&stdin);
            }
            Program::PrintNumbers => programs::print_numbers::print_numbers(),
            Program::SumInput => {
                programs::sum_input::sum_input(&stdin);
            }
            Program::Exit => {
                break;
            }
        }
    }
}
