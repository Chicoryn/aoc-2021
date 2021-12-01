use std::io::{self, BufRead};

pub fn lines() -> io::Result<Vec<String>> {
    io::stdin().lock().lines().collect()
}
