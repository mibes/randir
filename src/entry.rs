use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Entry {
    pub uid: usize,
    pub first_name: String,
    pub last_name: String,
    pub phone_nr: String,
    pub email: String,
}

impl Entry {
    pub fn new(
        uid: usize,
        first_name: &str,
        last_name: &str,
        phone_nr: String,
        email: String,
    ) -> Self {
        Entry {
            uid,
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            phone_nr,
            email,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let full_name = format!("{} {}", self.first_name, self.last_name);
        f.write_fmt(format_args!(
            "{:20} {:20} ({})",
            full_name, self.phone_nr, self.email
        ))
    }
}
