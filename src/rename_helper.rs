use aoc19::data_path;
use itertools::Itertools;
use std::fs::rename;


fn new_name(name: &str) -> String {
    name.split('_')
        .enumerate()
        .filter_map(|(ix, s)| if ix != 1 {
            Some(s)
        } else {
            None
        })
        .join("_")
}

fn main() {
    for day in data_path().read_dir().unwrap() {
        let day = day.unwrap().path();
        for files in day.read_dir().unwrap() {
            let file = files.unwrap().path();

            let mut new_path = day.clone();
            let name = file.file_name().unwrap();
            let name = new_name(name.to_str().unwrap());
            new_path.push(name);

            rename(file, new_path).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename()  {
        assert_eq!("in_01.data", &new_name("in_01_01.data"));
        assert_eq!("in_01.data", &new_name("in_09_01.data"));
    }

}
