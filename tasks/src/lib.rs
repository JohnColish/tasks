#[cfg(feature="tasks")]
pub mod tasks;
pub mod task_info;
pub mod galaga;

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

pub fn describe_async_function(name: &str, params: &[&str], return_type: Option<&str>, description: &str) -> String {
    let return_type = return_type.unwrap_or("None");
    let mut params = params.join("\n        ");
    if params.is_empty() {params = "None".to_string();}
    format!("Create an async function with:

    Name: {name}

    Parameters:

        {params}

    ReturnType: 
    
        {return_type}

    This function must:

        {description}")
}

pub fn describe_addition(name: &str, description: &str) -> String {
    format!("Inside the {name} function:

    {description}")
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

pub fn describe_goal(expectation: &str) -> String {
    format!("After the code compiles:

    {expectation}")
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
macro_rules! make_game_info {
    ($test:ident, $sec_info:expr, $info:expr, $desc:expr, $expt:expr) => {
        pub fn $test() {
            println!("
----------------------------------------------------------------------

{}: {}

INFO: {}

TASK DESCRIPTION:

    {}

{}

----------------------------------------------------------------------
                ",
                stringify!($test)
                    .strip_suffix("_info").unwrap()
                    .replace("_", " "),
                $sec_info,
                $info,
                $desc,
                $expt
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
        tasks::Warship{cannons: 5, torpedoes: 10, speed: 100}.torpedo_check(),
        "This ship is stocked up and ready to go"
    );

    assert_stdout_eq!(
        tasks::Warship{cannons: 0, torpedoes: 1, speed: 25}.torpedo_check(),
        "This ship is low and slow"
    );

    assert_stdout_eq!(
        tasks::Warship{cannons: 51, torpedoes: 12, speed: 90}.torpedo_check(),
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
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_7_info'

    assert_eq!(tasks::gold_coins("Galapagos"), Some(50));
    assert_eq!(tasks::gold_coins("Madagascar"), Some(100));
    assert_eq!(tasks::gold_coins("Maldives"), Some(50));
    assert_eq!(tasks::gold_coins("Bahamas"), None);
    assert_eq!(tasks::gold_coins("Iceland"), None);
    assert_eq!(tasks::gold_coins("Fiji"), None);

    //----------------------------------------------------------------------
}, task6_1);

test_setup::make_test!(task6_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_1_info'
    assert_eq!(tasks::kms_this_week(), [5, 7, 10, 5, 7, 5, 10]);
    //----------------------------------------------------------------------
}, task6_2);

test_setup::make_test!(task6_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_2_info'
    assert_stdout_eq!(tasks::eggs_this_week(), "I got [3, 5, 2, 4, 4, 2, 6] eggs this week");
    //----------------------------------------------------------------------
}, task6_3);

test_setup::make_test!(task6_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_3_info'
    assert_eq!(tasks::high_scores(vec![12, 16, 14], 8), [12, 16, 14, 8]);
    assert_eq!(tasks::high_scores(vec![6, 23, 7, 12], 31), [6, 23, 7, 12, 31]);
    assert_eq!(tasks::high_scores(vec![8, 8, 8], 8), [8, 8, 8, 8]);
    //----------------------------------------------------------------------
}, task6_4);

test_setup::make_test!(task6_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_4_info'

    //----------------------------------------------------------------------
}, task6_5);

test_setup::make_test!(task6_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_5_info'

    //----------------------------------------------------------------------
}, task6_6);

test_setup::make_test!(task6_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_5_info'

    //----------------------------------------------------------------------
}, task6_7);

test_setup::make_test!(task6_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_5_info'

    //----------------------------------------------------------------------
}, task6_8);

test_setup::make_test!(task6_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task6_5_info'

    //----------------------------------------------------------------------
}, task6_8);







test_setup::make_game_test!(galaga_1, {}, task6_8);
test_setup::make_game_test!(galaga_2, {}, task6_8);
test_setup::make_game_test!(galaga_3, {}, task6_8);
test_setup::make_game_test!(galaga_4, {}, task6_8);
test_setup::make_game_test!(galaga_5, {}, task6_8);
test_setup::make_game_test!(galaga_6, {}, task6_8);
test_setup::make_game_test!(galaga_7, {}, task6_8);
test_setup::make_game_test!(galaga_8, {}, task6_8);