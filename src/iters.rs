pub fn test_rust_iterators() {
    let fruit_list = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "kiwi",
        "lemon",
    ];

    let nut_list = vec![
        "almond",
        "brazil nut",
        "cashew",
        "hazelnut",
        "macadamia",
        "pecan",
        "pistachio",
        "walnut",
    ];

    let mut eatables_iter = fruit_list.iter().chain(nut_list.iter());
    let item01: Option<&&str> = eatables_iter.next();

    println!("First item is {}", item01.unwrap());

    let all_foods: Vec<&&str> = eatables_iter.collect();

    for food in &all_foods {
        println!("Eating {}", food);
    }

    println!("{:#?}", all_foods);

    let mut fruit_list_strings: Vec<String> = fruit_list.iter().map(|e| String::from(*e)).collect();

    let new_fruit_list_strings: Vec<String> = fruit_list_strings
        .iter_mut()
        .map(|e| {
            e.push_str(" fruit");
            e.clone()
        })
        .collect();

    new_fruit_list_strings
        .iter()
        .for_each(|e| println!("{}", e));

    println!("Last fruit {:#?}", new_fruit_list_strings.last());

    let last_fruit: Option<&String> = new_fruit_list_strings.last();
    if let Some(fruit) = last_fruit {
        println!("Last fruit {}", fruit);
    } else {
        println!("No fruits found");
    }

    let mut step_by = new_fruit_list_strings.clone().into_iter().step_by(2);

    println!("Step {}", step_by.next().unwrap());
    println!("Step {}", step_by.next().unwrap());
    println!("Step {}", step_by.next().unwrap());

    let first_names: Vec<&str> = vec!["Trevor", "Shannon", "James", "Tasha"];
    let first_names_strings = first_names
        .iter()
        .map(|e| String::from(*e))
        .collect::<Vec<String>>();

    let last_names: Vec<&str> = vec!["Jones", "Smiths", "Jameson", "Tordoux"];
    let last_names_strings = last_names
        .iter()
        .map(|e| String::from(*e))
        .collect::<Vec<String>>();

    let full_names: Vec<(&String, String)> =
        first_names_strings.iter().zip(last_names_strings).collect();

    full_names.iter().for_each(|e| println!("{} {}", e.0, e.1));

    for (index, value) in full_names.iter().enumerate() {
        println!("Index: {} Name: {} {}", index, value.0, value.1);
    }

    //	skip, take

    let foods: Vec<(&str, i32)> = vec![("potatoes", 10), ("strawberries", 25), ("burgers", 31)];

    let total_food_quantity = foods.iter().fold(0, |acc, e| acc + e.1);
    println!("Total foods: {}", total_food_quantity);

    let mut peekable_foods = foods.iter().peekable();
    let food: Option<&&(&str, i32)> = peekable_foods.peek();
    println!("{}", food.unwrap().0);
    peekable_foods.next();
    let food2: Option<&&(&str, i32)> = peekable_foods.peek();
    println!("{}", food2.unwrap().0);
}
