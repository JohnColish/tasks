#[cfg(feature="tasks")]
pub mod tasks;
pub mod task_info;

pub fn rules() {
    println!("
----------------------------------------------------------------------

RULES:
1. Make sure no warnings appear in either the tests or after running
    'cargo clippy' in your tasks directory
2. Never change a previous task after you've completed it.
3. Never prefix variables with _. Fix warnings in a proper way instead.
4. Do not use the test results to hard code the answer, meaning
    Any information provided by the test suite such as 'David' or 18
    should not be used in any of the task functions
5. Do not edit any other files in the project except for src/tasks.rs

Follow these rules to ensure your solutions are clean and correct!
----------------------------------------------------------------------
    ");
}

pub fn congrats() {
    println!("
----------------------------------------------------------------------

CONGRATS YOU FINISHED THE SECTION!

Move on by increasing the section number:

For Section 2: cargo test --features=task2_1_info

----------------------------------------------------------------------
    ");
}

pub fn list(items: &[&str], numbered: bool) -> String {
    items.into_iter().enumerate().map(|(ix, i)| {
        let ix = ix+1;
        let li = if numbered { format!("{ix}. ")} else { "- ".to_string()};
        format!("        {}{}", li, i)
    }).collect::<Vec<_>>().join("\n\n")
}

pub fn examples(exs: &[(&str, &str)]) -> String {
    exs.into_iter().map(|(desc, exa)|
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

    let constructors = constructors.into_iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
    let mut constructors = constructors.join("\n        ");
    if constructors.is_empty() {constructors = "None".to_string();}

    let methods = methods.into_iter().map(|m| m.split("\n").collect::<Vec<_>>().join("\n    ")).collect::<Vec<_>>();
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
});

test_setup::make_test!(task1_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_2_info'

    assert_stdout_eq!(tasks::hello_everyone(), "Hello, Everyone!");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task1_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_3_info'

    assert_stdout_eq!(tasks::hello_pet(), "Hello, Benji!");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task1_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_4_info'

    assert_stdout_eq!(tasks::hello_mitch(), "Hi, my name is Mitch");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task1_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_5_info'
    assert_stdout_eq!(tasks::hello_user("David"), "Hi, my name is David");
    assert_stdout_eq!(tasks::hello_user("Alex"), "Hi, my name is Alex");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task1_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_6_info'
    assert_stdout_eq!(tasks::formal_hello("David", "Lightman"), "Hi, my name is David Lightman");
    assert_stdout_eq!(tasks::formal_hello("Alex", "Rogan"), "Hi, my name is Alex Rogan");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task1_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task1_7_info'
    assert_stdout_eq!(tasks::hello_twice("David", "Lightman"), "Hi, my name is DavidHi, my name is David Lightman");
    assert_stdout_eq!(tasks::hello_twice("Alex", "Rogan"), "Hi, my name is AlexHi, my name is Alex Rogan");
    //----------------------------------------------------------------------
});


test_setup::make_test!(task2_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_1_info'
    assert_stdout_eq!(tasks::say_age(17), "My age is 17");
    assert_stdout_eq!(tasks::say_age(19), "My age is 19");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task2_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_2_info'

    let age: u8 = tasks::my_age();
    //----------------------------------------------------------------------
});

test_setup::make_test!(task2_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_3_info'

    assert_stdout_eq!(tasks::say_my_age(), format!("My age is {}", tasks::my_age()).as_str());
    //----------------------------------------------------------------------
});

test_setup::make_test!(task2_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_4_info'
    assert_eq!(tasks::add(1, 4), 5);
    assert_eq!(tasks::add(61, 42), 103);
    //----------------------------------------------------------------------
});

test_setup::make_test!(task2_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_5_info'
    assert_stdout_eq!(tasks::say_add(1, 5), "The result was 6");
    assert_stdout_eq!(tasks::say_add(61, 42), "The result was 103");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task2_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_6_info'
    assert_eq!(tasks::subtract(8, 4), 4);
    assert_eq!(tasks::subtract(61, 42), 19);
    //----------------------------------------------------------------------
});

test_setup::make_test!(task2_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task2_7_info'
    assert_eq!(tasks::add_three(8, 4, 45), 57);
    assert_eq!(tasks::add_three(61, 42, 1), 104);
    //----------------------------------------------------------------------
});

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
    congrats();
    //----------------------------------------------------------------------
});

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
});

test_setup::make_test!(task3_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_2_info'
    assert_eq!(tasks::is_equal(8, 2), false);
    assert_eq!(tasks::is_equal(8, 4), false);
    assert_eq!(tasks::is_equal(8, 8), true);
    //----------------------------------------------------------------------
});

test_setup::make_test!(task3_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_3_info'
    assert_stdout_eq!(tasks::say_is_equals(8, 2), "Values are not equal");
    assert_stdout_eq!(tasks::say_is_equals(8, 4), "Values are not equal");
    assert_stdout_eq!(tasks::say_is_equals(8, 8), "Values are equal");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task3_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_4_info'
    assert_eq!(tasks::is_not_equal(8, 2), true);
    assert_eq!(tasks::is_not_equal(8, 4), true);
    assert_eq!(tasks::is_not_equal(8, 8), false);
    //----------------------------------------------------------------------
});

test_setup::make_test!(task3_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_5_info'
    assert_eq!(tasks::is_not_equal_again(8, 2), true);
    assert_eq!(tasks::is_not_equal_again(8, 4), true);
    assert_eq!(tasks::is_not_equal_again(8, 8), false);
    //----------------------------------------------------------------------
});

test_setup::make_test!(task3_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_6_info'
    assert_stdout_eq!(tasks::shoes_on(true, true), "Take your shoes off!");
    assert_stdout_eq!(tasks::shoes_on(true, false), "Take your shoes off!");
    assert_stdout_eq!(tasks::shoes_on(false, false), "Good kid!");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task3_7, {
    assert_stdout_eq!(tasks::ready_to_go(true, true), "Ready to go!");
    assert_stdout_eq!(tasks::ready_to_go(true, false), "Not ready to go yet!");
    assert_stdout_eq!(tasks::ready_to_go(false, false), "Not ready to go yet!");
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task3_7_info'
    //----------------------------------------------------------------------
});

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
});

test_setup::make_test!(task4_1, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_1_info'

    let test = tasks::Tombstone{birth_year: 34, death_year: 76};
    //----------------------------------------------------------------------
});

test_setup::make_test!(task4_2, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_2_info'

    assert_stdout_eq!(
        tasks::print_tombstone(tasks::Tombstone{birth_year: 34, death_year: 76}),
        "34-76"
    );
    //----------------------------------------------------------------------
});

test_setup::make_test!(task4_3, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_3_info'
    assert_stdout_eq!(
        tasks::DriversLicense{issued: 34, expires: 76}.print(),
        "Issued on 34, Expires on 76"
    );
    //----------------------------------------------------------------------
});

test_setup::make_test!(task4_4, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_4_info'
    assert_stdout_eq!(
        tasks::DriversLicense::new(34).print(),
        "Issued on 34, Expires on 42"
    );
    //----------------------------------------------------------------------
});

test_setup::make_test!(task4_5, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_5_info'
    assert_stdout_eq!(
        tasks::FullName::new("David", "Lightman").print(),
        "David Lightman"
    );
    //----------------------------------------------------------------------
});

test_setup::make_test!(task4_6, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_6_info'
    assert_stdout_eq!(
        tasks::FullName::new("David", "Lightman").hello(),
        "Hi, my name is David Lightman"
    );
    //----------------------------------------------------------------------
});

test_setup::make_test!(task4_7, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_7_info'
    assert_stdout_eq!(tasks::print_age(10), "You are ten years old!");
    assert_stdout_eq!(tasks::print_age(tasks::my_age()), "You are the same age as me!");
    assert_stdout_eq!(tasks::print_age(14), "I don't know how old you are but you are alive!");
    assert_stdout_eq!(tasks::print_age(120), "I don't know how old you are but you are alive!");
    //----------------------------------------------------------------------
});

test_setup::make_test!(task4_8, {
    //----------------------------------------------------------------------
    //	THIS ERROR MEANS THE TEST COULD NOT FIND YOUR FUNCTION:
    //	CHECK FOR TYPOS OR RUN 'cargo test features=task4_8_info'

    assert_stdout_eq!(
        tasks::examine_tombstone(tasks::Tombstone{birth_year: 12, death_year: 92}),
        "He was born in the year '12 and lived 80 years!"
    );

    assert_stdout_eq!(
        tasks::examine_tombstone(tasks::Tombstone{birth_year: 49, death_year: 30}),
        "I don't know when they were born but they died in the year '30"
    );

    assert_stdout_eq!(
        tasks::examine_tombstone(tasks::Tombstone{birth_year: 20, death_year: 30}),
        "I don't know when they were born but they died in the year '30"
    );

    assert_stdout_eq!(
        tasks::examine_tombstone(tasks::Tombstone{birth_year: 20, death_year: 120}),
        "I don't know when they were born but they died in the year '120"
    );

    assert_stdout_eq!(
        tasks::examine_tombstone(tasks::Tombstone{birth_year: 20, death_year: 56}),
        "I don't know when they were born but they died in the year '56"
    );
    //----------------------------------------------------------------------
});
