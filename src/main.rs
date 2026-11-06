use std::io;

fn main() {
    let mut trials: i32 = 0;
    loop {
        let mut input: String = String::new();
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        io::stdin().read_line(&mut input).unwrap();
        trials += 1;
        if input.trim() == "The letter e" {
            break;
        }
        println!("You typed: {}", input.trim());
    }
    println!("Number of trials: {}", trials);
}
