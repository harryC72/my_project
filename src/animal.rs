// Removed the empty Person struct definition

trait Animal {
    fn make_sound(&self) -> ();
}

trait NotDangerous {}

struct Dog {}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof");
    }
}

struct Cat {}
impl NotDangerous for Cat {}
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Miau");
    }
}

struct Person<PetType: Animal + NotDangerous> {
    first_name: String,
    pet: PetType,
}

pub fn create_person() {
    let _pet1: Dog = Dog {};
    let pet2: Cat = Cat {};
    let _p1: Person<Cat> = Person {
        first_name: "Harry".to_string(),
        pet: pet2,
    };
    println!(
        "Person's name is {} and they have a pet that says:",
        _p1.first_name
    );
    _p1.pet.make_sound();
}
