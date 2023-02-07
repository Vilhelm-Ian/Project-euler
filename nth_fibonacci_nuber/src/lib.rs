mod main;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first_two_digit_number() {
        let first_fibbonachi = main::generate_first_fibonachi_number_with_n_digits(1, 1, 2);
        assert_eq!(first_fibbonachi, 13);
    }
    #[test]
    fn find_first_three_digit_number() {
        let first_fibbonachi = main::generate_first_fibonachi_number_with_n_digits(1, 1, 3);
        assert_eq!(first_fibbonachi, 144);
    }
}
