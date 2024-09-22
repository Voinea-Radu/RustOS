use crate::binaries::shell::ShellError;
use crate::driver::vga::WRITER;
use crate::println;
use alloc::string::String;
use alloc::vec::Vec;

pub fn nop_command_handler(_arguments_string: String) -> Result<(), ShellError> {
    Ok(())
}

pub fn pudel_command_handler(_arguments_string: String) -> Result<(), ShellError> {
    println!("Pudel <3 Daria");
    Ok(())
}

pub fn daria_command_handler(_arguments_string: String) -> Result<(), ShellError> {
    println!("Daria <3 Pudel");
    Ok(())
}

pub fn add_command_handler(arguments_string: String) -> Result<(), ShellError> {
    let arguments: Vec<&str> = arguments_string.split(" ").collect();

    if arguments.len() != 3 {
        println!("Usage: {} <number_1> <number_2>", arguments.get(0).unwrap());
        return Ok(());
    }

    let number1: i32 = arguments
        .get(1)
        .unwrap()
        .parse::<i32>()
        .map_err(|_error| ShellError::InvalidArgumentsError)?;
    let number2: i32 = arguments
        .get(2)
        .unwrap()
        .parse::<i32>()
        .map_err(|_error| ShellError::InvalidArgumentsError)?;

    println!("{} + {} = {}", number1, number2, number1 + number2);
    Ok(())
}
pub fn clear_command_handler(_arguments_string: String) -> Result<(), ShellError> {
    WRITER.lock().clear();
    Ok(())
}
