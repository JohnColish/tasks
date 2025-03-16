#[cfg(feature="tasks")]
pub mod tasks;
pub mod task_info;

pub fn rules() {
    std::process::Command::new("clear").status().unwrap();
    println!("
----------------------------------------------------------------------

RULES:
1. Never change a previous task after you've completed it.
2. Never prefix variables with _. Fix warnings in a different way.
3. Do not edit any other files in the project except for src/tasks.rs
4. Ensure there are no warnings in your solutions

Follow these rules to ensure your solutions are clean and correct!
----------------------------------------------------------------------
    ");
}

pub fn list(items: &[&str], numbered: bool) -> String {
    items.iter().enumerate().map(|(ix, i)| {
        let ix = ix+1;
        let li = if numbered { format!("{ix}. ")} else { "- ".to_string()};
        format!("        {}{}", li, i)
    }).collect::<Vec<_>>().join("\n\n")
}

pub fn examples(exs: &[(&str, &str)]) -> String {
    exs.iter().map(|(desc, exa)|
        format!("        - {desc}:\n\n            {exa}\n")
    ).collect::<Vec<_>>().join("\n")
}

pub fn describe_type(
    name: &str, description: &str, items: (bool, &[&str]), exas: &[(&str, &str)]
) -> String {
    let list = if items.1.is_empty() {"".to_string()} else {format!("\n\n{}", list(items.1, items.0))};
    let exas = if exas.is_empty() {"".to_string()} else {format!("\n\n{}", examples(exas))};
    format!("{name}:\n        {description}{list}{exas}\n")
}

pub fn describe_function(name: &str, params: &[&str], return_type: Option<&str>, description: &str) -> String {
    let return_type = return_type.unwrap_or("None");
    let mut params = params.join("\n        ");
    if params.is_empty() {params = "None".to_string();}
    format!("Create a function with:

    Name: {name}

    Parameters:

        {params}

    ReturnType: 
    
        {return_type}

    This function must:

        {description}")
}

pub fn describe_const(name: &str, value_type: &str, content: &str) -> String {
    format!("Create a const with:

    Name: {name}

    Type: {value_type}

    Value: {content}")
}

pub fn describe_structure(name: &str, fields: &[&str], _constructors: &[String], _methods: &[String]) -> String {
    let mut fields = fields.join("\n        ");
    if fields.is_empty() {fields = "None".to_string();}

    // let constructors = constructors.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    // let mut constructors = constructors.join("\n        ");
    // if constructors.is_empty() {constructors = "None".to_string();}

    // let methods = methods.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    // let mut methods = methods.join("\n        ");
    // if methods.is_empty() {methods = "None".to_string();}

    format!("Create a structure with:

    Name: {name}

    Fields:

        {fields}")
}

pub fn describe_enum(name: &str, fields: &[&str], _constructors: &[String], _methods: &[String]) -> String {
    let mut fields = fields.join("\n        ");
    if fields.is_empty() {fields = "None".to_string();}

    // let constructors = constructors.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    // let mut constructors = constructors.join("\n        ");
    // if constructors.is_empty() {constructors = "None".to_string();}

    // let methods = methods.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    // let mut methods = methods.join("\n        ");
    // if methods.is_empty() {methods = "None".to_string();}

    format!("Create a enum with:

    Name: {name}

    Variants:
        {fields}")
}

#[macro_export]
macro_rules! make_test_info {
    ($test:ident, $sec_info:expr, $info:expr, $desc:expr, $rest:expr, $hint:expr) => {
        pub fn $test() {
            let rest = $rest.unwrap_or("None");
            let hint = $hint.unwrap_or("None");
            println!("
----------------------------------------------------------------------

SECTION {}: {}

INFO:

    {}

TASK DESCRIPTION:

    {}

RESTRICTIONS:

    {}

HINT:

    {}

----------------------------------------------------------------------
                ",
                stringify!($test)
                    .strip_prefix("task").unwrap()
                    .strip_suffix("_info").unwrap()
                    .replace("_", "."),
                $sec_info,
                $info,
                $desc,
                rest,
                hint
            );
        }
    };
}

#[macro_export]
macro_rules! assert_stdout_eq {
    ($test:expr, $expected:expr) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stdout().unwrap();
		let mut clear = Vec::new();
        buf.read(&mut clear).unwrap();
        $test;
		print!("\n");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        assert_eq!(&output[..output.len()-1], $expected);
    }};
}

test_setup::make_test!(task1_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_1_info'

    assert_stdout_eq!(tasks::hello_world(), "Hello, World!");
    //----------------------------------------------------------------------
}, task1_2);

test_setup::make_test!(task1_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_2_info'

    assert_stdout_eq!(tasks::hello_everyone(), "Hello, Everyone!");
    //----------------------------------------------------------------------
}, task1_3);

test_setup::make_test!(task1_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_3_info'

    assert_stdout_eq!(tasks::hello_pet(), "Hello, Benji!");
    //----------------------------------------------------------------------
}, task1_4);

test_setup::make_test!(task1_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_4_info'

    assert_stdout_eq!(tasks::hello_user("David"), "Hi, my name is David");
    assert_stdout_eq!(tasks::hello_user("Alex"), "Hi, my name is Alex");
    //----------------------------------------------------------------------
}, task1_5);

test_setup::make_test!(task1_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_5_info'
    assert_stdout_eq!(tasks::hello_mitch(), "Hi, my name is Mitch");
    //----------------------------------------------------------------------
}, task1_6);

test_setup::make_test!(task1_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_6_info'
    assert_stdout_eq!(tasks::formal_hello("David", "Lightman"), "Hi, my name is David Lightman");
    assert_stdout_eq!(tasks::formal_hello("Alex", "Rogan"), "Hi, my name is Alex Rogan");
    //----------------------------------------------------------------------
}, task2_1);

// test_setup::make_test!(task1_7, {
//     //----------------------------------------------------------------------
//     //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
//     //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_7_info'
//     assert_stdout_eq!(tasks::hello_twice("David", "Lightman"), "Hi, my name is DavidHi, my name is David Lightman");
//     assert_stdout_eq!(tasks::hello_twice("Alex", "Rogan"), "Hi, my name is AlexHi, my name is Alex Rogan");
//     //----------------------------------------------------------------------
// }, task2_1);


test_setup::make_test!(task2_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_1_info'
    assert_stdout_eq!(tasks::say_age(17), "My age is 17");
    assert_stdout_eq!(tasks::say_age(19), "My age is 19");
    //----------------------------------------------------------------------
}, task2_2);

test_setup::make_test!(task2_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_2_info'

    let age: u8 = tasks::my_age();
    //----------------------------------------------------------------------
}, task2_3);

test_setup::make_test!(task2_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_3_info'

    assert_stdout_eq!(tasks::say_my_age(), format!("My age is {}", tasks::my_age()).as_str());
    //----------------------------------------------------------------------
}, task2_4);

test_setup::make_test!(task2_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_4_info'
    assert_eq!(tasks::add(1, 4), 5);
    assert_eq!(tasks::add(61, 42), 103);
    //----------------------------------------------------------------------
}, task2_5);

test_setup::make_test!(task2_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_5_info'
    assert_eq!(tasks::subtract(8, 4), 4);
    assert_eq!(tasks::subtract(61, 42), 19);
    //----------------------------------------------------------------------
}, task2_6);

test_setup::make_test!(task2_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_6_info'
    assert_stdout_eq!(tasks::jonah(), "Their name is Jonah");
    //----------------------------------------------------------------------
}, task2_7);

test_setup::make_test!(task2_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_7_info'
    assert_stdout_eq!(tasks::say_add(8, 4), "The result is 12");
    assert_stdout_eq!(tasks::say_add(61, 42), "The result is 103");
    //----------------------------------------------------------------------
}, task2_8);

test_setup::make_test!(task2_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_8_info'
    assert_stdout_eq!(
    	tasks::formal_greet("David", "Lightman", 10, 7),
    	"Hi, my name is David LightmanMy age is 17"
    );
    assert_stdout_eq!(
    	tasks::formal_greet("Alex", "Rogan", 10, 9),
    	"Hi, my name is Alex RoganMy age is 19"
    );
    //----------------------------------------------------------------------
}, task3_1);

test_setup::make_test!(task3_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_1_info'
    assert_eq!(tasks::is_awesome(), true);
    //----------------------------------------------------------------------
}, task3_2);

test_setup::make_test!(task3_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_2_info'
    assert_eq!(tasks::is_equal(8, 2), false);
    assert_eq!(tasks::is_equal(8, 4), false);
    assert_eq!(tasks::is_equal(8, 8), true);
    //----------------------------------------------------------------------
}, task3_3);

test_setup::make_test!(task3_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_3_info'
    assert_stdout_eq!(tasks::your_planet("mercury"), "About 77 million kilometers away.");
    assert_stdout_eq!(tasks::your_planet("venus"), "About 41 million kilometers away.");
    assert_stdout_eq!(tasks::your_planet("neptune"), "About 4.3 billion kilometers away.");
    assert_stdout_eq!(tasks::your_planet("ukraine"), "Undiscovered planet.");
    //----------------------------------------------------------------------
}, task3_4);

test_setup::make_test!(task3_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_4_info'
    assert_stdout_eq!(tasks::underwater(true), "I am underwater");
    assert_stdout_eq!(tasks::underwater(false), "Not underwater");
    //----------------------------------------------------------------------
}, task3_5);

test_setup::make_test!(task3_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_5_info'
    assert_stdout_eq!(tasks::say_is_equal(5, 5), "a is the same as b");
    assert_stdout_eq!(tasks::say_is_equal(10, 12), "a is not the same as b");
    //----------------------------------------------------------------------
}, task3_6);


test_setup::make_test!(task3_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_6_info'
    assert_eq!(tasks::power_status(true), false);
    assert_eq!(tasks::power_status(false), true);
    //----------------------------------------------------------------------
}, task3_7);


// test_setup::make_test!(task3_6, {
//     //----------------------------------------------------------------------
//     //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
//     //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_6_info'
//     assert_stdout_eq!(tasks::shoes_on(true, true), "Take your shoes off!");
//     assert_stdout_eq!(tasks::shoes_on(true, false), "Take your shoes off!");
//     assert_stdout_eq!(tasks::shoes_on(false, false), "Good kid!");
//     //----------------------------------------------------------------------
// }, task3_7);

test_setup::make_test!(task3_7, {
    assert_stdout_eq!(tasks::decide_battle(true, true), "Run!");
    assert_stdout_eq!(tasks::decide_battle(true, false), "Run!");
    assert_stdout_eq!(tasks::decide_battle(false, false), "Fight!");
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_7_info'
    //----------------------------------------------------------------------
}, task3_8);

test_setup::make_test!(task3_8, {
    assert_stdout_eq!(tasks::has_access(true, true), "You can enter!");
    assert_stdout_eq!(tasks::has_access(true, false), "Access denied.");
    assert_stdout_eq!(tasks::has_access(false, false), "Access denied.");
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_7_info'
    //----------------------------------------------------------------------
}, task4_1);

test_setup::make_test!(task4_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_1_info'

    let _test = tasks::Warship{cannons: 34, torpedoes: 76, speed: 100};
    //----------------------------------------------------------------------
}, task4_2);

test_setup::make_test!(task4_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_2_info'
    let test = tasks::create_warship();
    assert_eq!(test.cannons, 12);
    assert_eq!(test.torpedoes, 24);
    assert_eq!(test.speed, 100);
    //----------------------------------------------------------------------
}, task4_3);

test_setup::make_test!(task4_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_3_info'
    assert_stdout_eq!(
        tasks::cannon_count(tasks::Warship{cannons: 34, torpedoes: 76, speed: 100}),
        "My warship has 34 cannons left"
    );
    assert_stdout_eq!(
        tasks::cannon_count(tasks::Warship{cannons: 7, torpedoes: 76, speed: 100}),
        "My warship has 7 cannons left"
    );
    //----------------------------------------------------------------------
}, task4_4);


test_setup::make_test!(task4_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_4_info'
    let test = tasks::Warship::new(34);
    assert_eq!(test.speed, 34);
    assert_eq!(test.cannons, 12);
    assert_eq!(test.torpedoes, 24);

    let test = tasks::Warship::new(200);
    assert_eq!(test.speed, 200);
    assert_eq!(test.cannons, 12);
    assert_eq!(test.torpedoes, 24);
    //----------------------------------------------------------------------
}, task4_5);


test_setup::make_test!(task4_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_5_info'
    assert_stdout_eq!(
        tasks::Warship{cannons: 7, torpedoes: 2, speed: 100}.torpedo_check(),
        "I have 2 torpedoes left"
    );

    assert_stdout_eq!(
        tasks::Warship{cannons: 7, torpedoes: 76, speed: 100}.torpedo_check(),
        "I have 76 torpedoes left"
    );

    //----------------------------------------------------------------------
}, task4_6);

test_setup::make_test!(task4_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_6_info'
    assert_stdout_eq!(tasks::print_apples(1), "One apple");
    assert_stdout_eq!(tasks::print_apples(2), "Two apples");
    assert_stdout_eq!(tasks::print_apples(3), "Three apples");
    assert_stdout_eq!(tasks::print_apples(5), "More than three apples");
    assert_stdout_eq!(tasks::print_apples(94), "More than three apples");

    //----------------------------------------------------------------------
}, task4_7);

test_setup::make_test!(task4_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_7_info'

    assert_stdout_eq!(tasks::print_oranges(1), "One orange");
    assert_stdout_eq!(tasks::print_oranges(2), "Two oranges");
    assert_stdout_eq!(tasks::print_oranges(3), "Three oranges");
    assert_stdout_eq!(tasks::print_oranges(5), "5 oranges");
    assert_stdout_eq!(tasks::print_oranges(94), "94 oranges");
    //----------------------------------------------------------------------
}, task4_8);

test_setup::make_test!(task4_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_8_info'

    assert_stdout_eq!(
        tasks::Warship{cannons: 5, torpedoes: 10, speed: 100}.examine(),
        "This ship is stocked up and ready to go"
    );

    assert_stdout_eq!(
        tasks::Warship{cannons: 0, torpedoes: 1, speed: 24}.examine(),
        "This ship is low and slow"
    );

    assert_stdout_eq!(
        tasks::Warship{cannons: 51, torpedoes: 12, speed: 90}.examine(),
        "This ship is a mystery"
    );
    //----------------------------------------------------------------------
}, task5_1);

test_setup::make_test!(task5_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_1_info'

    let fighter = tasks::Spacecraft::Fighter{};
    let cargo = tasks::Spacecraft::Cargo{};
    let colonizer = tasks::Spacecraft::Colonizer{};
    let explorer = tasks::Spacecraft::Explorer{};

    //----------------------------------------------------------------------
}, task5_2);

test_setup::make_test!(task5_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_2_info'

    let test = tasks::get_fighter();

    //----------------------------------------------------------------------
}, task5_3);

test_setup::make_test!(task5_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_3_info'

    assert_eq!(tasks::Spacecraft::Fighter{}.is_passenger(), true);
    assert_eq!(tasks::Spacecraft::Cargo{}.is_passenger(), false);
    assert_eq!(tasks::Spacecraft::Colonizer{}.is_passenger(), true);
    assert_eq!(tasks::Spacecraft::Explorer{}.is_passenger(), true);

    //----------------------------------------------------------------------
}, task5_4);

test_setup::make_test!(task5_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_3_info'

    let invisibility = tasks::Potion::Invisibility{ strength: 12 };
    let healing = tasks::Potion::Healing{ strength: 14 };
    let poison = tasks::Potion::Poison{ strength: 8 };

    //----------------------------------------------------------------------
}, task5_5);

test_setup::make_test!(task5_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_3_info'

    assert_stdout_eq!(
        tasks::Potion::Invisibility{strength: 51}.say_strength(),
        "You found an Invisibility potion with 51 strength"
    );
    assert_stdout_eq!(
        tasks::Potion::Healing{strength: 15}.say_strength(),
        "You found a Healing potion with 15 strength"
    );
    assert_stdout_eq!(
        tasks::Potion::Poison{strength: 12}.say_strength(),
        "You found a Poison potion with 12 strength"
    );

    //----------------------------------------------------------------------
}, task5_6);

test_setup::make_test!(task5_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_3_info'

    assert_stdout_eq!(
        tasks::Potion::Invisibility{strength: 51}.poison_strength(),
        "Invisibility potion"
    );
    assert_stdout_eq!(
        tasks::Potion::Healing{strength: 15}.poison_strength(),
        "Healing potion"
    );
    assert_stdout_eq!(
        tasks::Potion::Poison{strength: 12}.poison_strength(),
        "12 strength poison potion"
    );

    //----------------------------------------------------------------------
}, task5_7);

test_setup::make_test!(task5_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_7_info'

    assert_stdout_eq!(tasks::my_trucks(), "I have 4 trucks");
    assert_eq!(tasks::my_trucks(), 4);

    //----------------------------------------------------------------------
}, task5_8);


test_setup::make_test!(task5_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_8_info'

    assert_stdout_eq!(tasks::eat_cookie(), "Who is eating all the cookies?");

    //----------------------------------------------------------------------
}, task6_1);

test_setup::make_test!(task6_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_1_info'

    assert_stdout_eq!(tasks::level_up(), "Level Up! Level is 13");

    //----------------------------------------------------------------------
}, task6_2);

test_setup::make_test!(task6_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_2_info'

    assert_stdout_eq!(tasks::burn_candle(23), "I burned a candle and now I have 22 left");
    assert_stdout_eq!(tasks::burn_candle(4), "I burned a candle and now I have 3 left");
    assert_stdout_eq!(tasks::burn_candle(112), "I burned a candle and now I have 111 left");

    //----------------------------------------------------------------------
}, task6_3);

test_setup::make_test!(task6_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_3_info'
    assert_eq!(tasks::kms_this_week(), [5, 7, 10, 5, 7, 5, 10]);
    //----------------------------------------------------------------------
}, task6_4);

test_setup::make_test!(task6_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_4_info'
    assert_stdout_eq!(tasks::eggs_this_week(), "I got [3, 5, 2, 4, 4, 2, 6] eggs this week");
    //----------------------------------------------------------------------
}, task6_5);

test_setup::make_test!(task6_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_5_info'
    assert_eq!(tasks::high_scores(vec![12, 16, 14], 8), [12, 16, 14, 8]);
    assert_eq!(tasks::high_scores(vec![6, 23, 7, 12], 31), [6, 23, 7, 12, 31]);
    assert_eq!(tasks::high_scores(vec![8, 8, 8], 8), [8, 8, 8, 8]);
    //----------------------------------------------------------------------
}, task6_6);

test_setup::make_test!(task6_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_6_info'
    assert_eq!(tasks::disqualified(vec![12, 16, 14]), [12, 16]);
    assert_eq!(tasks::disqualified(vec![6, 23, 7, 12]), [6, 23, 7]);
    assert_eq!(tasks::disqualified(vec![8, 8, 8]), [8, 8]);
    //----------------------------------------------------------------------
}, task6_7);

test_setup::make_test!(task6_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_7_info'
    assert_stdout_eq!(tasks::mission(), "Mission, Passengers: (\"Apollo 11\", 3)");
    //----------------------------------------------------------------------
}, task6_8);

test_setup::make_test!(task6_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_8_info'
    assert_stdout_eq!(tasks::sports_car(("Ferrari", 220)), "I have a Ferrari with top speed 220");
    assert_stdout_eq!(tasks::sports_car(("Porsche", 200)), "I have a Porsche with top speed 200");
    assert_stdout_eq!(tasks::sports_car(("Apollo", 180)), "I have a Apollo with top speed 180");
    //----------------------------------------------------------------------
}, task7_1);

test_setup::make_test!(task7_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_1_info'

    assert_eq!(tasks::gold_coins("Galapagos"), Some(50));
    assert_eq!(tasks::gold_coins("Madagascar"), Some(100));
    assert_eq!(tasks::gold_coins("Maldives"), Some(50));
    assert_eq!(tasks::gold_coins("Bahamas"), None);
    assert_eq!(tasks::gold_coins("Iceland"), None);
    assert_eq!(tasks::gold_coins("Fiji"), None);


    //----------------------------------------------------------------------
}, task7_2);

test_setup::make_test!(task7_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_2_info'
    use std::collections::HashMap;
    assert_eq!(tasks::soccer_player(), HashMap::from([(12, 3)]));
    //----------------------------------------------------------------------
}, task7_3);

test_setup::make_test!(task7_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_3_info'
    use std::collections::HashMap;
    assert_stdout_eq!(
        tasks::salmon_count(HashMap::from([("Salmon", 3)])), 
        "Today we caught Some(3) salmon!"
    );

    assert_stdout_eq!(
        tasks::salmon_count(HashMap::from([("Salmon", 12)])), 
        "Today we caught Some(12) salmon!"
    );

    assert_stdout_eq!(
        tasks::salmon_count(HashMap::from([("Salmon", 77)])), 
        "Today we caught Some(77) salmon!"
    );
    //----------------------------------------------------------------------
}, task7_4);

test_setup::make_test!(task7_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_4_info'

    assert_eq!(tasks::make_n_drink(8, 3), (7, 4));
    assert_eq!(tasks::make_n_drink(3, 3), (2, 4));
    assert_eq!(tasks::make_n_drink(12, 2), (11, 3));

    //----------------------------------------------------------------------
}, task7_5);

test_setup::make_test!(task7_5, {
     //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_5_info'
    //----------------------------------------------------------------------
}, task7_6);

test_setup::make_test!(task7_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_6_info'
    assert_stdout_eq!(
        tasks::throw_snowballs(6), 
        "Snowball!Snowball!Snowball!Snowball!Snowball!Snowball!Snowball!"
    );
    assert_stdout_eq!(
        tasks::throw_snowballs(1), 
        "Snowball!Snowball!"
    );
    assert_stdout_eq!(
        tasks::throw_snowballs(3), 
        "Snowball!Snowball!Snowball!Snowball!"
    );
    //----------------------------------------------------------------------
}, task7_7);

test_setup::make_test!(task7_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_7_info'
    assert_stdout_eq!(
        tasks::feed_capybara(1), 
        "I have fed 0 capybaras!I have fed 1 capybaras!"
    );
    assert_stdout_eq!(
        tasks::feed_capybara(2), 
        "I have fed 0 capybaras!I have fed 1 capybaras!I have fed 2 capybaras!"
    );
    assert_stdout_eq!(
        tasks::feed_capybara(3), 
        "I have fed 0 capybaras!I have fed 1 capybaras!I have fed 2 capybaras!I have fed 3 capybaras!"
    );
    //----------------------------------------------------------------------
}, task7_8);

test_setup::make_test!(task7_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task7_8_info'
    assert_stdout_eq!(
        tasks::feed_penguin(3), 
        "I have fed 1 penguins!I have fed 2 penguins!I have fed 3 penguins!"
    );
    assert_stdout_eq!(
        tasks::feed_penguin(1), 
        "I have fed 1 penguins!"
    );
    assert_stdout_eq!(
        tasks::feed_penguin(4), 
        "I have fed 1 penguins!I have fed 2 penguins!I have fed 3 penguins!I have fed 4 penguins!"
    );
    //----------------------------------------------------------------------
}, task8_1);

test_setup::make_test!(task8_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_1_info'
    assert_stdout_eq!(tasks::if_west(true), "Going west!");
    assert_stdout_eq!(tasks::if_west(false), "Not going west!");
    //----------------------------------------------------------------------
}, task8_2);

test_setup::make_test!(task8_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_2_info'
    assert_stdout_eq!(tasks::take_order("water"), "Here is your water!");
    assert_stdout_eq!(tasks::take_order("soda"), "Here is your soda!");
    assert_stdout_eq!(tasks::take_order("juice"), "Here is your juice!");
    assert_stdout_eq!(tasks::take_order("milk"), "We don't have that here.");
    //----------------------------------------------------------------------
}, task8_3);

test_setup::make_test!(task8_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_3_info'
    assert_stdout_eq!(tasks::give_gift(true), "Happy Birthday! I got you trucks");
    assert_stdout_eq!(tasks::give_gift(false), "Happy Birthday! I got you candy");
    //----------------------------------------------------------------------
}, task8_4);

test_setup::make_test!(task8_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_4_info'
    assert_stdout_eq!(tasks::hunting_rifle(Some(12)), "My gun has 12 bullets left");
    assert_stdout_eq!(tasks::hunting_rifle(Some(24)), "My gun has 24 bullets left");
    assert_stdout_eq!(tasks::hunting_rifle(Some(4)), "My gun has 4 bullets left");
    assert_stdout_eq!(tasks::hunting_rifle(None), "My gun has no bullets left");
    //----------------------------------------------------------------------
}, task8_5);

test_setup::make_test!(task8_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_5_info'
    assert_eq!(tasks::mission_log(), String::from("Log: Oxygen levels stable. Aliens unknown."));
    //----------------------------------------------------------------------
}, task8_6);

test_setup::make_test!(task8_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_6_info'
    let a: u8 = 23;
    assert_stdout_eq!(tasks::dogs_in_park(&a), "There are 23 dogs playing in the park!");
    let a: u8 = 3;
    assert_stdout_eq!(tasks::dogs_in_park(&a), "There are 3 dogs playing in the park!");
    let a: u8 = 9;
    assert_stdout_eq!(tasks::dogs_in_park(&a), "There are 9 dogs playing in the park!");
    //----------------------------------------------------------------------
}, task8_7);

test_setup::make_test!(task8_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_7_info'
    let mut a: u8 = 9;
    assert_stdout_eq!(tasks::pop_balloon(&mut a), "POP! There are now 8 balloons left.");
    let mut a: u8 = 30;
    assert_stdout_eq!(tasks::pop_balloon(&mut a), "POP! There are now 29 balloons left.");
    let mut a: u8 = 64;
    assert_stdout_eq!(tasks::pop_balloon(&mut a), "POP! There are now 63 balloons left.");
    //----------------------------------------------------------------------
}, task8_8);

test_setup::make_test!(task8_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task8_8_info'
    assert_stdout_eq!(tasks::knight_move(), "K to C7!");
    //----------------------------------------------------------------------
}, task9_1);

test_setup::make_test!(task9_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_1_info'
    use rot13::rot13;
    let a = rot13("John smells.");

    assert_stdout_eq!(
        tasks::encode_message("John smells."), 
        format!("I have a secret to tell you. {a}")
    );
    let a = rot13("If you're reading this, send help.");
    assert_stdout_eq!(
        tasks::encode_message("If you're reading this, send help."), 
        format!("I have a secret to tell you. {a}")
    );
    let a = rot13("Definitely not a bug, it's a feature.");
    assert_stdout_eq!(
        tasks::encode_message("Definitely not a bug, it's a feature."), 
        format!("I have a secret to tell you. {a}")
    );
    //----------------------------------------------------------------------
}, task9_2);

test_setup::make_test!(task9_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_2_info'    
    tasks::french_word();
    //----------------------------------------------------------------------
}, task9_3);


test_setup::make_test!(task9_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_3_info'
    assert_eq!(tasks::cook_chicken(114), Err(String::from("Gross! Chicken is raw.")));
    assert_eq!(tasks::cook_chicken(140), Err(String::from("Gross! Chicken is raw.")));
    assert_eq!(tasks::cook_chicken(68), Err(String::from("Gross! Chicken is raw.")));

    assert_eq!(tasks::cook_chicken(165), Ok(String::from("Chicken is cooked!")));
    assert_eq!(tasks::cook_chicken(170), Ok(String::from("Chicken is cooked!")));
    assert_eq!(tasks::cook_chicken(188), Ok(String::from("Chicken is cooked!")));
    //----------------------------------------------------------------------
}, task9_4);

test_setup::make_test!(task9_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_4_info'
    assert_eq!(tasks::borrow_book("Dune"), Err(String::from("Book not available")));
    assert_eq!(tasks::borrow_book("The Lord of the Rings"), Err(String::from("Book not available")));
    assert_eq!(tasks::borrow_book("Stranger in a Strange Land"), Err(String::from("Book not available")));
    assert_eq!(tasks::borrow_book("Pride and Prejudice"), Err(String::from("Book not available")));

    assert_eq!(tasks::borrow_book("Starman Jones"), Ok(()));
    //----------------------------------------------------------------------
}, task9_5);


test_setup::make_test!(task9_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_5_info'
    assert_stdout_eq!(tasks::reserve_book(), "Error: Book not available");
    //----------------------------------------------------------------------
}, task9_6);

test_setup::make_test!(task9_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_6_info'
    assert_stdout_eq!(tasks::eat_chicken(), "Yummy, yummy! Chicken is cooked!");
    //----------------------------------------------------------------------
}, task9_7);

test_setup::make_test!(task9_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_7_info'
    assert_stdout_eq!(tasks::brew_coffee(), "");
    //----------------------------------------------------------------------
}, task9_8);

test_setup::make_test!(task9_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task9_8_info'
    assert_stdout_eq!(tasks::serve_customer(), "");
    //----------------------------------------------------------------------
}, task10_1);


test_setup::make_test!(task10_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_1_info'
    assert_stdout_eq!(tasks::tell_mark("indent your code"), "Hey Mark, I gotta tell you: indent your code.");
    assert_stdout_eq!(tasks::tell_mark("Tuesday"), "Hey Mark, I gotta tell you: Tuesday.");
    assert_stdout_eq!(tasks::tell_mark(104), "Hey Mark, I gotta tell you: 104.");
    assert_stdout_eq!(tasks::tell_mark("you stink"), "Hey Mark, I gotta tell you: you stink.");
    //----------------------------------------------------------------------
}, task10_2);

test_setup::make_test!(task10_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_2_info'

    let reindeer = tasks::Reindeer::<&str>::new(String::from("Rudolph"), "three", String::from("Chocolates"));
    assert_eq!(reindeer.name, String::from("Rudolph"));
    assert_eq!(reindeer.age, "three");
    assert_eq!(reindeer.favorite_candy, String::from("Chocolates"));

    let reindeer = tasks::Reindeer::<u8>::new(String::from("Prancer"), 2, String::from("Lollipops"));
    assert_eq!(reindeer.name, String::from("Prancer"));
    assert_eq!(reindeer.age, 2);
    assert_eq!(reindeer.favorite_candy, String::from("Lollipops"));
    //----------------------------------------------------------------------
}, task10_3);

test_setup::make_test!(task10_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_3_info'

    let a = tasks::MysteryBox::<String>::new(Some(String::from("Plastic Cups")));
    let a = tasks::MysteryBox::<String>::new(Some(String::from("Gummy Bears")));
    let a = tasks::MysteryBox::<String>::new(Some(String::from("Analog Clocks")));
    let a = tasks::MysteryBox::<String>::new(Some(String::from("Swordfish Toes")));

    let a = tasks::MysteryBox::<String>::new(None);
    //----------------------------------------------------------------------
}, task10_4);

test_setup::make_test!(task10_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_4_info'

    let w = tasks::WashingMachine;
    let d = tasks::DryingMachine;

    use crate::tasks::Machine;

    assert_stdout_eq!(w.start(), "Starting machine....");
    assert_stdout_eq!(d.start(), "Starting machine....");

    //----------------------------------------------------------------------
}, task10_5);

test_setup::make_test!(task10_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_5_info'
    let w = tasks::WashingMachine;
    let d = tasks::DryingMachine;

    assert_stdout_eq!(tasks::clean_laundry(w, d), "Starting machine....Starting machine....");

    //----------------------------------------------------------------------
}, task10_6);

test_setup::make_test!(task10_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_6_info'

    let w = tasks::WashingMachine;
    let d = tasks::DryingMachine;
    let o = tasks::Oven;

    use crate::tasks::Machine;

    assert_stdout_eq!(w.start(), "Starting machine....");
    assert_stdout_eq!(d.start(), "Starting machine....");
    assert_stdout_eq!(o.start(), "Preheating to 450f....");
    //----------------------------------------------------------------------
}, task10_7);

test_setup::make_test!(task10_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_7_info'

    let fishy = tasks::Fish{specie: String::from("Rainbow Trout"), scales: 100};

    let (fishy, fishist) = tasks::fish_duplicater(fishy);
    //----------------------------------------------------------------------
}, task10_8);

test_setup::make_test!(task10_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_8_info'
    assert_stdout_eq!(tasks::double_values(vec![2, 4, 6, 8]), "481216");

    //----------------------------------------------------------------------
}, task10_9);

test_setup::make_test!(task10_9, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task10_9_info'
    assert_eq!(tasks::increase_rations(vec![2, 4, 6, 8]), vec![4, 8, 12, 16]);
    //----------------------------------------------------------------------
}, task10_9);
