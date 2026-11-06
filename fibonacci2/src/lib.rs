pub fn fibonacci2(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci2(n - 1) + fibonacci2(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
