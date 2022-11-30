fn execute_code_no_return() {
    print!("Hello, ");
}

fn execute_code_return_string(name: String) -> String {
    format!("{}", name)
}

fn main() {
    execute_code_no_return();
    print!("{}\n", execute_code_return_string(String::from("Akash")));
}
