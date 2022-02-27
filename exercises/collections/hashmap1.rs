// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint hashmap1` if you need
// hints.



use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket: HashMap<String, u32> = HashMap::new();// TODO: declare your hash map here.

    // Two bananas are already given for you :)
    // basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    // basket.insert(String::from("apple"), 8);
    let fruit: Vec<String> = vec![String::from("melon"), String::from("cherry"), String::from("grape"), String::from("banana"), String::from("apple")] ;
    let prices: Vec<u32> = vec![23, 24, 45, 2, 3];
    let basket: HashMap<String, u32> = fruit.into_iter().zip(prices.into_iter()).collect();

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
