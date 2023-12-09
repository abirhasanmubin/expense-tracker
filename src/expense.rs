use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseList {
    pub expenses: Vec<Expense>,
    current_id: u32,
}

impl ExpenseList {
    pub fn new() -> ExpenseList {
        ExpenseList {
            expenses: Vec::new(),
            current_id: 0,
        }
    }

    pub fn add(&mut self, category: ExpenseCategory, amount: u32, description: String) {
        self.current_id += 1;
        self.expenses
            .push(Expense::new(self.current_id, category, amount, description))
    }

    pub fn remove(&mut self, id: u32) {
        if id > self.current_id {
            println!("{}", "Index out of bound".red());
        } else {
            self.expenses.retain(|expense| expense.id != id);
        }
    }

    pub fn len(&self) -> usize {
        self.expenses.len()
    }
    pub fn get_current_id(&self) -> u32 {
        self.current_id
    }

    pub fn view_all(&self) {
        for (index, expense_ref) in self.expenses.iter().enumerate() {
            println!("{}: {}", index + 1, expense_ref.formatted().green());
        }
    }

    pub fn get_total_expense(&self) -> u32 {
        let total_expense: u32 = self.expenses.iter().map(|expense| expense.amount).sum();
        total_expense
    }
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum ExpenseCategory {
    FoodAndDining,
    Transportation,
    Utilities,
    Housing,
    Entertainment,
    Healthcare,
    ClothingAndPersonalCare,
    Travel,
    Education,
    GiftsAndDonations,
    SavingsAndInvestments,
    Miscellaneous,
}

impl Display for ExpenseCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpenseCategory::FoodAndDining => write!(f, "Food And Dining"),
            ExpenseCategory::Transportation => write!(f, "Transportation"),
            ExpenseCategory::Utilities => write!(f, "Utilities"),
            ExpenseCategory::Housing => write!(f, "Housing"),
            ExpenseCategory::Entertainment => write!(f, "Entertainment"),
            ExpenseCategory::Healthcare => write!(f, "Healthcare"),
            ExpenseCategory::ClothingAndPersonalCare => write!(f, "Clothing and Personal Care"),
            ExpenseCategory::Travel => write!(f, "Travel"),
            ExpenseCategory::Education => write!(f, "Education"),
            ExpenseCategory::GiftsAndDonations => write!(f, "Gifts and Donations"),
            ExpenseCategory::SavingsAndInvestments => write!(f, "Savings and Investments"),
            ExpenseCategory::Miscellaneous => write!(f, "Miscellaneous"),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Expense {
    pub id: u32,
    pub amount: u32,
    pub category: ExpenseCategory,
    pub description: String,
    pub date: DateTime<Utc>,
}

impl Expense {
    pub fn new(id: u32, category: ExpenseCategory, amount: u32, description: String) -> Expense {
        Expense {
            id,
            category,
            amount,
            description,
            date: Utc::now(),
        }
    }

    pub fn formatted(&self) -> String {
        format!(
            "৳{} for {} on {} (ID: {})",
            self.amount,
            &self.category,
            self.date.format("%x %I:%M %P"),
            self.id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_tests() {
        let mut expense_list = ExpenseList::new();
        expense_list.add(
            ExpenseCategory::FoodAndDining,
            200,
            String::from("Test Description"),
        );
        expense_list.add(
            ExpenseCategory::FoodAndDining,
            100,
            String::from("Test Description"),
        );

        expense_list.view_all();

        assert_eq!(expense_list.len(), 2);
        assert_eq!(expense_list.get_total_expense(), 300);

        expense_list.remove(0);
        assert_eq!(1, expense_list.len());
        assert_eq!(100, expense_list.get_total_expense());
        assert_eq!(2, expense_list.get_current_id());
    }

    #[test]
    fn expense_new_test() {
        let expense = Expense::new(
            1,
            ExpenseCategory::FoodAndDining,
            100,
            String::from("Test Description"),
        );

        assert_eq!(100, expense.amount);
        assert_eq!(1, expense.id);
        assert_eq!("Test Description", expense.description);
    }

    #[test]
    fn expense_display_test() {
        let expense = Expense::new(
            1,
            ExpenseCategory::FoodAndDining,
            100,
            String::from("Test Description"),
        );

        let time = Utc::now().format("%x %I:%M %P");
        let str = format!("৳100 for Food And Dining on {time} (ID: 1)");
        assert_eq!(str, expense.formatted());
    }
}
