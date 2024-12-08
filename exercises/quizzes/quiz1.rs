// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    let quantity = 45; // Example: Mary buys 45 apples
    let total_price = calculate_price_of_apples(quantity);
    println!(
        "The total price for {} apples is {} rustbucks.",
        quantity, total_price
    );
}
fn calculate_price_of_apples(quantity: u32) -> u32 {
    let price_per_apple = if quantity > 40 { 1 } else { 2 };
    quantity * price_per_apple
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
