use std::io;

fn user_input() -> (i32, i32) {
    println!("Enter 2 Numbers");
    let mut numbers = String::new();
    
    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");

/*
Shadowing the variable numbers
*/

    let numbers: Vec<&str> = numbers 
                .trim()
                .split_whitespace()
                .collect();
    (numbers[0].parse().expect("Number1 is not valid"), numbers[1].parse().expect("Number2 is not valid"))
}

fn add() -> i32 {
    let (num1 , num2) = user_input();
    num1 + num2
}

fn sub() -> i32 {
    let (num1 , num2) = user_input();
    num1 - num2
}

fn mul() -> i32 {
    let (num1 , num2) = user_input();
    num1 * num2
}

fn div() -> i32 {
    let (num1 , num2) = user_input();
    num1 / num2
}   

fn main() {

    loop{
        println!("Enter Choice:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice
            .trim()
            .parse() { 
            Ok(num) => num,
            Err(_) => panic!("Invalid input..... Input must be a number"),
        };
        
        match choice {
            1 => {
                println!("Addition");
                println!("Result: {}", add());
            },
            2 => {
                println!("Subtraction");
                println!("Result: {}", sub());
            },
            3 => {
                println!("Multiplication");
                println!("Result: {}", mul());
            },
            4 => {
                println!("Division");
                println!("Result: {}", div());
            },
            5 => {
                println!("Exiting");
                break;
            },
            _ => println!("Invalid input.....Please enter valid choice"),
        }
    }     
}
