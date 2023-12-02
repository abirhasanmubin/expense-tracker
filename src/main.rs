mod driver;
mod expense;

fn main() {
    println!("Hello, world!");
    let mut expense_list = expense::ExpenseList::new();
    driver::home(&mut expense_list);
}
