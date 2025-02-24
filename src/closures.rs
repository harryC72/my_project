pub fn test_closures() {
    let add = |x, y| {
        let res: i32 = x + y;
        res.to_string()
    };

    let result = add(2, 3);

    println!("The result of adding 2 to 3 is {}", result);

    let print_result = || println!("Using Closure to get the result {}", result);

    print_result();

    let mut p1 = Person {
        first_name: "John".to_string(),
        last_name: "Williams".to_string(),
    };

    println!("{} {}", p1.first_name, p1.last_name);

    let mut change_name = |new_last_name: &str| {
        p1.last_name = new_last_name.to_string();
        println!("New last name: {}", p1.last_name); // Print after each change
    };

    change_name("Johnsey");
    change_name("Roosevelt");
}
