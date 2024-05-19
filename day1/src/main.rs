use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    part_one();

    part_two();
}

fn part_one() {
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

struct Num<'a> {
    substring: &'a str,
    digit: &'a str,
    value: u32,
}

struct Match {
    value: u32,
    index: u32,
}

fn part_two() {
    let search_values: &[Num] = [
        Num {
            substring: "zero",
            digit: "0",
            value: 0,
        },
        Num {
            substring: "one",
            digit: "1",
            value: 1,
        },
        Num {
            substring: "two",
            digit: "2",
            value: 2,
        },
        Num {
            substring: "three",
            digit: "3",
            value: 3,
        },
        Num {
            substring: "four",
            digit: "4",
            value: 4,
        },
        Num {
            substring: "five",
            digit: "5",
            value: 5,
        },
        Num {
            substring: "six",
            digit: "6",
            value: 6,
        },
        Num {
            substring: "seven",
            digit: "7",
            value: 7,
        },
        Num {
            substring: "eight",
            digit: "8",
            value: 8,
        },
        Num {
            substring: "nine",
            digit: "9",
            value: 9,
        },
    ]
    .as_slice();
    if let Ok(lines) = read_lines("input.txt") {
        let result = lines
            .map_while(Result::ok)
            .fold(0u32, |acc: u32, line: String| {
                let mut matches: Vec<Match> = vec![];
                for search_value in search_values {
                    line.match_indices(search_value.digit).for_each(|(idx, _)| {
                        matches.push(Match {
                            value: search_value.value,
                            index: idx as u32,
                        });
                    });
                    line.match_indices(search_value.substring)
                        .for_each(|(idx, _)| {
                            matches.push(Match {
                                value: search_value.value,
                                index: idx as u32,
                            });
                        });
                }
                matches.sort_by_key(|m| m.index);
                let first = matches.first().unwrap();
                let last = matches.last().unwrap();
                let num_string: String = format!("{}{}", first.value, last.value);
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
