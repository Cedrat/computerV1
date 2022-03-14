mod tokeniser;
use crate::tokeniser::create_dict_token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let tokens: Vec<String> = "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0".split(" ").map(|s| s.to_string()).collect();
        println!(create_dict_token(tokens));
    }
}