# randir
Generates a directory with random names, telephone numbers & email addresses.

# Example use
```rust
use randir::utils::generate_entries;

fn main() {
    // generate 100 random names, telephone numbers & email addresses
    let directory = generate_entries(100);
    for entry in directory {
        println!("{}", entry)
    }
}
```

# Detail
The result type of the `generate_entries()` function is a `Vec<Entry>`. `Entry` is defined as:

```rust
pub struct Entry {
    pub uid: usize,
    pub first_name: String,
    pub last_name: String,
    pub phone_nr: String,
    pub email: String,
}
```

# Importing
To use the random directory generator, you need to include it in the `Cargo.toml` file of your Rust project:

```toml
[dependencies]
randir = "0.2"
```
