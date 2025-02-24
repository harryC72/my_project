use my_project::person::Person;
use std::thread;

pub fn test_thread_variables() {
    let age = 45;

    let person01 = Person {
        first_name: "Harry".to_string(),
        last_name: "Cole".to_string(),
    };

    let print_age = move || {
        println!("Age: {}", age);
        println!("First name: {}", person01.first_name);
    };

    let _result = thread::spawn(print_age).join();

    println!("Finished printing age");
}
