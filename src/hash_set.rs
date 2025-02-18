use std::collections::HashSet;
pub fn create_planet_list() -> HashSet<&'static str> {
    let mut first_planet_list: HashSet<&'static str> = HashSet::from(["Mercury", "Venus", "Earth"]);
    first_planet_list.insert("Mercury");
    first_planet_list.insert("Venus");
    first_planet_list.insert("Earth");
    let mut second_planet_list: HashSet<&'static str> =
        HashSet::from(["Mars", "Jupiter", "Saturn"]);
    second_planet_list.insert("Mars");
    second_planet_list.insert("Jupiter");
    second_planet_list.insert("Saturn");

    println!("first planet list");
    println!("{:?}", first_planet_list);

    println!("second planet list");
    println!("{:?}", second_planet_list);
    first_planet_list
}
pub fn create_hash_set() -> HashSet<i32> {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    println!("hashset");
    println!("{:?}", set);
    set
}
