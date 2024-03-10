// Define a trait named Sound
trait Sound {
    // A method indicating the sound the object makes
    fn make_sound(&self);
}

// Define a struct Dog
struct Dog {
    name: String,
}

// Implement the Sound trait for Dog
impl Sound for Dog {
    fn make_sound(&self) {
        println!("{} says woof!", self.name);
    }
}

// Define a struct Car
struct Car {
    model: String,
    year: u32,
}

// Implement the Sound trait for Car
impl Sound for Car {
    fn make_sound(&self) {
        println!("The {} car makes a vroom vroom sound from {}.", self.model, self.year);
    }
}

fn main() {
    // Create instances of Dog and Car
    let my_dog = Dog { name: String::from("Buddy") };
    let my_car = Car { model: String::from("Toyota Camry"), year: 2022 };

    // Call the make_sound method for Dog
    my_dog.make_sound();

    // Call the make_sound method for Car
    my_car.make_sound();
}
