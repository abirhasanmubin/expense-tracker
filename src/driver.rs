use std::io;
use std::num::ParseIntError;
use std::process;

use colored::*;

use crate::expense::{ExpenseCategory, ExpenseList};

pub fn home(expense_list: &mut ExpenseList) {
    println!("Welcome to Expense Tracker CLI!");
    println!();
    println!("1. Add an expense");
    println!("2. View all expenses");
    // println!("3. View expenses by category");
    // println!("4. View expenses by date range");
    println!("3. Calculate total expenses");
    println!("4. Delete an expense");
    println!("5. Exit");
    // take input Enter your choice:
    let input = take_number_input("Please Enter Your Choice: ", 1, 5);
    match input {
        1 => {
            add_expense(expense_list);
        }
        2 => {
            view_all_expense(expense_list);
        }
        3 => {
            calculate_total_expenses(expense_list);
        }
        4 => {
            delete_expense(expense_list);
        }
        5 => {
            println!("{}", "Good bye".green());
            process::exit(0);
        }
        _ => {
            println!("{}", "Good bye".red());
            process::exit(1);
        }
    }
}

pub fn add_expense(expense_list: &mut ExpenseList) {
    println!("Enter expense details:");
    let category = match take_number_input("Select Category", 1, 12) {
        1 => ExpenseCategory::FoodAndDining,
        2 => ExpenseCategory::Transportation,
        3 => ExpenseCategory::Utilities,
        4 => ExpenseCategory::Housing,
        5 => ExpenseCategory::Entertainment,
        6 => ExpenseCategory::Healthcare,
        7 => ExpenseCategory::ClothingAndPersonalCare,
        8 => ExpenseCategory::Travel,
        9 => ExpenseCategory::Education,
        10 => ExpenseCategory::GiftsAndDonations,
        11 => ExpenseCategory::SavingsAndInvestments,
        12 => ExpenseCategory::Miscellaneous,
        _ => ExpenseCategory::Miscellaneous,
    };
    let amount = take_number_input("Enter amount: ", 1, 100000);
    println!("Enter description: ");
    let description = take_input();
    expense_list.add(category, amount, description);
    println!("{}", "Expense added successfully!".green());
    println!();
    home(expense_list);
}

pub fn view_all_expense(expense_list: &mut ExpenseList) {
    println!("Here are all your expenses");
    expense_list.view_all();
    println!();
    home(expense_list);
}

pub fn calculate_total_expenses(expense_list: &mut ExpenseList) {
    println!(
        "Total expense: ৳{}",
        expense_list.get_total_expense().to_string().green()
    );
    // total function call
    println!();
    // call home function
    home(expense_list);
}

pub fn delete_expense(expense_list: &mut ExpenseList) {
    let id = take_number_input("Enter id: ", 1, 100000);

    expense_list.remove(id);
    // take input Enter ID of expense to delete:
    println!("{}", "Expense deleted successfully!".yellow());
    // call home function
    home(expense_list);
}

fn take_number_input(input_message: &str, minimum: u32, maximum: u32) -> u32 {
    println!("{}", input_message);
    loop {
        let input = take_input();
        match validate_choice(input.as_str(), minimum, maximum) {
            Ok(true) => return input.parse().expect("failed to parse input"),
            Ok(false) => {
                println!("{}", "Choice is out of range.".red());
                continue;
            }
            Err(_) => {
                println!("{}", "Please enter a valid input.".red());
                continue;
            }
        }
    }
}

fn take_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");
    input.trim().to_string()
}

fn validate_choice(input: &str, start: u32, end: u32) -> Result<bool, ParseIntError> {
    match input.parse::<u32>() {
        Ok(value) => Ok(value >= start && value <= end),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_choice() {
        assert_eq!(validate_choice("5", 1, 10), Ok(true));
    }

    #[test]
    fn test_invalid_choice() {
        assert_eq!(validate_choice("15", 1, 10), Ok(false));
    }

    #[test]
    fn test_invalid_input() {
        assert!(validate_choice("abc", 1, 10).is_err());
    }
}
