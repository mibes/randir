use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Entry {
    pub uid: usize,
    pub first_name: String,
    pub last_name: String,
    pub phone_nr: String,
}

impl Entry {
    pub fn new(uid: usize, first_name: &str, last_name: &str, phone_nr: &str) -> Self {
        Entry {
            uid,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            phone_nr: phone_nr.to_string(),
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let full_name = format!("{} {}", self.first_name, self.last_name);
        f.write_fmt(format_args!("{:20} {}", full_name, self.phone_nr))
    }
}
