//use test_setup::make_test_info;
use super::*;
const TYPE_SEP: &str = "\n    ";
make_test_info!(
    task1_1_info, "This is your first task!",
    "Testing a task will either succeed or give you an error.
    If it gives you an error, you'll see the description of the task and
    the reason for the failure. You can see task 1 currently has an
    error shown at the bottom.

    left: 'Goodbye, World!'
    right: 'Hello, World!'

    Here:
    - 'left' is your function's returned result (what it actually gave).
    - 'right' is the expected result (what it should be).
    Your goal is to fix the function so that left matches right.",
    "Modify the function called hello_world to say 'Hello, World!' instead
    of 'Goodbye, World!'",
    None,
    None
);

make_test_info!(
    task1_2_info, "First Function",
    describe_type("Functions (fn)",
        "
        Imagine you're making a recipe.
        A function is like a recipe that tells the computer how to do something.

        Here's how the different parts of a function work",
        (true, &[
        "pub: This is the keyword to say \"Hey, I'm sharing this recipe with everyone.
        ",
        "fn: This tells the computer you're now creating a function (recipe)
        ",
        "Name: This is the name of your recipe. It's the label you use to reference the function later.
        ",
        "Parameters: These are optional ingredients for your recipe. 
        You're function (recipe) might need paremeters (ingredients) to work, but some functions don't.
        ",
        "Return Type: This is the result of your recipe. What will the function give back after it's finished?
        For example, if the recipe is making a cake, the Return Type would be 'Cake'.
            ",
        "Function Body: This is the part where the function does it's work, like the steps in a recipe.
        Here, you tell the computer exactly what this recipe should do or make.
            "
        ]),
        &[(
            "Example of a function",
            "pub fn function_name(my_str: &str) -> u8 {
                print!(\"Hello, World!\");
                16
            }"
        ),
        (
            "Explanation",
            "- pub fn means we're sharing this function with everyone
            - function_name is what we're calling this function (recipe's name)
            - my_str: &str is the ingredient we need: a piece of text
            - -> u8 tells us the function will give back a small number
            - The function body prints 'Hello, World!' then tells the function 
                to give back 16"
        )]
    ),
    describe_function("hello_everyone", &[], None,
        "Create a print! statement that says 'Hello, Everyone!'"
    ),
    None,
    None
);

make_test_info!(
    task1_3_info, "The &str Type",
    describe_type("&str",
        "
        An &str (string) is just simply a piece of text.",
        (true, &[]),
        &[("To create an &str with the text 'Hello, World!', you surround the text in double quotes", "\"Hello, World!\"")],
    )+TYPE_SEP+
    &describe_type("Formatting String",
        "
        You can use {} as placeholders in a &str, and Rust will 'format' the string 
        by replacing those placeholders with the values you provide",
        (true, &[]),
        &[(
            "You can use placeholders in print! statements.
        It's like a talking program that says whatever you tell it to",
            "print!(\"Hello, {}!\", \"Golden Retreiver\")"
        ),
        (
            "The {} is the blank space.
        Rust takes \"Golden Retreiver\" and puts it in the blank space",
            "The program then says: \"Hello, Golden Retriever!\""
        )],
    ),
    describe_function("hello_pet", &[], None,
        "Create a print! statement that prints \"Hello, Benji!\" by using
        \"Benji\" as the value and a {} placeholder to insert it into 
        the sentence after the comma",
    ),
    None,
    None
);

make_test_info!(
    task1_4_info, "Function Parameters",
    describe_type("Parameters",
        "Parameters are like ingredients for a recipe. 
        When you cook something, you don't always use the same ingredient every time.
        You change it depending on what you want.

        For example, a sandwich recipe might need filling. 
        But you can choose different fillings, like peanut butter or cheeese.
        ",
        (true, &[]),
        &[(
            "In a function",
            "pub fn make_sandwich(filling: &str) {
                print!(\"Here's a {} sandwich!\", filling);
            }",
        ),
        (
            "'filling: &str' is the ingredient (parameter)
        - When you call make_sandwich(\"bacon\"); Rust uses \"bacon\" as the filling
        - The function prints",
            "\"Here's a bacon sandwich!\"",
        )]
    ),
    describe_function("hello_user", &["name: &str"], None,
        "print 'Hi, my name is ' followed by the parameter 'name'"
    ),
    None,
    None
);

make_test_info!(
    task1_5_info, "Calling A Function",
    describe_type("Function Calling",
        "Calling a function is like using a recipe.
        You provide the needed ingredients (parameters), and the
        function does its job
        ",
        (true, &[]),
        &[
            ("To call a function",
            "function_name(parameter_value);"),
            ("To call a function named bake_cake that requires a u8 (small number)",
            "bake_cake(18);"),
            ("To call a function named make_sandwich that requires a &str (string)",
            "make_sandwich(\"cheese\");")
        ],
    ),
    describe_function("hello_mitch", &[], None,
        "Call your function 'hello_user' and give it 'mitch' as the name parameter,
        to print 'Hi, my name is mitch'"
    ),
    Some( "Do not use print! directly in this function"),
    None
);

make_test_info!(
    task1_6_info, "Formal Hello",
    describe_type("Printing Multiple Parameters",
        "Using multiple parmaters is like following a recipe that needs more than
        one ingredient.",
        (true, &[]),
        &[
            (
            "Function with multiple parameters",
            "pub fn describe_animal(name: &str, animal: &str) {
                print!(\"This is {} the {}, name, animal);
            }"),
            ("describe_animal(\"Harry\", \"Frog\")", "\"This is Harry the Frog\""),
            ("describe_animal(\"Danny\", \"Chicken\")", "\"This is Danny the Chicken\""),
            ("Each value fills in a different blank in a print! statement",
            "print!(\"{} has {} apples\", \"Frank\", 5)"
            ),
            ("This will print", "\"Frank has 5 apples\""),
        ],
    ),
    describe_function("formal_hello", &["first_name: &str", "last_name: &str"], None,
        "print 'Hi, my name is ' followed by first_name and last_name with a space in between"
    ),
    None,
    None
);

// make_test_info!(
//     task1_7_info, "Paramater Reuse",
//     describe_type("Reusing Parameters",
//         "Just like reusing the same ingredient in different recipes, parameters can be used
//         multiple times in different function calls.",
//         (true, &[]),
//         &[
//             ("Example reusing 'ingredient' parameter", 
//             "pub fn bake(dessert: &str, ingredient: &str) {
//                 print!(\"Using {} to make {}!\", dessert, ingredient);
//             }
            
//             pub fn prepare_party(ingredient: &str) {
//                 bake(\"cookies\", ingredient);
//                 bake(\"cupcakes\", ingredient);
//             }"),
//             ("Running prepare_party(\"flour\") will print",
//             "Using flour to make cookies!
//             Using flour to make cupcakes!")
//         ],
//     ),
//     describe_function("hello_twice", &["first_name: &str", "last_name: &str"], None,
//         "use your previous function 'hello_user' to print the first_name,
//         then use 'formal_hello' to print both first_name and last_name"
//     ),
//     Some("Do not use print! directly in this function"),
//     None
// );

// ---------------- SECTION TWO ---------------- //

make_test_info!(
    task2_1_info, "First number Type: u8",
    describe_type("u8",
        "A u8 is a whole number between 0 and 255",
        (true, &[]),
        &[],
    ),
    describe_function("say_age", &["age: u8"], None,
        "print 'My age is: ' followed by the parameter"
    ),
    None,
    None
);

make_test_info!(
    task2_2_info, "First Return Type",
    describe_type("Function Return Types",
        "
        A function return type tells your computer what kind of value the function
        will give back after running.",
        (false, &[
            "Remember how a function is like a cake recipe?
            You can require ingredients (parameters)
            Then, you bake the dessert (function body)
            And afterwards, you'll have a finished dessert (return value)
            Depending on the recipe, you'll get a different dessert (return type)"
        ]),
        &[
            ("Example",
            "pub fn bake_cake() -> &str {
                \"Chocolate Cake\"
            }"
            ),
            ("Explanation

            -> : This arrow is saying, \"After the function does its work, it will give something back\"

            &str : This is the type of value the function will return.",
            "So, '-> &str' means, \"This function will give back a &str (string) when it finishes\"")
        ],
    ),
    describe_function("my_age", &[], Some("u8"),
        "return your age"
    ),
    None,
    None
);

make_test_info!(
    task2_3_info, "Using a Returned Value",
    describe_type("Returned Values",
        "
        The returned value is what the function gives back to you after it finishes.",
        (true, &[]),
        &[(
            "This function gives you the &str \"Chocolate Cake\" when it finishes",
            "pub fn bake_cake() -> &str {
                \"Chocolate Cake\"
            }"
        ),
        (
            "Using returned value in print! statement",
            "print!(\"I made you a {}\", bake_cake());

            This will print: \"I made you a Chocolate Cake\""
        )
        ],
    ),
    describe_function("say_my_age", &[], None,
        "print 'My age is ' followed by your the result of my_age()"
    ),
    Some("Do not set your age directly, instead use my_age()"),
    None
);

make_test_info!(
    task2_4_info, "First Operator: +",
    describe_type("Operators",
        "Operators are special functions that take two parameters:
            one before and one after the operator symbol.
        They perform a specific operation and return the resulting value.
        Similar to functions, once an operator is declared, it evaluates
        to its resulting value.",
        (false, &[
            "Operators work with many different types, but the first and second
            parameter must be the exact same Type",
            "(4 + \"Hi\") will not compile because they are different Types",
        ]),
        &[]
    )+TYPE_SEP+
    &describe_type("'+' Operator",
        "The + (plus) operator is used to add two numbers together.",
        (true, &[]),
        &[
            ("(4 + 3) becomes", "7")
        ],
    ),
    describe_function("add", &["a: u8", "b: u8"], Some("u8"),
        "add the two u8 parameters and return the result"
    ),
    None,
    None
);

make_test_info!(
    task2_5_info, "First Variable",
    describe_type("Variables",
        "Until now, we've used parameters and values directly. However, it can
        get confusing if you need to do more than one thing in a function.",
        (false, &[
             "Variables work like parameters, but you declare their name , type, and value.",
            "You declare them using the let keyword, then you can use them
            just like parameters.",
        ]),
        &[(
            "To declare (create) a variable",
            "let name: Type = value;
            let my_number: u8 = 18;
            ",
        ),
        ("To create a variable using the returned value of a function", "let name: Type = my_function();")
        ],
    ),
    describe_function("say_add", &["a: u8", "b: u8"], None,
        "Declare a variable called 'result' with Type u8.
        Set 'result' to be the returned value of add(a, b).
        Print 'The result was ' followed by the variable 'result'"
    ),
    Some("Do not use + directly and instead use 'add'"),
    None
);

make_test_info!(
    task2_6_info, "Second Operator: -",
    describe_type("'-' Operator",
        "The - (minus) operator is used to subtract two numbers.",
        (true, &[]),
        &[
            ("(4 - 3) becomes", "1")
        ],
    ),
    describe_function("subtract", &["a: u8", "b: u8"], Some("u8"),
        "subtract 'a' from 'b' and return the result"
    ),
    None,
    None
);

make_test_info!(
    task2_7_info, "Second Variable",
    "No Additional Info",
    describe_function("add_three", &["a: u8", "b: u8", "c: u8"], Some("u8"),
        "add all three parameters together and return the result"
    ),
    Some("Do not use + directly instead use your 'add()' function"),
    Some("You can call 'add()' more than once. You can store the value add() returned inside a variable for later use")
);

make_test_info!(
    task2_8_info, "Put It All Together!",
    "",
    describe_function("formal_greet", &["first_name: &str", "last_name: &str", "a: u8", "b: u8"], None,
        "use 'formal_hello()' to print the two strings.
        add the u8s together and print them using 'say_age()'"
    ),
    Some("Do not use print! or '+' in 'formal_greet' directly, instead use 'add'"),
    None
);

// ---------------- SECTION THREE ---------------- //

make_test_info!(
    task3_1_info, "First Boolean",
    describe_type("Booleans",
        "Booleans: Can only store one of two values: true or false.
        They are often used in conditions and comparisons.",
        (true, &[]),
        &[
            ("To declare a false boolean", "false"),
            ("To declare a true boolean", "true"),
        ],
    ),
    describe_function("say_bool", &["a: bool"], None,
         "print 'My boolean is: ' followed by the parameter 'a'"
    ),
    None,
    None
);


make_test_info!(
    task3_2_info, "Equals Operator",
    describe_type("'==' Operator",
        "The == (equals) operator is used check if two values are equal
        and return a boolean (true or false)",
        (true, &[]),
        &[
            ("(5 == 5) becomes", "true"),
            ("(5 == 3) becomes", "false"),
        ],
    ),
    describe_function("is_equal", &["a: u8", "b: u8"], Some("bool"),
        "return true if a is equal to b, otherwise return false"
    ),
    None,
    None
);

make_test_info!(
    task3_3_info, "Using Match Statements",
    describe_type("Match Statement",
        "A match statement is an expression that takes a value and compares
        it against a series of patterns. If a pattern matches, the code after the
        corosponding \"=>\" is ran. A pattern matches if the value == pattern",
        (true, &[]),
        &[
            ("Match statements follow this format",
            "match value {
                pattern => {
                    //function body
                },
                pattern => {
                    //function body
                }
            }"),
            ("For example",
            "match 30 {
                10 => {
                    print!(\"I am 10\");
                },
                20 => {
                    print!(\"I am 20\");
                },
                30 => {
                    print!(\"I am 30\");
                }
            }

            This will print \"I am 30\""),
        ],
    ),
    describe_function("say_is_equals", &["a: u8", "b: u8"], None,
        "call 'is_equal(a, b)' and use a match statement to:
          - Print \"Values are not equal\" if it is false.
          - Print \"Values are equal\" if it is true."
    ),
    Some("Do not use == directly in 'say_is_equals', instead use 'is_equals'
    Do not use an if statement, match statements only"),
    None
);

make_test_info!(
    task3_4_info, "Not Operator",
    describe_type("'!' Operator",
        "The ! (not) operator is a unique operator that only has one parameter.
        When applied to a bool, it flips the booleans value. true to false, or false to true",
        (true, &[]),
        &[
            ("(!true) becomes", "false"),
            ("(!false) becomes", "true"),
        ],
    ),
    describe_function("is_not_equal", &["a: u8", "b: u8"], Some("bool"),
        "return true if a is not equal to b, otherwise return false"
    ),
    Some("Use the ! operator and the function 'is_equals' only",),
    Some("is_equals returns a bool, flip it")
);

make_test_info!(
    task3_5_info, "Compound Operators",
    describe_type("NotEquals Operator !=",
        "Compound Operators are a special category of operators that combine multiple operators.
        For example, the NotEquals operator works exactly like the previous function:",
        (false, &[
            "!(\"hi\" == \"hi\") == false",
            "\"hi\" != \"hi\" == false",
        ]),
        &[],
    ),
    describe_function("is_not_equal_again", &["a: u8", "b: u8"], Some("bool"),
        "return true if a is not equal to b, otherwise return false",
    ),
    Some("Do not use 'is_equals' or '!' and '==', instead use the != symbol"),
    None
);

make_test_info!(
    task3_6_info, "|| Operator",
    describe_type("'||' Operator",
        "The || (or) operator evaluates two boolean expressions and returns
        true if either of the expressions is true.",
        (true, &[]),
        &[
            ("(false || true) becomes", "true"),
            ("(true || false) becomes", "true"),
            ("(false || false) becomes", "false"),
        ],
    ),
    describe_function("shoes_on", &["left: bool", "right: bool"], None,
        "use the || operator to check if either boolean is true
        then, use a match statement to:
          - Print \"Take your shoes off!\" if true.
          - Print \"Good kid!\" if false.",
    ),
    Some("Do not use an if statement, match statements only"),
    None
);

make_test_info!(
    task3_7_info, "&& Operator",
    describe_type("'&&' Operator",
        "The && (and) Operator evaluates two boolean expressions and returns true only if both expressions are true.",
        (true, &[]),
        &[
            ("(true && true) becomes", "true"),
            ("(false && true) becomes", "false"),
            ("(false && false) becomes", "false"),
        ],
    ),
    describe_function("ready_to_go", &["shoes: bool", "coat: bool"], None,
        "use the && operator to check if coat and shoes are both true
        then, use a match statement to:
          - Print \"Ready to go!\" if both are true.
          - Print \"Not ready to go yet!\" if either or both conditions are false.",
    ),
    Some("Do not use an if statement, match statements only"),
    None
);

make_test_info!(
    task3_8_info, "Chained Operators",
    describe_type("Chained Operators",
        "Operators can also be chained together to make multiple operations at once.
        But, you must be careful not to confuse things.
        It is recommendend to always wrap your operations in parentheses.
        Running 'cargo clippy' will let you know when the parentheses are not needed.",
        (false, &[
            "Here is how the chained expression evaluates with ():",
            "(false && true) || true",
            "false || true",
            "true",
            "But without them specified the compiler guesses resulting in:",
            "false && true || true",
            "false && (true || true) <- compiler guessing",
            "false && true",
            "false"
        ]),
        &[],
    ),
    describe_function("ready_to_play", &["shoes: bool", "is_hot: bool", "coat: bool"], Some("bool"),
        "use || and && to return true if:
            shoes are true and (is_hot or coat) is true",
    ),
    None,
    None
);

// String - String
// &String - Borrowed String
// &mut String - Mutable Borrowed String
// &str - String Slice
// mut &str - Mutable String Slice

make_test_info!(
    task4_1_info, "First Structure",
    describe_type("Structures",
        "
        A struct (structure) is like a custom box where you can put different pieces
        of related information together.
        
        Think of it as a container for things that belong together. 
        Each item inside the container has a name and type.",
        (false, &[
            "Think of a Tank ready for war. 

            Every Tank has a few key things: 
                - Armor
                - Shells
                - Speed

            These are all different but important pieces of information that belong together.
            In Rust, we use a struct to group this info into one place, 
            just like carrying all the tank's important details on a card.
            "
        ]),
        &[
            (
            "Example",
            "pub struct Tank {
                pub armor: u8,
                pub shells: u8,
                pub speed: u8
            }
            "
            ),
            (
            "Explanation",
            "- pub struct: This tells the computer that we are creating a public structure
            - Tank: We named this struct Tank
            - armor, shells, speed: These are the fields in our Tank. 
                - Fields are the individual pieces of data inside the struct.
                - Each field holds a specific value that describes a part of the struct.
            "
            ),
        ],
    ),
    describe_structure(
        "Warship",
        &["pub cannons: u8", "pub torpedoes: u8", "pub speed: u8"],
        &[],
        &[],
    ),
    None,
    None
);

make_test_info!(
    task4_2_info, "Struct Instances",
    describe_type("Creating a Struct Instance",
        "
        When you create a struct instance, you're making a variable that holds all
        the values of that struct. 
        
        This is like building a Tank - once it's made you can use it and check its stats.",
        (false, &[]),
        &[
            (
            "Example",
            "pub struct Tank {
                pub armor: u8,
                pub shells: u8,
                pub speed: u8
            }

            let my_tank: Tank = Tank {
                armor: 10,
                shells: 24,
                speed: 100,
            };"
            ),
            (
            "Explanation",
            "let my_tank: Tank - We create a Tank based off our struct
            Tank {} - We start our struct by using the struct's name and curly braces {}
            armor: 10, shells: 24, speed: 100 - We set the values of each field for this tank"
            ),
        ],
    ),
    describe_function(
        "create_warship", &[], Some("Warship"),
        "Return an instance of your Warship structure.

        Set cannons to 12,
        Set torpedoes to 24,
        Set speed to 100"
    ),
    None,
    None
);

make_test_info!(
    task4_3_info, "Using Fields",
    describe_type("Structure Fields",
        "
        Structure fields are exactly like variables.
        To 'access' them you need to state the variable name followed by '.' and the field name.",
        (true, &[]),
        &[
            (
            "Example",
            "let my_tank: Tank = Tank {
                armor: 10,
                shells: 24,
                speed: 100,
            };

            To see how many shells you have left: my_tank.shells"
            ),
            (
            "To print shells left",
            "print!(\"My tank has {} shells left\", my_tank.shells);"
            ),
        ],
    ),
    describe_function(
        "cannon_count",
        &["warship: Warship"],
        None,
        "print the cannons field in the warship parameter like this: 
        
            \"My warship has 5 cannons left\"
        ",
    ),
    None,
    None
);

make_test_info!(
    task4_4_info, "Impl Block",
    describe_type("Structure Impl",
        "
        Usually when we create a structure, 
        we also want functions that are specific to that structure.
        
        To Implement these functions, you create an 'impl' block
        to tell the computer which functions are specific to which structures",
        (false, &[]),
        &[
            (
            "Example",
            "pub struct Tank {
                pub armor: u8,
                pub shells: u8,
                pub speed: u8
            }

            impl Tank {
                pub fn new() -> Tank {
                    Tank {
                        armor: 10,
                        shells: 24,
                        speed: 100,
                    }
                }
            }"
            ),
            (
            "Explanation",
            "new() is a function that most structs will have. 
            new() is classified as a 'constructor' function because it
            creates (constructs) an instance of the struct."
            ),
            (
                "Calling new(): let my_tank: Tank = Tank::new(250)",
                "let my_tank: Tank - The start of a variable

            Tank:: - This tells the computer that we are about to call a 
                     function from inside the Impl block of the Tank struct

            new() - Calling the new function"
            ),
        ],
    ),
    "Create an impl block for Warship.

    Inside this impl Warship block: \n\n        ".to_string()
    +&describe_function(
        "new", &["max_speed: u8"], Some("Warship"),
        "Return an instance of your Warship structure.

        Set cannons to 12,
        Set torpedoes to 24,
        Set speed to the max_speed parameter"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task4_5_info, "Methods",
    describe_type("Structure Methods",
        "A method is another kind function that belongs to a struct.

        This type of function requires an instance of the struct as a parameter.
        The struct parameter is passed in differently than most functions.",
        (false, &[]),
        &[
            (
            "Example of a 'method' function",
            "impl Tank {
                pub fn check_speed(self) {
                    print!(\"Speed is currently at {}\", self.speed);
                }
            }"
            ),
            (
            "Explanation",
            "In order to call a method, you will need an instance of the struct.
            
            let my_tank: Tank = Tank::new();
            
            Now, lets call the check_speed method on my_tank:
            
            my_tank.check_speed()
            
            Since we are attaching check_speed to my_tank, 
            check_speed will use my_tank as the 'self' parameter"
            ),
        ],
    ),
    "Inside your impl Warship block, beneath the new function:\n        ".to_string()
    +&describe_function(
        "torpedo_check", &["self: Warship"], None,
        "print the torpedo field of the self parameter like this: 
        
            \"I have 5 torpedoes left\""
    ).replace("\n", "\n    "),
    None,
    None
);


make_test_info!(
    task4_6_info, "Matching & Wildcards",
    describe_type("Matching",
        "
        Just like we can match using a boolean, we can match on any Type.
        But, when creating a match statment you have to cover every single case.
        With booleans, True and False are the only cases.
        But with u8s, there are 255 cases.
        
        In order to not have to create 255 match arms, 
        we use a special character to catch all other cases",
        (true, &[]),
        &[
            (
                "The Wildcard _",
                "You can use _ to catch 'all other cases'"
            ),
            (
            "Example using u8s",
            "match 24 {
                3 => {
                    print!(\"I am 3!\");
                },
                18 => {
                    print!(\"I am 18!\");
                },
                _ => {
                    print!(\"I am not 3 or 18.\");
                }
            }"
            )
        ],
    ),
    describe_function("print_apples", &["apples: u8"], None,
        "Use a match statement on the apples parameter to print:

            if apples is 1 print \"One apple\",
            if apples is 2 print \"Two apples\",
            if apples is 3 print \"Three apples\",
            Otherwise, print \"More than three apples\""
    ),
    None,
    None
);

make_test_info!(
    task4_7_info, "Matching on Types",
    describe_type("Variable Catch",
        "
        The second solution is to catch the value into a variable",
        (true, &[]),
        &[
            (
            "Example",
            "match 20 {
                3 => {
                    print!(\"I am 3!\");
                },
                18 => {
                    print!(\"I am 18!\");
                },
                age => {
                    print!(\"I am {}!\", age);
                }
            }"
            ),
        ],
    ),
    describe_function("print_oranges", &["oranges: u8"], None,
        "Use a match statement on oranges to print:

            if oranges is 1 print \"One orange\",
            if oranges is 2 print \"Two oranges\",
            if oranges is 3 print \"Three oranges\",
            Otherwise, print the number of oranges followed by \" oranges\""
    ),
    None,
    None
);

make_test_info!(
    task4_8_info, "Matching on Structures",
    describe_type("Structure Matching",
        "Not only can you match on regular types but you can also match on entire Structures.",
        (true, &[]),
        &[
            (
            "Example",
            "pub struct Bandit {
                pub is_wanted: bool,
                pub bounty: u8,
            }

            let wilbur = Bandit {
                is_wanted: true,
                bounty: 250,
            };

            match wilbur {
                Bandit { is_wanted: false, bounty: _ } => {
                    print!(\"Wilbur is a free man\");
                },
                Bandit { is_wanted: true, bounty: 250 } => {
                    print!(\"Wilbur is an expensive wanted man\");
                },
                _ => {
                    print!(\"Wilbur is a man\");
                },
            }"
            ),
        ],
    ),
    "Inside your Impl Warship block, beneath the torpedo_check function:
    \n        ".to_string()
    +&describe_function(
        "examine", &["self: Warship"], None,
        "Use a match statement on self:

            In the case where cannons is 5, torpedoes is 10, speed is 100 

                - print: \"This ship is stocked up and ready to go\"

            In the case where cannons is 0, torpedoes is 1, speed is 24 

                - print: \"This ship is low and slow\"
            
            In any other case, 

                - print: \"This ship is a mystery\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task5_1_info, "Enumerators",
    describe_type("Enumerator",
        "An enumerator is a Type where its value can be one of a list of sub-structs",
        (false, &[
            "All the sub-structs for an enumerator a defined with the enumerator",
            "An enumerator can be matched on, to figure out what sub-struct it is",
            "Enumerators have an impl block that can contain methods and constructors",
            "To declare an Enum you use the TypePath Seperator '::' to choose the sub-struct"
        ]),
        &[
            (
            "Example",
            "pub enum Weather {
                Clear{},
                Rain{},
                Snow{}
            }
            //Enum TypeName is Weather
            //Clear, Rain, Snow are sub-structs

            let current_weather: Weather = Weather::Rain{};
            let preferred_weather: Weather = Weather::Snow{};"
            ),
        ],
    ),
    describe_enum(
        "Color",
        &["Blue{}", "Red{}", "Green{}", "Yellow{}"],
        &[],
        &[],
    ),
    None,
    None
);

make_test_info!(
    task5_2_info, "Matching on Enums",
    describe_type("Enum Matching",
        "Just like structures you can attach methods or constructors with an impl block.
        As well as matching on the sub-structs and values of an enum",
        (true, &[]),
        &[
            (
            "Example",
            "pub enum Weather {
                Clear{},
                Rain{},
                Snow{}
            }

            impl Weather {
                pub fn print(self: Weather) {
                    match self {

                       Weather::Clear{} => print!(\"Clear and sunny skies\"),
                        Weather::Rain{} => print!(\"Rain and light clouds\"),
                        Weather::Snow{} => print!(\"Snow and heavy clouds\"),
                    }
                }
            }"
            ),
        ],
    ),
    "Add and impl block to the Color enum and add a method to it:\n        ".to_string()
    +&describe_function("is_primary", &["self: Color"], Some("bool"),
        "Use a match statement to return true if the Color sub-struct is Blue, Red, or Green"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task5_3_info, "Fields",
    describe_type("Sub-Struct Fields",
        "Like structures Sub-Structs can have fields too, to access the fields you'll have
        to match the enum just like structure matching.",
        (false, &[
            "The sub-struct fields do not need to be prefixed with pub, explanation later"
        ]),
        &[
            (
            "Example",
            "pub enum Weather {
                Clear{chance: u8},
                Rain{chance: u8},
                Snow{chance: u8}
            }

            impl Weather {
                pub fn print(self: Weather) {
                    match self {
                       Weather::Clear{chance: the_chance} => print!(\"{}% of Clear\", the_chance),
                        Weather::Rain{chance: the_chance} => print!(\"{}% of Rain\", the_chance),
                        Weather::Snow{chance: the_chance} => print!(\"{}% of Snow\", the_chance),
                    }
                }
            }"
            ),
        ],
    ),
    describe_enum(
        "Furniture",
        &["Couch{
            legs: u8,
            cushions: u8
        },", "Chair{
            legs: u8
        },", "Table{
            legs: u8,
            plates: u8,
        }"],
        &[],
        &[describe_function("get_legs", &["self: Furniture"], Some("u8"),
            "match on the furniture and return the number of legs each piece of furniture has"
        )],
    ),
    None,
    Some("Remember to use WildCard '_' to ignore unneeded fields")
);

make_test_info!(
    task5_4_info, "Type Parameter",
    describe_type("T: TypeParameter",
        "There is a special feature of Types, including structures and enums, that allow you
        to accept a TypeParemeter, This is for situations where you want a type that can be
        used to store any type in a field.",
        (false, &[
            "To declare a Type Parameter place arrow braces after the Structure Name",
            "To use the structure you have to provide the TypeParameter this is done
            using the :: TypePathSeperator and Arrow Braces <>"
        ]),
        &[
            (
            "Example",
            "pub enum Result<T> {
                Ok{
                    value: T
                },
                Error{
                    code: u8
                }
            }

            let my_ok_result = Result::<u8>::Ok{value: 125};//125 is a u8 replacing the T
            let my_err_result = Result::<&str>::Error{code: 2};
            //For the Error we still have to specify the Type Param even though it is unused
            "
            ),
        ],
    ),
    describe_enum(
        "Option<T>",
        &["Some{
            value: T,
        },",
        "None{}"
        ],
        &[],
        &[],
    ),
    None,
    None
);

make_test_info!(
    task5_5_info, "Impl TypeParamater",
    describe_type("TypeParam Constructor",
        "TypeParameters can be a bit of a pain to deal with since you won't know what
        Type your dealing when creating methods or constructors",
        (false, &[
            "In order to impl a TypeParameter Structure you have to include the TypeParam
            in the impl block definition as shown in the example.",
            "To create or construct a Structure with a Type Parameter you have to include
            the type as apart of the brace constructor Name::<Type>::SubStruct{...}",
        ]),
        &[
            (
            "Example",
            "pub enum Result<T> {
                Ok{
                    value: T
                },
                Error{
                    code: u8
                }
            }

            impl<T> Result<T> {
                //For a method that accepts any Type T
                pub fn new_ok(value_param: T) -> Result<T> {
                    Result::<T>::Ok{value: value_param}
                }

                //To just create a Result with a u8
                pub fn new_ok_u8(value_param: u8) -> Result<u8> {
                    Result::<u8>::Ok{value: value_param}
                }

                //For a new error, even if the TypeParam is unused it still has to be specified
                pub fn new_err(code_param: u8) -> Result<T> {
                    Result::<T>::Error{code: code_param}
                }
            }"
            ),
        ],
    ),
    "Add and impl<T> block to the Option<T> enum and add a constructor to it:\n        ".to_string()
    +&describe_function("new_some", &["value_param: T"], Some("Option<T>"),
        "Construct the Some SubStrucure of Option and use value_param as the value"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task5_6_info, "Impl TypeParamater",
    describe_type("TypeParam Method",
        "Another downside is that returning a value with a TypeParamater is much harder,
        since you cannot provide a value that could be one Type or another. But a match
        statement can still come in handy here.",
        (false, &[]),
        &[
            (
            "Example",
            "pub enum Result<T> {
                Ok{
                    value: T
                },
                Error{
                    code: u8
                }
            }

            impl<T> Result<T> {
                pub fn get_error_code(self: Result<T>) -> u8 {
                    match self {
                        Result::<T>::Ok{value: _} => 0,//Zero error code is no error
                        Result::<T>::Error{code: err_code} => err_code,
                    }
                }
            }"
            ),
        ],
    ),
    "Add a method to the impl<T> Option<T> Block:\n        ".to_string()
    +&describe_function("is_some", &["self: Option<T>"], Some("bool"),
        "Use a match statement to return true if the Option sub-struct is Some"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task5_7_info, "TypeParam Structure",
    describe_type("Structure TypeParam",
        "Structures can accept Type Parameters exactly the same way as enums",
        (false, &[]),
        &[
            (
            "Example",
            "pub struct Box<T> {
                pub boxed_value: T,
                pub count: u8
            }

            let my_box = Box::<u8>{boxed_value: 10, count: 1};
            "
            ),
        ],
    ),
    describe_structure(
        "Box<T>",
        &[
            "pub boxed_value: T",
            "pub count: u8"
        ],
        &[describe_function("new", &["value: T, count: u8"], Some("Box<T>"),
            "Create a new Box<T> and set the boxed_value to value, and count to the count param"
        )],
        &[describe_function("add_one", &["self: Box<T>"], Some("Box<T>"),
            "Take in self and create a new Box<T> setting boxed_value to self.boxed_value,
            But set count to the old count plus one"
        )],
    ),
    None,
    Some("While add_one does return a Box<T> like a constructor, because it accepts self: Box<T>
    its considered to be a method and ran just like any other method.")
);

make_test_info!(
    task5_8_info, "Nested TypeParam",
    describe_type("Nested TypeParam",
        "You can use your structure or enums inside other structures or enums just like
        any other type. But when you have types that require a Type Param you have to
        specify the TypeParam in every place you use it. And when you do this you
        will likely end up with nested TypeParams, fortunately this is possible by simply
        nesting the arrow braces.",
        (false, &[]),
        &[
            (
            "Example",
            "pub struct Box<T> {
                pub boxed_value: T,
                pub count: u8
            }

            pub enum Result<T> {
                Ok{value: T},
                Error{code: u8}
            }

            impl<T> Result<T> {
                pub fn new_ok(my_value: T) -> Result<T> {
                    Result::<T>::Ok{value: my_value}
                }
            }

            let my_boxed_result: Box<Result<u8>> = Box::<Result<u8>>{
                boxed_value: Result::<u8>::new_ok(10),
                count: 1
            };"
            ),
        ],
    ),
    describe_function(
        "create_boxed_option", &["value: u8"], Some("Box<Option<u8>>"),
        "Accept value which is a u8, use a match statement and '>' to check:
        If value is greater than 50:
            Return the Option Some substructure with value equal to the param
            Box the Option Some with count 10
        Otherwise:
            Return None SubStructure Boxed with count 20"
    ),
    None,
    Some("Use the Box::<Option<u8>>::new() and the Option::<u8>::new_some() constructors")
);

//  make_test_info!(
//      task4_5_info, "Methods on Types",
//      describe_type("String",
//          "The String Type is exactly like a &str but it can be stored in structures.
//          All rust Types are just Structures and have methods and constructors:
//          the &str Type has a .to_string() method to convert it to a String.
//          The &str Type cannot be stored because it is borrowed(more on that later)
//          To store a string you must convert the &str to a String with '.to_string()'",
//          (false, &[]),
//          &[
//              (
//              "Example",
//              "let my_str: &str = \"Hello\";
//              let my_string: String = my_str.to_string();

//              Or more directly:

//              let my_string: String = \"Hello\".to_string();"
//              ),
//          ],
//      ),
//      describe_structure(
//          "FullName",
//          &["first: String", "last: String"],
//          &[describe_function("new", &["first: &str", "last: &str"], Some("FullName"),
//              "Create a new FullName by runing .to_string() on the parameters"
//          )],
//          &[describe_function("print", &["full_name: &FullName"], None,
//              "print the first and last name with a space between them"
//          )]
//      ),
//      None,
//      None
//  );

//  make_test_info!(
//      task4_6_info, "Pointers/Borrowing",
//      describe_type("Pointer",
//          "Previously we spoke of reusing a parameter in 'hello_twice'. This was possible because
//          the parameter was a &str (borrowed str) or better called (pointer to a str).",
//          (false, &[
//              "Values can only be used once unless they are (Copy)ied, (Clone)d, or pointed to(&)",
//              "Copying is availble for certian types and is taken care of automatically",
//              "Cloning is creating an exact duplicate with .clone() and is best avoided unless necessary",
//              "Borrowing is creating a readonly pointer to the value.",
//              "&str are always borrowed so they cannot be stored nor modified after creation",
//              "(String)s are a version of string that can be stored and its created from a &str.",
//              "Strings can be borrowed or a pointer can be created to it by placing a & behind it.
//              This essentially turns it into a &str and this is useful when ever you need
//              to simlpy read/print a value and not modifiy it."
//          ]),
//          &[
//              (
//              "Example",
//              "let my_name: String = \"Alex\".to_string();

//              hello_user(&my_name);"
//              ),
//          ],
//      ),
//      "Add a method to the bottom of the impl for FullName:\n        ".to_string()
//      +&describe_function("hello", &["full_name: &FullName"], None,
//          "Use 'formal_hello' to print the first and last name. Don't forget to borrow
//          the fields as you pass them to formal_hello"
//      ).replace("\n", "\n    "),
//      None,
//      None
//  );



// make_test_info!(
//     task4_1_info, "Ownership",
//     describe_type("Understanding Ownership",
//         "
//         Only one variable can 'own' a piece of data at a time.
//         When ownership is 'transferred', the original variable can no longer access that data.",
//         (false, &[]),
//         &[
//             (
//                 "Here's an example showing ownership being moved from x to y",
//                 "let x: u8 = 48;
//             let y = x;"
//             ),
//             (
//                 "In this example",
//                 "Ownership of the value 48 is with x.
//             When y = x is executed, ownership of 48 is moved to y
//             After the transfer, trying to access x results in an error because ownership was moved."
//             )
//         ],
//     ),
//     describe_function("move_value", &["a: u8"], None,
//         "Move ownership of a's value to a variable you create called b.
//         Then, print the value of b like this:

//             print!(\"b has ownership of {}\", b)",
//     ),
//     None,
//     None
// );
