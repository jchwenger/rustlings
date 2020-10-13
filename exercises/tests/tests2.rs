// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2, 2);
        assert_eq!("2".to_string(), String::from("2"));
        assert_eq!('2'.to_string(), String::from("2")); // a char, https://stackoverflow.com/a/16267760
    }
}
