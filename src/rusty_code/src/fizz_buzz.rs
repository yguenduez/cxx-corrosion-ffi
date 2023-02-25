pub struct FizzBuzz;

impl FizzBuzz {
    pub fn to_fizz_buzz(&self, input: u32) -> String {
        if input % 15 == 0 {
            return "FizzBuzz".to_string();
        }

        if input % 5 == 0 {
            return "Buzz".to_string();
        }

        if input % 3 == 0 {
            return "Fizz".to_string();
        }

        input.to_string()
    }
}

mod tests {
    use crate::fizz_buzz::FizzBuzz;

    #[test]
    fn on_3_return_fizz() {
        // given
        let fizz_buzzer = FizzBuzz;
        let number = 3;

        // when
        let result = fizz_buzzer.to_fizz_buzz(number);

        // then
        assert_eq!("Fizz".to_string(), result);
    }

    fn on_4_return_4_as_string() {
        // given
        let fizz_buzzer = FizzBuzz;
        let number = 4;

        // when
        let result = fizz_buzzer.to_fizz_buzz(number);

        // then
        assert_eq!("4".to_string(), result);
    }

    #[test]
    fn on_5_return_buzz() {
        // given
        let fizz_buzzer = FizzBuzz;
        let number = 5;

        // when
        let result = fizz_buzzer.to_fizz_buzz(number);

        // then
        assert_eq!("Buzz".to_string(), result);
    }

    #[test]
    fn on_15_return_fizzbuzz() {
        // given
        let fizz_buzzer = FizzBuzz;
        let number = 15;

        // when
        let result = fizz_buzzer.to_fizz_buzz(number);

        // then
        assert_eq!("FizzBuzz".to_string(), result);
    }
}
