pub fn first_subword(s: String) -> String {
    for (i, c) in s.char_indices() {
        if i != 0 && (c == '_' || c.is_uppercase()) {
            return s[..i].to_string();
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = first_subword("helloWorld".to_owned());
        assert_eq!(result, "hello");
    }
}
