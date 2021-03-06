fn fizzbuzz(num: i8) -> String {
    if num % (3*5) == 0 {
        return "fizzbuzz".to_string();
    }
    if num % 3 == 0 {
        return "fizz".to_string();
    }
    if num % 5 == 0 {
        return "buzz".to_string();
    }

    return num.to_string();
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz;

    #[test]
    fn test_it_returns_given_number() {
        let result = fizzbuzz(1);

        assert_eq!(result, "1");
    }

    #[test]
    fn test_it_returns_fizz_when_given_three() {
        let result = fizzbuzz(3);

        assert_eq!(result, "fizz");
    }

    #[test]
    fn test_it_returns_fizz_when_given_a_multiple_of_three() {
        assert_eq!(fizzbuzz(6), "fizz");
        assert_eq!(fizzbuzz(9), "fizz");
        assert_eq!(fizzbuzz(12), "fizz");
    }

    #[test]
    fn test_it_returns_buzz_when_given_a_5() {
        let result = fizzbuzz(5);

        assert_eq!(result, "buzz");
    }

    #[test]
    fn test_it_returns_buzz_when_given_a_multiple_of_5() {
        assert_eq!(fizzbuzz(5), "buzz");
        assert_eq!(fizzbuzz(10), "buzz");
        assert_eq!(fizzbuzz(20), "buzz");
    }

    #[test]
    fn test_it_returns_fizzbuzz_when_given_a_multiple_of_3_and_5() {
        assert_eq!(fizzbuzz(15), "fizzbuzz");
        assert_eq!(fizzbuzz(30), "fizzbuzz");
    }
}