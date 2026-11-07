pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let div: i32 = x / y;
    let rem: i32 = x % y;
    (div, rem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: (i32, i32) = divide(6, 3);
        assert_eq!(result, (2, 0));
    }
}
