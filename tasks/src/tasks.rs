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

pub struct Tombstone {
    pub birth_year: u8,
    pub death_year: u8,
}

pub fn print_tombstone(tombstone: &Tombstone) {
    print!("{}-{}", tombstone.birth_year, tombstone.death_year);
}

pub struct DriversLicense {
    pub issued: u8,
    pub expires: u8,
}

impl DriversLicense {
    pub fn new(issued: u8) -> DriversLicense {
        DriversLicense{issued: issued, expires: issued+8}
    }
    pub fn print(self: &DriversLicense) {
        print!("Issued on {}, Expires on {}", self.issued, self.expires);
    }
}

pub struct FullName {
    pub first: String,
    pub last: String,
}

impl FullName {
    pub fn new(first: &str, last: &str) -> FullName {
        FullName{first: first.to_string(), last: last.to_string()}
    }
    pub fn print(self: &FullName) {
        print!("{} {}", self.first, self.last);
    }
    pub fn hello(self: &FullName) {
        formal_hello(&self.first, &self.last);
    }
}

pub fn print_age(age: u8) {
    match age {
        10 => print!("You are ten years old!"),
        20 => print!("You are the same age as me!"),
        _ => print!("I don't know how old you are but you are alive!"),
    }
}

pub fn examine_tombstone(tombstone: &Tombstone) {
    match tombstone {
        Tombstone{birth_year: 12, death_year: 92} => {
            print!("He was born in the year '12 and lived 80 years!");
        },
        Tombstone{birth_year: _, death_year: 30} => {
            print!("I don't know when they were born but they died in the year '30");
        },
        Tombstone{birth_year: _, death_year: year} => {
            print!("I don't know when they were born but they died in the year '{}", year);
        },
    }
}
