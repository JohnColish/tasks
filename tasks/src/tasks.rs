pub fn hello_world() {
	print!("Hello, World!")
}

pub fn hello_everyone() {
	print!("Hello, Everyone!");
}

pub fn hello_pet() {
	print!("Hello, {}!", "Benji");
}

pub fn hello_user(f: &str) {
	print!("Hi, my name is {}", f);
}

pub fn hello_mitch() {
	hello_user("Mitch");
}

pub fn formal_hello(f: &str, l: &str) {
	print!("Hi, my name is {} {}", f, l);
}

pub fn hello_twice(f: &str, l: &str) {
	hello_user(f);
	formal_hello(f, l);
}


pub fn say_age(age: u8) {
	print!("My age is {}", age);
}

pub fn my_age() -> u8 {
	20
}

pub fn say_my_age() {
	say_age(my_age());
}

pub fn add(a: u8, b: u8) -> u8 {
	a + b
}


pub fn subtract(a: u8, b: u8) -> u8 {
	a - b
}

pub fn jonah() {
    let jonah: &str = "Jonah";
    print!("Their name is {}", jonah)
}

pub fn say_add(a: u8, b: u8) {
    let result = add(a, b);
    print!("The result is {}", result);
}

pub fn formal_greet(f: &str, l: &str, a: u8, b: u8) {
	formal_hello(f, l);
	say_age(add(a, b));
}

pub fn is_awesome() -> bool {
    true
}

pub fn is_equal(a: u8, b: u8) -> bool {
    a == b
}

pub fn your_planet(planet: &str) {
    match planet {
        "mercury" => print!("About 77 million kilometers away."),
        "venus" => print!("About 41 million kilometers away."),
        "neptune" => print!("About 4.3 billion kilometers away."),
        _ => print!("Undiscovered planet.")
    };
}

pub fn underwater(is_underwater: bool) {
    match is_underwater {
        true => print!("I am underwater"),
        false => print!("Not underwater")
    }
}

pub fn say_is_equal(a: u8, b: u8) {
    match a == b {
        true => print!("a is the same as b"),
        false => print!("a is not the same as b")
    }
}

pub fn power_status(is_active: bool) -> bool {
    !is_active
}

pub fn decide_battle(is_dragon_sleeping: bool, is_giant_away: bool) {
    match is_dragon_sleeping || is_giant_away {
        true => print!("Run!"),
        false => print!("Fight!")
    }
}

pub fn has_access(has_pass: bool, has_permission: bool) {
    match has_pass && has_permission {
        true => print!("You can enter!"),
        false => print!("Access denied.")
    }
}

pub fn is_not_equal(a: u8, b: u8) -> bool {
    !is_equal(a, b)
}

pub fn is_not_equal_again(a: u8, b: u8) -> bool {
    a != b
}

pub fn shoes_on(left: bool, right: bool) {
    match left || right {
        true => print!("Take your shoes off!"),
        false => print!("Good kid!"),
    } }

pub fn ready_to_go(coat: bool, shoes: bool) {
    match coat && shoes {
        true => print!("Ready to go!"),
        false => print!("Not ready to go yet!"),
    }
}

pub fn ready_to_play(shoes: bool, is_hot: bool, coat: bool) -> bool {
    shoes && (is_hot || coat)
}

pub struct Warship {
    pub cannons: u8,
    pub torpedoes: u8,
    pub speed: u8
}

impl Warship {
    pub fn new(max_speed: u8) -> Warship {
        Warship {
            cannons: 12,
            torpedoes: 24,
            speed: max_speed,
        }
    }

    pub fn torpedo_check(self: Warship) {
        print!("I have {} torpedoes left", self.torpedoes)
    }
}

pub fn create_warship() -> Warship {
    Warship {
        cannons: 12,
        torpedoes: 24,
        speed: 100,
    }
}

pub fn cannon_count(warship: Warship) {
    print!("My warship has {} cannons left", warship.cannons)
}

pub fn print_apples(apples: u8) {
    match apples {
        1 => print!("One apple"),
        2 => print!("Two apples"),
        3 => print!("Three apples"),
        _ => print!("More than three apples"),
    }
}

pub fn print_oranges(oranges: u8) {
    match oranges {
        1 => print!("One orange"),
        2 => print!("Two oranges"),
        3 => print!("Three oranges"),
        count => print!("{} oranges", count),
    }
}

pub enum Spacecraft {
    Fighter {},
    Cargo {},
    Colonizer {},
    Explorer {}
}

pub fn get_fighter() -> Spacecraft {
    Spacecraft::Fighter{}
}

impl Spacecraft {
    pub fn is_passenger(self: Spacecraft) -> bool {
        match self {
            Spacecraft::Fighter{} => true,
            Spacecraft::Cargo{} => false,
            Spacecraft::Colonizer{} => true,
            Spacecraft::Explorer{} => true,
        }
    }
}

pub enum Potion {
    Invisibility { strength: u8 },
    Healing { strength: u8 },
    Poison { strength: u8 }
}

impl Potion {
    pub fn say_strength(self: Potion) {
        match self {
            Potion::Invisibility {strength: x} => {
                print!("You found an Invisibility potion with {x} strength");
            },
            Potion::Healing {strength: x} => {
                print!("You found a Healing potion with {x} strength");
            },
            Potion::Poison {strength: x} => {
                print!("You found a Poison potion with {x} strength");
            }
        }
    }

    pub fn poison_strength(self: Potion) {
        match self {
            Potion::Invisibility {strength: _} => {
                print!("Invisibility potion");
            },
            Potion::Healing {strength: _} => {
                print!("Healing potion");
            },
            Potion::Poison {strength: x} => {
                print!("{x} strength poison potion");
            }
        }
    }
}

pub fn my_trucks() -> u8 {
    let trucks = 4;
    print!("I have {} trucks", trucks);
    return trucks
}

pub fn gold_coins(island: &str) -> Option<u8> {
    match island {
        "Galapagos" => Some(50),
        "Madagascar" => Some(100),
        "Maldives" => Some(50),
        _ => None,
    }
}

pub fn kms_this_week() -> Vec<u8> {
    vec![5, 7, 10, 5, 7, 5, 10]
}

pub fn eggs_this_week() {
    let eggs: Vec<u8> = vec![3, 5, 2, 4, 4, 2, 6];
    print!("I got {:?} eggs this week", eggs);
}
