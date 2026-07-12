use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut i = 0;

    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        input.clear();
        io::stdin().read_line(&mut input)?;
        i += 1;
        if input.trim() == "e" {
            break;
        }
    }

    println!("Number of trials: {}", i);
    Ok(())
}
