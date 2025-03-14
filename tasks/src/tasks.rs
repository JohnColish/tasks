use std::collections::HashMap;

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
}

const COOKIES_LEFT: u8 = 2;

pub fn eat_cookie() {
    match COOKIES_LEFT < 3 {
        true => print!("Who is eating all the cookies?"),
        false => print!("There are so many cookies. I will eat one!")
    }
}

pub fn level_up() {
    let mut player_level: u8 = 12;
    player_level = player_level + 1;
    print!("Level Up! Level is {}", player_level);
}

pub fn burn_candle(mut candles: u8) {
    candles = candles - 1;
    print!("I burned a candle and now I have {} left", candles);
}

pub fn kms_this_week() -> Vec<u8> {
    vec![5, 7, 10, 5, 7, 5, 10]
}

pub fn eggs_this_week() {
    let eggs: Vec<u8> = vec![3, 5, 2, 4, 4, 2, 6];
    print!("I got {:?} eggs this week", eggs);
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

pub fn high_scores(mut scores: Vec<u8>, new_score: u8) -> Vec<u8> {
    scores.push(new_score);
    return scores
}

pub fn disqualified(mut scores: Vec<u8>) -> Vec<u8> {
    scores.pop();
    return scores
}

pub fn mission() {
    let moon_mission: (&str, u8) = ("Apollo 11", 3);
    print!("Mission, Passengers: {:?}", moon_mission);
}

pub fn sports_car(car: (&str, u8)) {
    print!("I have a {} with top speed {}", car.0, car.1)
}

pub fn soccer_player() -> HashMap<u8, u8> {
    let mut players: HashMap<u8, u8> = HashMap::new();
    players.insert(12, 3);
    return players
}

pub fn salmon_count(todays_catch: HashMap<&str, u8>) {
    let salmon = todays_catch.get("Salmon");
    print!("Today we caught {:?} salmon!", salmon);
}

pub fn make_n_drink(mut oranges: u8, mut orange_juice: u8) -> (u8, u8) {
    oranges -= 1;
    orange_juice += 1;

    (oranges, orange_juice)
}

pub fn car_horn() {
    loop {
        print!("BEEP!");
    }
}

pub fn throw_snowballs(snowballs: u8) {
    for _snowball in 0..=snowballs {
        print!("Snowball!");
    }
}

pub fn feed_capybara(capybaras: u8) {
    for capybara in 0..=capybaras {
        print!("I have fed {} capybaras!", capybara);
    }
}

pub fn feed_penguin(penguins: u8) {
    for penguin in 1..=penguins {
        print!("I have fed {} penguins!", penguin);
    }
}

pub fn if_west(west: bool) {
    if west {
        print!("Going west!");
    } else {
        print!("Not going west!");
    }
}

pub fn take_order(request: &str) {
    if request == "water" {
        print!("Here is your water!");
    } else if request == "soda" {
        print!("Here is your soda!");
    } else if request == "juice" {
        print!("Here is your juice!");
    } else {
        print!("We don't have that here.");
    }
}

pub fn give_gift(toys_in_stock: bool) {
    let gift: &str = if toys_in_stock {
        "trucks"
    } else {
        "candy"
    };

    print!("Happy Birthday! I got you {}", gift);
}

pub fn hunting_rifle(magazine: Option<u8>) {
    if let Some(mag) = magazine {
        print!("My gun has {} bullets left", mag);
    } else {
        print!("My gun has no bullets left");
    }
}

pub fn mission_log() -> String {
    String::from("Log: Oxygen levels stable. Aliens unknown.")
}

pub fn dogs_in_park(dogs: &u8) {
    print!("There are {dogs} dogs playing in the park!");
}

pub fn pop_balloon(balloons: &mut u8) {
    *balloons -= 1;
    print!("POP! There are now {balloons} balloons left.")
}

pub fn knight_move() {
    let piece: char = 'K';
    print!("{} to C7!", piece);
}

pub fn encode_message(message: &str) {
    use rot13::rot13;
    print!("I have a secret to tell you. {}", rot13(message));
}

use random_word::Lang;

pub fn french_word() {
    let my_word = random_word::gen(Lang::Fr);
    print!("I'm so French. {}!", my_word);
}

use tokio::time::{sleep, Duration};

pub async fn brew_coffee() {
    print!("Brewing coffee...");
    sleep(Duration::from_secs(3)).await;
    print!("Coffee browed!");
}

pub async fn serve_customer() {
    brew_coffee().await;
    print!("Here's a fresh coffee!");
}

pub fn cook_chicken(temp: u8) -> Result<String, String> {
    match temp >= 165 {
        true => Ok(String::from("Chicken is cooked!")),
        false => Err(String::from("Gross! Chicken is raw."))
    }
}

pub fn borrow_book(title: &str) -> Result<(), String> {
    match title == "Starman Jones" {
        true => Ok(()),
        false => Err(String::from("Book not available"))
    }
}

pub fn reserve_book() {
    let my_book = "The Hobbit";

    match borrow_book(my_book) {
        Ok(_) => print!("Successfully reserved {}", my_book),
        Err(err) => print!("Error: {}", err)
    }
}

pub fn eat_chicken() -> Result<(), String> {
    let chicken = cook_chicken(170)?;

    print!("Yummy, yummy! {}", chicken);

    Ok(())
}

pub fn tell_mark<T: std::fmt::Display>(message: T) {
    print!("Hey Mark, I gotta tell you: {}.", message);
}

pub struct Reindeer<T> {
    pub name: String,
    pub age: T,
    pub favorite_candy: String
}

impl<T> Reindeer<T> {
    pub fn new(name: String, age: T, favorite_candy: String) -> Reindeer<T> {
        Reindeer {
            name,
            age,
            favorite_candy
        }
    }
}

pub enum MysteryBox<T> {
    Contains(T),
    Empty
}

impl<T> MysteryBox<T> {
    pub fn new(item: Option<T>) -> MysteryBox<T> {
        match item {
            Some(content) => MysteryBox::Contains(content),
            None => MysteryBox::Empty
        }
    }
}

pub struct WashingMachine;
pub struct DryingMachine;
pub struct Oven;

pub trait Machine {
    fn start(&self) {
        print!("Starting machine....");
    }
}

impl Machine for WashingMachine {}
impl Machine for DryingMachine {}

impl Machine for Oven {
    fn start(&self) {
        print!("Preheating to 450f....");
    }
}

pub fn clean_laundry(washer: WashingMachine, dryer: DryingMachine) {
    washer.start();
    dryer.start();
}

#[derive(Clone)]
pub struct Fish {
    pub specie: String,
    pub scales: u8
}

pub fn fish_duplicater(fish: Fish) -> (Fish, Fish) {
    let duplicate = fish.clone();
    (fish, duplicate)
}

pub fn double_values(values: Vec<u8>) {
    for val in values.iter() {
        print!("{}", val * 2);
    }
}

pub fn increase_rations(rations: Vec<u8>) -> Vec<u8> {
    rations.into_iter().map(|x| x * 2).collect()
}