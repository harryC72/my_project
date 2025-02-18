pub fn insert_and_remove_cars() {
    let mut car_list: Vec<Car> = vec![
        Car {
            manufacturer: "Toyota".to_string(),
            model: "Corolla".to_string(),
        },
        Car {
            manufacturer: "Honda".to_string(),
            model: "Civic".to_string(),
        },
    ];

    // Insert a new car at the second position
    car_list.insert(
        1,
        Car {
            manufacturer: "Ford".to_string(),
            model: "Mustang".to_string(),
        },
    );

    println!("After insertion:");

    for car in &car_list {
        println!("Manufacturer: {}, Model: {}", car.manufacturer, car.model);
    }

    // Remove the first car if the list is not empty
    if !car_list.is_empty() {
        car_list.remove(0);
    }

    println!("After removal:");
    for car in &car_list {
        println!("Manufacturer: {}, Model: {}", car.manufacturer, car.model);
    }

    // Retain only the cars that are Porches
    car_list.retain(|car| car.manufacturer == "Porche");

    println!("After retaining only Porches:");
}
pub fn test_vec_int() {
    let mut my_ints: Vec<i32> = Vec::new();

    my_ints.push(30);
    my_ints.push(4);
    my_ints.push(8);

    println!("The vector length {:?}", my_ints.len());
    println!("The vector capacity {:?}", my_ints.capacity());
    println!("The vector containt {:?}", my_ints);
    print!("A cut of the vector {:?}", &my_ints[0..]);
    println!("");
}

pub fn test_vec_string() {
    let first_names: Vec<&str> = vec!["Harry", "Nancy", "Li"];

    for first_name in first_names.as_slice() {
        println!("Processing {} ...", first_name)
    }

    println!("{:?}", first_names);
}

#[derive(Clone, Debug)]
struct Car {
    manufacturer: String,
    model: String,
}

pub fn test_vec_car() {
    let mut car_list: Vec<Car> = vec![];

    for _ in 1..=8u8 {
        car_list.push(Car {
            manufacturer: "Porche".to_string(),
            model: "Benz".to_string(),
        })
    }

    for car in &car_list {
        println!("Manufacturer: {}, Model: {}", car.manufacturer, car.model);
    }
}
