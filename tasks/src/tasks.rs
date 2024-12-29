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

pub fn say_add(a: u8, b: u8) {
	let result: u8 = add(a, b);
	print!("The result was {}", result);
}

pub fn subtract(a: u8, b: u8) -> u8 {
	a - b
}

pub fn add_three(a: u8, b: u8, c: u8) -> u8 {
	let d: u8 = add(a, b);
	add(d, c)
}

pub fn formal_greet(f: &str, l: &str, a: u8, b: u8) {
	formal_hello(f, l);
	say_age(add(a, b));
}

pub fn say_bool(b: bool) {
    print!("My boolean is: {}", b);
}

pub fn is_equal(a: u8, b: u8) -> bool {
    a == b
}

pub fn say_is_equals(a: u8, b: u8) {
    match is_equal(a, b) {
        true => print!("Values are equal"),
        false => print!("Values are not equal"),
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


pub struct DriversLicense {
    pub issued: u8,
    pub expires: u8,
}

pub fn my_new_dl() -> DriversLicense {
    DriversLicense{issued: 4, expires: 12}
}

pub fn print_drivers_license(a_dl: DriversLicense) {
    print!("{}-{}", a_dl.issued, a_dl.expires);
}

impl DriversLicense {
    pub fn new(issued: u8) -> DriversLicense {
        DriversLicense{issued, expires: issued+8}
    }

    pub fn is_valid(self: DriversLicense, current_year: u8) -> bool {
        current_year >= self.issued && current_year < self.expires
    }

    pub fn examine(self: DriversLicense) {
        match self {
            DriversLicense{issued: 20, expires: 28} => {
                print!("Hey that's my drivers license");
            },
            DriversLicense{issued: 22, expires: _} => {
                print!("Drivers license issued during covid");
            },
            DriversLicense{issued, expires: 24} => {
                print!("Expires on the 24 and was issued on {}", issued);
            },
            _ => print!("No comment")
        }
    }
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
