pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            name.split_whitespace()
                .filter_map(|word| word.chars().next())
                .map(|c| format!("{}. ", c))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}
