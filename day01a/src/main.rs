pub fn main() {
    println!("{}",
        include_str!("../input.txt")
            .split("\n")
            .fold(0, |acc: i32, s: &str| {
                let left: char = s.chars().find(|c| c.is_numeric()).unwrap();
                let right: char = s.chars().rfind(|c| c.is_numeric()).unwrap();
                acc + String::from_iter([left, right]).parse::<i32>().unwrap()
            })
    )
}
