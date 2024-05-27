fn main() {
    let lines = include_str!("../input.txt").lines();

    lines.for_each(|line| {
        println!("{}", line);
    });
}
