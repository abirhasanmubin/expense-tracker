mod driver;
mod expense;

fn main() {
    println!("Hello, world!");
    let mut expense_list = driver::read_file();
    driver::home(&mut expense_list);
}
