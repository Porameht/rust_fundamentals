use std::string;

fn main() {
    let mut original = String::from("original value");
    print!("\nouter scope original: \t\'{}'", original);

    {
        print_original(&original);
    }
}

fn print_original(original: &String) {
    println!("\nfn print_original: \t\"{}\"", original)
}
