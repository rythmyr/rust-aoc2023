use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let result = lines
            .map_while(Result::ok)
            .fold(0u32, |acc: u32, line: String| {
                let mut first: Option<char> = None;
                let mut last: Option<char> = None;
                for char in line.chars() {
                    if char.is_ascii_digit() {
                        first = Some(char);
                        break;
                    }
                }
                for char in line.chars().rev() {
                    if char.is_ascii_digit() {
                        last = Some(char);
                        break;
                    }
                }
                let num_string: String = format!("{}{}", first.unwrap(), last.unwrap());
                let num: u32 = num_string.parse().unwrap();
                acc + num
            });
        println!("{}", result);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
