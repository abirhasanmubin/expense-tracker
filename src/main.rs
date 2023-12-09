mod driver;
mod expense;

fn main() {
    let mut expense_list = driver::read_file();
    driver::home(&mut expense_list);
}
