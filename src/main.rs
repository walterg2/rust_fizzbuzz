fn fizzbuzz(num: i8) -> String {
    return num.to_string();
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;

    #[test]
    fn test_fizzbuzz_returns_given_number() {
        let result = fizzbuzz(1);

        assert_eq!(result, "1");
    }
}