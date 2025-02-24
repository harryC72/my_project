// mod animal;
// mod hash_map;
// mod hash_set;
//mod vec_training_ground;
//mod iters;
//mod date_time;
//mod options;
//mod closures;
//mod matcher;
//mod struct_sample;
//mod threads;
mod scoped_threads;

fn main() {
    // let arr = [1, 2, 3];
    // print_array(arr);

    // let arr = ["hello", "world", "now"];
    // print_array(arr);

    // animal::create_person();
    // vec_training_ground::test_vec_int();
    // vec_training_ground::test_vec_string();
    // vec_training_ground::test_vec_car();
    // vec_training_ground::insert_and_remove_cars();
    // hash_map::test_hashmap_basic();
    // hash_set::create_hash_set();
    // hash_set::create_planet_list();
    // iters::test_rust_iterators();
    //date_time::test_stdtime();
    // date_time::test_chrono();
    // let result = options::test_option_type();

    // let chosen_char = chose_chartype(Archer);

    // println!("The chosen class is: {}", chosen_char.unwrap().to_string());

    // println!("{}", result.unwrap())
    // closures::test_closures();
    //matcher::test_match_string();
    //matcher::test_match_array();

    // let instance_person = struct_sample::new_person("John", "Carlson", 1983, 8);

    // instance_person.print_data();

    // let mut instance_vehicle =
    //     Vehicle::create_vehicle("BMW", 1979, "Bernie", struct_sample::VehicleColor::Blue);

    // instance_vehicle.print_data();

    // instance_vehicle.paint(struct_sample::VehicleColor::Green);

    // instance_vehicle.nick_name.set("Steve".to_string());

    //spawn_thread();

    scoped_threads::test_thread_variables();
}

// fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
