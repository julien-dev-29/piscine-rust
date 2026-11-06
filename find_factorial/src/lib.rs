pub fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => num * factorial(num - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: u64 = factorial(10);
        assert_eq!(result, 3628800);
    }
}
