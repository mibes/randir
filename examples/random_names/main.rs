use randir::utils::generate_entries;

fn main() {
    // generate 100 random names and telephone numbers
    let directory = generate_entries(100);
    for entry in directory {
        println!("{}", entry)
    }
}
