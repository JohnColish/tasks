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

pub fn describe_structure(name: &str, fields: &[&str], constructors: &[String], methods: &[String]) -> String {
    let mut fields = fields.join("\n        ");
    if fields.is_empty() {fields = "None".to_string();}

    let constructors = constructors.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    let mut constructors = constructors.join("\n        ");
    if constructors.is_empty() {constructors = "None".to_string();}

    let methods = methods.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    let mut methods = methods.join("\n        ");
    if methods.is_empty() {methods = "None".to_string();}

    format!("Create a structure with:
        Name: {name}

    Fields:
        {fields}

    Constructors:
        {constructors}

    Methods:
        {methods}")
}

pub fn describe_enum(name: &str, fields: &[&str], constructors: &[String], methods: &[String]) -> String {
    let mut fields = fields.join("\n        ");
    if fields.is_empty() {fields = "None".to_string();}

    let constructors = constructors.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    let mut constructors = constructors.join("\n        ");
    if constructors.is_empty() {constructors = "None".to_string();}

    let methods = methods.iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    let mut methods = methods.join("\n        ");
    if methods.is_empty() {methods = "None".to_string();}

    format!("Create a enum with:
        Name: {name}

    Variants:
        {fields}

    Constructors:
        {constructors}

    Methods:
        {methods}")
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
}, task1_7);

test_setup::make_test!(task1_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_7_info'
    assert_stdout_eq!(tasks::hello_twice("David", "Lightman"), "Hi, my name is DavidHi, my name is David Lightman");
    assert_stdout_eq!(tasks::hello_twice("Alex", "Rogan"), "Hi, my name is AlexHi, my name is Alex Rogan");
    //----------------------------------------------------------------------
}, task2_1);


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
    assert_stdout_eq!(tasks::say_add(1, 5), "The result was 6");
    assert_stdout_eq!(tasks::say_add(61, 42), "The result was 103");
    //----------------------------------------------------------------------
}, task2_6);

test_setup::make_test!(task2_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_6_info'
    assert_eq!(tasks::subtract(8, 4), 4);
    assert_eq!(tasks::subtract(61, 42), 19);
    //----------------------------------------------------------------------
}, task2_7);

test_setup::make_test!(task2_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_7_info'
    assert_eq!(tasks::add_three(8, 4, 45), 57);
    assert_eq!(tasks::add_three(61, 42, 1), 104);
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
    assert_stdout_eq!(
    	tasks::say_bool(true), "My boolean is: true"
    );
    assert_stdout_eq!(
    	tasks::say_bool(false), "My boolean is: false"
    );
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
    assert_stdout_eq!(tasks::say_is_equals(8, 2), "Values are not equal");
    assert_stdout_eq!(tasks::say_is_equals(8, 4), "Values are not equal");
    assert_stdout_eq!(tasks::say_is_equals(8, 8), "Values are equal");
    //----------------------------------------------------------------------
}, task3_4);

test_setup::make_test!(task3_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_4_info'
    assert_eq!(tasks::is_not_equal(8, 2), true);
    assert_eq!(tasks::is_not_equal(8, 4), true);
    assert_eq!(tasks::is_not_equal(8, 8), false);
    //----------------------------------------------------------------------
}, task3_5);

test_setup::make_test!(task3_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_5_info'
    assert_eq!(tasks::is_not_equal_again(8, 2), true);
    assert_eq!(tasks::is_not_equal_again(8, 4), true);
    assert_eq!(tasks::is_not_equal_again(8, 8), false);
    //----------------------------------------------------------------------
}, task3_6);

test_setup::make_test!(task3_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_6_info'
    assert_stdout_eq!(tasks::shoes_on(true, true), "Take your shoes off!");
    assert_stdout_eq!(tasks::shoes_on(true, false), "Take your shoes off!");
    assert_stdout_eq!(tasks::shoes_on(false, false), "Good kid!");
    //----------------------------------------------------------------------
}, task3_7);

test_setup::make_test!(task3_7, {
    assert_stdout_eq!(tasks::ready_to_go(true, true), "Ready to go!");
    assert_stdout_eq!(tasks::ready_to_go(true, false), "Not ready to go yet!");
    assert_stdout_eq!(tasks::ready_to_go(false, false), "Not ready to go yet!");
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_7_info'
    //----------------------------------------------------------------------
}, task3_8);

test_setup::make_test!(task3_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_8_info'
    assert_eq!(tasks::ready_to_play(false, false, false), false);
    assert_eq!(tasks::ready_to_play(true, false, false), false);
    assert_eq!(tasks::ready_to_play(false, true, false), false);
    assert_eq!(tasks::ready_to_play(true, true, false), true);
    assert_eq!(tasks::ready_to_play(false, false, true), false);
    assert_eq!(tasks::ready_to_play(true, false, true), true);
    assert_eq!(tasks::ready_to_play(false, true, true), false);
    assert_eq!(tasks::ready_to_play(true, true, true), true);
    //----------------------------------------------------------------------
}, task4_1);

test_setup::make_test!(task4_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_1_info'

    let _test = tasks::DriversLicense{issued: 34, expires: 76};
    //----------------------------------------------------------------------
}, task4_2);

test_setup::make_test!(task4_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_2_info'
    let test = tasks::my_new_dl();
    assert_eq!(test.issued, 4);
    assert_eq!(test.expires, 12);
    //----------------------------------------------------------------------
}, task4_3);

test_setup::make_test!(task4_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_3_info'
    assert_stdout_eq!(
        tasks::print_drivers_license(tasks::DriversLicense{issued: 34, expires: 42}),
        "34-42"
    );
    assert_stdout_eq!(
        tasks::print_drivers_license(tasks::DriversLicense{issued: 72, expires: 80}),
        "72-80"
    );
    //----------------------------------------------------------------------
}, task4_4);


test_setup::make_test!(task4_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_4_info'
    let test = tasks::DriversLicense::new(34);
    assert_eq!(test.issued, 34);
    assert_eq!(test.expires, 42);

    let test = tasks::DriversLicense::new(57);
    assert_eq!(test.issued, 57);
    assert_eq!(test.expires, 65);
    //----------------------------------------------------------------------
}, task4_5);


test_setup::make_test!(task4_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_5_info'
    assert_eq!(tasks::DriversLicense{issued: 34, expires: 42}.is_valid(33), false);
    assert_eq!(tasks::DriversLicense{issued: 34, expires: 42}.is_valid(34), true);
    assert_eq!(tasks::DriversLicense{issued: 34, expires: 42}.is_valid(40), true);
    assert_eq!(tasks::DriversLicense{issued: 34, expires: 42}.is_valid(42), false);
    assert_eq!(tasks::DriversLicense{issued: 34, expires: 42}.is_valid(45), false);

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
        tasks::DriversLicense{issued: 20, expires: 28}.examine(),
        "Hey that's my drivers license"
    );

    assert_stdout_eq!(
        tasks::DriversLicense{issued: 22, expires: 28}.examine(),
        "Drivers license issued during covid"
    );

    assert_stdout_eq!(
        tasks::DriversLicense{issued: 22, expires: 45}.examine(),
        "Drivers license issued during covid"
    );

    assert_stdout_eq!(
        tasks::DriversLicense{issued: 22, expires: 24}.examine(),
        "Drivers license issued during covid"
    );

    assert_stdout_eq!(
        tasks::DriversLicense{issued: 18, expires: 24}.examine(),
        "Expires on the 24 and was issued on 18"
    );

    assert_stdout_eq!(
        tasks::DriversLicense{issued: 10, expires: 24}.examine(),
        "Expires on the 24 and was issued on 10"
    );
    //----------------------------------------------------------------------
}, task5_1);

test_setup::make_test!(task5_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_1_info'

    let blue = tasks::Color::Blue{};
    let red = tasks::Color::Red{};
    let green = tasks::Color::Green{};
    let yellow = tasks::Color::Yellow{};

    //----------------------------------------------------------------------
}, task5_2);

test_setup::make_test!(task5_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_2_info'

    assert_eq!(tasks::Color::Blue{}.is_primary(), true);
    assert_eq!(tasks::Color::Red{}.is_primary(), true);
    assert_eq!(tasks::Color::Green{}.is_primary(), true);
    assert_eq!(tasks::Color::Yellow{}.is_primary(), false);

    //----------------------------------------------------------------------
}, task5_3);

test_setup::make_test!(task5_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_3_info'

    assert_eq!(tasks::Furniture::Couch{legs: 8, cushions: 1}.get_legs(), 8);
    assert_eq!(tasks::Furniture::Couch{legs: 24, cushions: 43}.get_legs(), 24);
    assert_eq!(tasks::Furniture::Chair{legs: 83}.get_legs(), 83);
    assert_eq!(tasks::Furniture::Chair{legs: 22}.get_legs(), 22);
    assert_eq!(tasks::Furniture::Table{legs: 49, plates: 12}.get_legs(), 49);
    assert_eq!(tasks::Furniture::Table{legs: 29, plates: 13}.get_legs(), 29);

    //----------------------------------------------------------------------
}, task5_4);

test_setup::make_test!(task5_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_4_info'

    let some_u8 = tasks::Option::<u8>::Some{value: 134};
    let some_string = tasks::Option::<String>::Some{value: "test".to_string()};
    let none_u8 = tasks::Option::<u8>::None{};
    let none_string = tasks::Option::<String>::None{};

    //----------------------------------------------------------------------
}, task5_5);

test_setup::make_test!(task5_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_5_info'

    let _test = tasks::Option::<u8>::new_some(10);
    let _test = tasks::Option::<String>::new_some("test".to_string());

    //----------------------------------------------------------------------
}, task5_6);

test_setup::make_test!(task5_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_6_info'

    assert_eq!(tasks::Option::<u8>::Some{value: 134}.is_some(), true);
    assert_eq!(tasks::Option::<String>::Some{value: "test".to_string()}.is_some(), true);
    assert_eq!(tasks::Option::<u8>::None{}.is_some(), false);
    assert_eq!(tasks::Option::<String>::None{}.is_some(), false);

    //----------------------------------------------------------------------
}, task5_7);

test_setup::make_test!(task5_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_7_info'

    let my_box = tasks::Box::<u8>::new(134, 20);
    assert_eq!(my_box.boxed_value, 134);
    assert_eq!(my_box.count, 20);

    let my_box = tasks::Box::<String>::new("test".to_string(), 20);
    assert_eq!(my_box.boxed_value, "test".to_string());
    assert_eq!(my_box.count, 20);

    let my_box = my_box.add_one();
    assert_eq!(my_box.boxed_value, "test".to_string());
    assert_eq!(my_box.count, 21);

    //----------------------------------------------------------------------
}, task5_7);

test_setup::make_test!(task5_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task5_8_info'

    let my_box = tasks::create_boxed_option(20);
    assert_eq!(my_box.count, 20);
    assert_eq!(my_box.boxed_value.is_some(), false);

    let my_box = tasks::create_boxed_option(70);
    assert_eq!(my_box.count, 10);
    assert_eq!(my_box.boxed_value.is_some(), true);


    //----------------------------------------------------------------------
}, task6_1);
