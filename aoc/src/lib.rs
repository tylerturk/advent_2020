use std::fs;

pub fn input() -> String {
    read_file("input.txt")
}

pub fn read_file(path: &str) -> String  {
    fs::read_to_string(path).unwrap()
}

pub fn sample() -> String {
    read_file("sample.txt")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
