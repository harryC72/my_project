use std::collections::HashMap;

pub fn test_hashmap_basic() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();

    stock_list.insert("NVDA".to_string(), 478.52);
    stock_list.insert("AAPL".to_string(), 150.73);
    stock_list.insert("GOOGL".to_string(), 2750.00);

    println!("{:#?}", stock_list);

    stock_list.remove(&("NVDA".to_string()));

    println!("{:#?}", stock_list);

    stock_list.entry("MSFT".to_string()).or_insert(299.35);
    println!("{:#?}", stock_list);

    for (key, curr_value) in &stock_list {
        println!("{} is trading at {}", key, curr_value);
    }
}
