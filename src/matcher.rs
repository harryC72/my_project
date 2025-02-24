pub fn test_match_string() {
    let car_manufacturer = "Porsche";

    let price = match car_manufacturer {
        "Hyundai" => 3000,
        "Porsche" => 9000,
        _ => 0,
    };
    println!("The price is: {}", price);
}

pub fn test_match_array() {
    let prices = [5000, 8000, 10000];

    match prices[0..=1] {
        [3000, 5000] => println!("You got some reasonably priced cars"),
        _ => println!("You are just to pricy"),
    }
}
