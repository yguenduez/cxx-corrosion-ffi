pub struct Sender;

impl Sender{
    pub fn get_from_rust_lang(&self) -> String{
        let body = reqwest::blocking::get("https://www.rust-lang.org").unwrap()
            .text().unwrap();
        body
    }
}

mod tests {
    use crate::blocking_http_sender::Sender;
    use crate::fizz_buzz::FizzBuzz;

    #[test]
    fn print_html_as_string() {
        // given
        let sender = Sender;
        println!("{}", sender.get_from_rust_lang());
    }
}