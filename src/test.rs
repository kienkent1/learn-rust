fn caps(input: &str) -> String {
    input.to_string()
}

#[cfg(test)] // configuration
mod test {
    use crate::*;

    #[test]
    fn check() {
        let result = super::caps("hello");
        let expected = String::from("hello");
        assert_eq!(result, expected, "Fail test");
    }
}
