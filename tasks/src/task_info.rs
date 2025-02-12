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
        "pub: This is the keyword to say \"Hey, I'm sharing this function (recipe) with all my files\".
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
            "- pub means this function can be seen by any file
            - fn means we're creating a function now
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
            ("Example",
            "bake_cake(18);"),
            ("Explanation",
            "- bake_cake is the name of our function
            - () parentheses are used to provide the parameters (ingredients)
            - If your function doesn't need parameters, leave the () empty"),
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
    describe_type("'+' Operator",
        "
        The + (plus) operator is used to add two numbers together.

        Examples - 

            4 + 3 becomes: 7
            1 + 2 becomes: 3
            12 + 12 becomes: 24",
        (false, &[]),
        &[
            (
                "To add two variables together",
                "x + y"
            ),
            (
                "When using the + (plus) operator",
                "Rust will take the first variable 
            and add it to the second variable"
            )
        ],
    ),
    describe_function("add", &["a: u8", "b: u8"], Some("u8"),
        "return the a and b parameters added together"
    ),
    None,
    None
);

make_test_info!(
    task2_5_info, "Second Operator: -",
    describe_type("'-' Operator",
        "
        The - (minus) operator is used to subtract two numbers.
        
        Examples - 

            10 - 7 becomes: 3
            4 - 2 becomes: 2
            120 - 20 becomes: 100",
        (true, &[]),
        &[
            (
                "To subtract two variables",
                "x - y"
            ),
            (
                "When using the - (minus) operator",
                "Rust will take the first variable 
            and subtract it from the second variable"
            )
        ],
    ),
    describe_function("subtract", &["a: u8", "b: u8"], Some("u8"),
        "subtract 'a' from 'b' and return the result"
    ),
    None,
    None
);

make_test_info!(
    task2_6_info, "First Variable",
    describe_type("Variables",
        "Variables are like boxes in rust.
        A variable can hold one value",
        (false, &[]),
        &[(
            "To create a variable (box) containing the text \"Chocolate Cake\"",
            "let my_cake: &str = \"Chocolate Cake\";",
        ),
        (
            "Explanation",
            "let - this is the keyword that tells the computer we are now creating a variable (box)

            my_cake - this is what we are naming our box so we can use it later

            : &str - variables (boxes) need to know what type of value they hold (this box contains an &str)

            =   we use the '=' symbol to put a value inside the variable (box)

            \"Chocolate Cake\" - this is the value we put inside our variable (the text \"Chocolate Cake\")
            
            ; - this semi-colon tells the computer we are done creating a variable now"
            
        )
        ],
    ),
    describe_function("jonah", &[], None,
        "Declare a variable called 'name' with the type &str.
        Put the text \"Jonah\" inside the name variable.
        Print \"Their name is \" followed by 'name'"
    ),
    None,
    None
);

make_test_info!(
    task2_7_info, "Second Variable",
    describe_type("Returned Values in Variables",
        "When you call a function, sometimes it gives back something. (the return value)
        You can keep that value in a variable to use it",
        (false, &[]),
        &[(
            "Example",
            "pub fn pbj_sandwich() -> &str {
                \"Here's a PBJ sandwich!\"
            }

            let sandwich: &str = pbj_sandwich();",
        ),
        (
            "Explanation",
            "let sandwich: &str - creating a variable that will hold an &str

            pbj_sandwich(); - running the pbj_sandwich function
                - it will give us the text \"Here's a PBJ sandwich!\"
            
            sandwich is now a box containing the text \"Here's a PBJ sandwich!\""
        )
        ],
    ),
    describe_function("say_add", &["a: u8", "b: u8"], None,
        "Declare a variable called 'result' with Type u8.
        Set 'result' to be the returned value of add(a, b).
        Print \"The result is \" followed by the variable 'result'"
    ),
    None,
    None
);

make_test_info!(
    task2_8_info, "Put It All Together!",
    "",
    describe_function("formal_greet", &["first_name: &str", "last_name: &str", "a: u8", "b: u8"], None,
        "use 'formal_hello()' to print the two strings.
        add the u8s together and print them using 'say_age()'"
    ),
    Some("Do not use '+' directly, instead use 'add'
    Do not use print! directly, instead use formal_hello()"),
    None
);

// ---------------- SECTION THREE ---------------- //

make_test_info!(
    task3_1_info, "First Boolean",
    describe_type("Booleans",
        "
        A Boolean is like a switch. It's either true or false",
        (true, &[]),
        &[
            ("Example", "let is_alive: bool = true;"),
            ("Example", "let is_underwater: bool = false;"),
        ],
    ),
    describe_function("is_awesome", &[], Some("bool"),
         "return true"
    ),
    None,
    None
);


make_test_info!(
    task3_2_info, "Equals Operator",
    describe_type("'==' Operator",
        "The == (equals) operator is used check if two values are equal",
        (true, &[]),
        &[
            ("(5 == 5)", "This is true, because 5 is equal to 5"),
            ("(5 == 3)", "This is false, because 5 is not equal to 3"),
        ],
    ),
    describe_function("is_equal", &["a: u8", "b: u8"], Some("bool"),
        "use the == operator on a and b and return the result"
    ),
    None,
    None
);

make_test_info!(
    task3_3_info, "Using Match Statements",
    describe_type("Match Statement",
        "
        Let’s say you have a box, and inside the box could be one of three things: 
        an apple, an orange, or a banana. 
        
        The match statement helps you check what’s inside the box and do 
        something different depending on what you find.",
        (true, &[]),
        &[
            ("Example",
            "let fruit_box: &str = \"apple\";

            match fruit_box {
                \"apple\" => print!(\"You have an apple!\"),
                \"orange\" => print!(\"You have an orange!\"),
                \"banana\" => print!(\"You have a banana!\"),
                _ => print!(\"I don't know what this fruit is!\")
            }"),
            ("Explanation",
            "
            let fruit_box: &str = \"apple\"; 
                - We create a box and put the text \"apple\" inside it

            match fruit_box {} 
                - This tells the computer we want to check the contents of 
                fruit_box and run case specific code

             \"apple\" => print!(\"You have an apple!\"),
                - If the content is \"apple\" then we will run the code after the => arrow
            
            _ => print!(\"I don't know what this fruit is!\")
                - We use an underscore to say \"If the content is none of the above,
                then run the code after this => arrow\"
            

            ")
        ],
    ),
    describe_function("your_planet", &["planet: &str"], None,
        "create a match statement for the parameter 'planet'
        
        if the content of the parameter 'planet' is:
        
            \"mercury\", then print \"About 77 million kilometers away.\"

            \"venus\", then print \"About 41 million kilometers away.\"
            
            \"neptune\", then print \"About 4.3 billion kilometers away.\"
            
            if it's anything else, use a _, then print \"Undiscovered planet.\""
    ),
    None,
    None
);

make_test_info!(
    task3_4_info, "Match Statements on booleans",
    describe_type("Match Statement",
        "
        We can also match on a boolean (a box that either contains true or false)",
        (true, &[]),
        &[
            ("Example",
            "let is_alive: bool = true;

            match is_alive {
                true => print!(\"Yes! I am still alive.\"),
                false => print!(\"No, I have died.\")
            }"),
        ],
    ),
    describe_function("underwater", &["is_underwater: bool"], None,
        "create a match statement for the parameter 'is_underwater'
        
        if the content of 'is_underwater' is:
        
            true, then print \"I am underwater\"

            false, then print \"Not underwater\"
            
        p.s. We have covered the only options, so we do not need an underscore arm"
    ),
    None,
    None
);


make_test_info!(
    task3_5_info, "Match Statements on operations",
    describe_type("Match Statement",
        "
        We can also match on the result of an operation (as long as that operation results in a boolean)",
        (true, &[]),
        &[
            ("Example",
            "match 10 == 10 {
                true => print!(\"This is equal\"),
                false => print!(\"This is not equal\")
            }"),
        ],
    ),
    describe_function("say_is_equal", &["a: u8, b: u8"], None,
        "create a match statement to see if a == b
        
        if the result is:
        
            true, then print \"a is the same as b\"

            false, then print \"as is not the same as b\"
            
        p.s. We have covered the only options, so we do not need an underscore arm"
    ),
    None,
    None
);

make_test_info!(
    task3_6_info, "Not Operator",
    describe_type("'!' Operator",
        "
        The ! operator is called the 'not' operator. 
        It inverts or flips a boolean value. 

        If a value is true, ! changes it to false. 
        If the value is false, ! changes it to true.",
        (true, &[]),
        &[
            ("Example", "let light_on = true; // The light is on (true)
    
            let light_off = !light_on; // The ! operator flips it, making it false (off)")
        ],
    ),
    describe_function("power_status", &["is_active: bool"], Some("bool"),
        "flip the is_active boolean using the ! operator and return it"
    ),
    None,
    None
);

make_test_info!(
    task3_7_info, "|| Operator",
    describe_type("'||' Operator",
        "The || (or) operator checks two booleans and if either is true, then the expression will result in true
        If both of the booleans are false, then the expression will result in false",
        (true, &[]),
        &[
            ("(false || true) is", "true - because one of the booleans is true"),
            ("(true || false) is", "true - because one of the booleans is true"),
            ("(false || false) is", "false - because neither of the booleans are true"),
        ],
    ),
    describe_function("decide_battle", &["is_dragon_sleeping: bool", "is_giant_away: bool"], None,
        "create a match statement to see if is_dragon_sleeping || is_giant_away
        
        if the result is:
        
            true, then print \"Run!\"

            false, then print \"Fight!\"
            
        p.s. We have covered the only options, so we do not need an underscore arm",
    ),
    None,
    None
);

make_test_info!(
    task3_8_info, "&& Operator",
    describe_type("'&&' Operator",
        "The && (and) operator checks two booleans and if both is true, then the expression will result in true
        If either of the booleans are false, then the expression will result in false",
        (true, &[]),
        &[
            ("(true && true) is", "true - because both of the booleans are true"),
            ("(false && true) is", "false - because one of the booleans is false"),
            ("(false && false) is", "false - because one of the booleans is false"),
        ],
    ),
    describe_function("has_access", &["has_pass: bool", "has_permission: bool"], None,
        "create a match statement to see if has_pass && has_permission
        
        if the result is:
        
            true, then print \"You can enter!\"

            false, then print \"Access denied.\"
            
        p.s. We have covered the only options, so we do not need an underscore arm",
    ),
    None,
    None
);

// Haven't covered:
    // Chained Operators
    // Is greater than
    // Is less than
    // Is greater than or equal to
    // Is less than or equal to
    // Is not equal

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
        
            \"My warship has _ cannons left\"
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
        
            \"I have _ torpedoes left\""
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
        "
        Not only can you match on regular types but you can also match on entire Structures.",
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
        "
        An enumerator is a container of structs",
        (false, &[]),
        &[
            (
                "Example using empty structs",
                "pub enum Treasure {
                Gold {},
                Gems {},
                Artifact {},
            }"
            ),
            (
                "Explanation",
                "pub enum - This tells the computer \"I am now going to create a 'public enumerator'
            Treasure - This is the name of our enumerator
            Gold {}, Gems {}, Artifact {} - These are the struct variants inside our enum"
            ),
        ],
    ),
    describe_enum(
        "Spacecraft",
        &["\n\tFighter {}", "Cargo {}", "Colonizer {}", "Explorer {}"],
        &[],
        &[],
    ),
    None,
    None
);

make_test_info!(
    task5_2_info, "Enumerators",
    describe_type("Instance of enumerator variant",
        "
        To create a variable containing a variant of an enumerator",
        (false, &[]),
        &[
            (
                "Example",
                "pub enum Treasure {
                Gold {},
                Gems {},
                Artifact {},
            }
                
            let my_treasure = Treasure::Gems{};"
            ),
            (
                "Explanation",
                "let my_treasure - Creating a variable called my_treasure
            Treasure:: - Tells the computer we want to instantiate a variant of the Treasure enum
            Gems{} - The variant that we chose"
            ),
        ],
    ),
    describe_function("get_fighter", &[], Some("Spacecraft"),
        "return an instance of the Fighter {} variant from your Spacecraft enum"
    ),
    None,
    None
);

make_test_info!(
    task5_3_info, "Matching on Enums",
    describe_type("Enum Matching",
        "
        Just like structures you can attach methods or constructors with an impl block.
        As well as matching on the sub-structs and values of an enum.",
        (true, &[]),
        &[
            (
            "Example",
            "pub enum Treasure {
                Gold {},
                Gems {},
                Artifact {},
            }
                
            impl Treasure {
                pub fn print(self: Treasure) {
                    match self {
                        Treasuer::Gold{} => print!(\"Wow! Real gold!\"),
                        Treasure::Gems{} => print!(\"Beautiful gems!\"),
                        Treasure::Artifact{} => print!(\"A priceless artifact!\"),
                    }
                }
            }"
            ),
        ],
    ),
    "Create an impl block for the Spacecraft enum,
    and inside:
    \n        ".to_string()
    +&describe_function("is_passenger", &["self: Spacecraft"], Some("bool"),
        "Use a match statement to return true if 
        self is Fighter, Colonizer, or Explorer"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task5_4_info, "Fields",
    describe_type("Fields",
        "
        Like regular Structs, enumerator Sub-Structs can have fields too.",
        (false, &[
            "The sub-struct fields do not need to be prefixed with pub, explanation later"
        ]),
        &[
            (
            "Example",
            "pub enum Treasure {
                Gold { count: u8 },
                Gems { count: u8 },
                Artifact { count: u8 },
            }

            let chest = Treasure::Gold { count: 48 };

            let gold_count = chest.count;"
            ),
        ],
    ),
    describe_enum(
        "Potion",
        &["Invisibility { strength: u8 },", 
        "Healing { strength: u8 },", 
        "Poison { strength: u8 },"],
        &[],
        &[],
    ),
    None,
    None
);


make_test_info!(
    task5_5_info, "Matching with Enumerators",
    describe_type("Enum Matching",
        "
        When matching on an enumerator variant, we must 'collect' the value of each field in a variable",
        (false, &[]),
        &[
            (
            "Example",
            "pub enum Treasure {
                Gold { count: u8 },
                Gems { count: u8 },
                Artifact { count: u8 },
            }

            let chest = Treasure::Gold { count: 48 };

            match chest {
                Treasure::Gold { count: gold } => print!(\"{} gold!\", gold),
                Treasure::Gems { count: gems } => print!(\"{} gems!\", gems),
                Treasure::Artifact { count: artifacts } => print!(\"{} artifacts!\", artifacts),
            };"
            ), 
            (
                "Explanation",
                "Treasure::Gold { count: gold } 
                - Here we are 'collecting' count value in a variable named gold
                
            print!(\"{} gold!\", gold),
                - Now, we print the value of gold"
            )
        ],
    ),
    "Create an impl block for the Potion enum,
    and inside:
    \n        ".to_string()
    +&describe_function("say_strength", &["self: Potion"], None,
        "use a match statement to print each potions strength:

            \"You found an Invisibility potion with _ strength\"
            \"You found a Healing potion with _ strength\"
            \"You found a Poison potion with _ strength\""
    ).replace("\n", "\n    "),
    None,
    None
);


make_test_info!(
    task5_6_info, "Matching with Enumerators",
    describe_type("Using _ to collect field values",
        "
        Sometimes, we don't want to do anything with a field's value.
        
        So, we 'collect' this value with an underscore _",
        (false, &[]),
        &[
            (
            "Example",
            "match chest {
                Treasure::Gold { count: _ } => print!(\"This is gold. I want artifacts!\"),
                Treasure::Gems { count: _ } => print!(\"These are gems. I want artifacts!\"),
                Treasure::Artifact { count: artifacts } => print!(\"{} artifacts!\", artifacts),
            };"
            ), 
            (
                "Explanation",
                "Treasure::Gold { count: _ } 
                - We don't care what the value of count is, so we 'collect' it in _"
            )
        ],
    ),
    "Create an impl block for the Potion enum,
    and inside:
    \n        ".to_string()
    +&describe_function("poison_strength", &["self: Potion"], None,
        "use a match statement to print potions. Only the strength of the poison variant:

            \"Invisibility potion\"
            \"Healing potion\"
            \"_ strength poison potion\""
    ).replace("\n", "\n    "),
    None,
    None
);


make_test_info!(
    task5_7_info, "Explicit Returning",
    describe_type("return keyword",
        "
        Sometimes, we want to return a variable we've already created. 
        Without creating a new one.
        
        We can use the 'return' keyword",
        (false, &[]),
        &[
            (
            "Example",
            "pub fn make_sandwich() -> &str {
                let sandwich = \"Chicken Sandwich\";
                print!(\"Here is a {}\", sandwich);
                return sandwich
            }"
            ), 
            (
                "Explanation",
                "let sandwich = \"Chicken Sandwich\"; - Created a sandwich variable
            print!(\"Here is a {}\", sandwich); - Printed the sandwich value
            return sandwich - Now the function will return sandwich"
            )
        ],
    ),
    describe_function("my_trucks", &[], Some("u8"),
        "create a variable called trucks, with the type u8
        set the value to 4
        print the 'trucks' variable: \"I have {} trucks\"
        return the 'trucks' variable using the 'return' keyword"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task5_8_info, "Const Variables",
    describe_type("Const",
        "
        When we create a variable inside a function, we can only use it within that function.
        If you tried to use it inside of another function, it would not work.
        
        You can create a special kind of variable that will be usable anywhere.
        This variable is called a const variable and is created outside outside a function.
        The value of a const cannot be changed after created.",
        (true, &[]),
        &[
            (
            "Example",
            "const MAGIC_WORD: &str = \"Abracadabra\";"
            ),
            (
            "Explanation",
            "const - This is the keyword used to create a const variable (like let)
            MAGIC_WORD - This is the name of our const. Const names have to be all uppercase
            : &str - This const contains a piece of text
            \"Abracadabra\" - The text"
            ),
        ],
    ),
    describe_const("COOKIES_LEFT", "u8", "2").replace("\n", "\n    ")+"\n\n    "
    +&describe_function("eat_cookie", &[""], None,
        "match COOKIES_LEFT < 3
        
        true, print! \"Who is eating all the cookies?\"
        false, print! \"There are so many cookies. I will eat one!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_1_info, "Mutatable Variables",
    describe_type("mut keyword",
        "
        By default, after creating a variable you cannot change the value.
        But this isn't always what we want sometimes we want to add, subtract, etc. from a value.
        
        We can allow variables to be changeable, 
        by using the keyword 'mut' (mutable) when declaring the variable",
        (true, &[]),
        &[
            (
            "Example",
            "let mut my_dollars: u8 = 12;"
            ),
            (
            "Explanation",
            "let mut - we are creating a variable who's value can be changed
            my_dollars: u8 - the variable is named my_dollars and contains a number
            12; - the variable contains the number 12
            
            Now, later if I spend any dollars, I can adjust the variable to keep track!"
            )
        ],
    ),
    describe_function("level_up", &[], None,
        "Create a mutable variable called player_level
        set the type to u8, and the value to 12
        
        Add 1 to player_level
        
        Print player_level \"Level Up! Level is _\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_2_info, "Mutatable Parameters",
    describe_type("mut keyword",
        "
        By default, parameters are also not mutable.
        
        We can allow parameters to be mutable, 
        by using the keyword 'mut' (mutable) when defining the parameters",
        (true, &[]),
        &[
            (
            "Example",
            "pub fn eat_carrot(mut carrots: u8) {
                carrots = carrots - 1;
                print!(\"I ate a carrot and now I have {} left\", carrots);
            }"
            )
        ],
    ),
    describe_function("burn_candle", &["mut candles: u8"], None,
        "Subtract 1 from candles 
        
        Print candles \"I burned a candle and now I have _ left\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_3_info, "First Collection",
    describe_type("Vectors",
        "
        A vector is a type that can contain multiple values",
        (true, &[]),
        &[
            (
            "Example",
            "let creatures: Vec<&str> = vec![\"Dragon\", \"Kraken\", \"Griffin\", \"Wyvern\"];"
            ),
            (
            "Explanation",
            "let creatures: - We create a variable called creatures
            
            Vec<&str> - This variable's type is a Vector of &str
            
            = vec![] - This is how you create a Vector
            
            \"Dragon\", \"Kraken\", \"Griffin\", \"Wyvern\" 
                - The values that our vector contains, separated by commas"
            )
        ],
    ),
    describe_function("kms_this_week", &[], Some("Vec<u8>"),
        "return a vector with the values 5, 7, 10, 5, 7, 5, 10"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_4_info, "Printing Vectors",
    describe_type("Debug Formatting",
        "
        We cannot print a complex data type like a vector in rust using {} as the blankspace.
        Rust will not know how to display vectors properly.

        We have to use {:?} as the blankspace. 
        Using {:?} tells the computer to do its best to display the value",
        (true, &[]),
        &[
            (
            "Example",
            "let creatures: Vec<&str> = vec![\"Dragon\", \"Kraken\", \"Griffin\", \"Wyvern\"];
            print!(\"In my house are {:?}, creatures);
            "
            ),
            (
            "This will print",
            "\"In my house are [\"Dragon\", \"Kraken\", \"Griffin\", \"Wyvern\"]"
            )
        ],
    ),
    describe_function("eggs_this_week", &[], None,
        "create a variable called eggs with type Vec<u8>
        set the values to 3, 5, 2, 4, 4, 2, 6
        print the vector like this: \"I got _ eggs this week\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_5_info, "Adding to a vector",
    describe_type(".push() method",
        "
        Vectors have a method called push(), this is used to add a value to the end of a vector",
        (true, &[]),
        &[
            (
            "Example",
            "let creatures: Vec<&str> = vec![\"Dragon\", \"Kraken\", \"Griffin\", \"Wyvern\"];
            creatures.push(\"John Colish\");
            "
            ),
            (
            "The variable creatures after the push contains",
            "[\"Dragon\", \"Kraken\", \"Griffin\", \"Wyvern\", \"John Colish\"]"
            )
        ],
    ),
    describe_function("high_scores", &["mut scores: Vec<u8>", "new_score: u8"], Some("Vec<u8>"),
        "push new_score onto the end of scores
        return the 'scores' variable"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_6_info, "Adding to a vector",
    describe_type(".pop() method",
        "
        Vectors have a method called pop(), this is used to remove a value from the end of a vector
        The .pop() method doesn't take in any parameters",
        (true, &[]),
        &[
            (
            "Example",
            "let creatures: Vec<&str> = vec![\"Dragon\", \"Kraken\", \"Griffin\", \"Wyvern\"];
            creatures.pop();
            "
            ),
            (
            "The variable creatures after the push contains",
            "[\"Dragon\", \"Kraken\", \"Griffin\"]"
            )
        ],
    ),
    describe_function("disqualified", &["mut scores: Vec<u8>"], Some("Vec<u8>"),
        "pop the end of scores
        return the 'scores' variable"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_7_info, "Second Collection",
    describe_type("Tuples",
        "
        The values in a vector have to be all the same kind. 
        To make a collection of values with different types, we use Tuples",
        (true, &[]),
        &[
            (
            "Example",
            "let my_friend: (&str, u8) = (\"Luke\", 24);
            "
            ),
            (
            "Explanation",
            "let my_friend:  - the start of a variable
            (&str, u8) - this tuple will contain a piece of text and a small number
            = (\"Luke\", 24) - my_friend contains a tuple with \"Luke\" as the text and 24 as the number"
            )
        ],
    ),
    describe_function("mission", &[], None,
        "Create a variable called moon_mission

        Set the type to (&str, u8)

        Set the value to a tuple where 
        the first value is the mission name: \"Apollo 11\" 
        and the second is a u8 for passenger count: 3
        
        Print the variable like this: \"Mission, Passengers: _\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task6_8_info, "Printing Tuple Value",
    describe_type("Indexing",
        "
        To 'access' a single value inside a tuple, 
        you will need the variable/parameter name followed by a period . 
        followed by the position of the value
        
        Collections in rust are 'zero indexed' which means the first value is at position 0,
        the second value is at position 1,
        the third value is at position 2,
        and so on...",
        (true, &[]),
        &[
            (
            "Example",
            "let my_friend: (&str, u8) = (\"Luke\", 24)
            "
            ),
            (
            "print!(\"I have a friend named {}\", my_friend.0); will print",
            "\"I have a friend named Luke\""
            ),
            (
            "print!(\"My friend is {} years old\", my_friend.1); will print",
            "\"My friend is 24 years old\""
            )
        ],
    ),
    describe_function("sports_car", &["car: (&str, u8)"], None,
        "print the parameter like this: \"I have a _ with top speed _\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_1_info, "Optional Value",
    describe_type("Some & None",
        "
        Option<> is a type of value that can either contain something, or contain nothing",
        (false, &[]),
        &[
            (
            "Example",
            "pub fn vending_machine(choice: &str) -> Option<&str> {
                match choice {
                    \"chips\" => Some(\"You got a bag of chips!\"),
                    \"soda\" => Some(\"A refreshing soda appears!\"),
                    \"candy\" => Some(\"Sweet! You got a candy bar!\"),
                    _ => None, // The machine is out of stock!
                }
            }"
            ),
            (
            "Explanation",
            "pub fn vending_machine(choice: &str) -> - the start of a function

            Option<&str> - this function will either return an &str or None

            When returning an optional value, you have to wrap the value in Some()
            Some(\"You got a bag of chips!\")

            _ => None - If the choice was none of the above, return None
            "
            )
        ],
    ),
    describe_function("gold_coins", &["island: &str"], Some("Option<u8>"),
        "Create a match statement for island and if 

        \"Galapagos\", then return Some(50)
        \"Madagascar\", then return Some(100)
        \"Maldives\", then return Some(50)

        If anything else, use an underscore _ and return None"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_2_info, "Third Collection",
    describe_type("Hashmaps",
        "
        A HashMap is like this chest of labeled drawers:

        Each label is called a key
        The item inside is called a value

        You use the key to find the value.
        This is useful when you want to store and quickly look up things, like:
        A phone book (names → phone numbers) 
        A scoreboard (players → high scores)
        A dictionary (word → definition)",
        (true, &[]),
        &[
            (
            "Example",
            "let mut monkeys: Hashmap<&str, &str> = HashMap::new();

            monkeys.insert(\"Terence\", \"Capuchin\");
            monkeys.insert(\"Alex\", \"Proboscis\");
            "
            ),
            (
            "Explanation",
            "let mut monkeys: - the start of a mutable variable
            
            Hashmap<&str, &str> - the type will be a Hashmap where the key and value are &str
            
            HashMap::new() - creates an new, empty hashmap
            
            monkeys.insert() - this is the method used to add the key and value pair

            \"Terence\" - this is the key
            \"Capuchin\" - this is the value"
            ),
            (
                "Help",
                "Hashmaps are not a type that comes by default. 
            Add ' use std::collections::HashMap; ' to the top of your tasks.rs file"
            )
        ],
    ),
    describe_function("soccer_player", &[], Some("Hashmap<u8, u8>"),
        "Create a mutable variable called 'players' with type Hashmap<u8, u8>
        Set 'players' to a new Hashmap
        Insert the key 12 with value 3 (12 being player's number, 3 being goals)
        Return players"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_3_info, "Getting Hashmap Values",
    describe_type(".get() method",
        "
        To get a value from a hashmap using a key, 
        we use the .get() method on the variable and pass in the key as a parameter
        
        .get() will return an Option<> value, because the hashmap might not be able
        to find the value you requested from the key you provided",
        (true, &[]),
        &[
            (
            "Example using monkeys",
            "let terence_breed: &str = monkeys.get(\"Terence\");"
            ),
            (
            "Explanation",
            "let terence_breed: &str = - creating a variable
            monkeys.get(\"Terence\") - using the get method with the key \"Terence\"
            
            terence_breed now contains \"Capuchin\""
            ),
            (
            "Help",
            "HashMaps cannot be printed with the default {} blankspace. Use {:?}"
            )
        ],
    ),
    describe_function("salmon_count", &["todays_catch: Hashmap<&str, u8>"], None,
        "todays_catch is a hashmap of fish species (the &str) and amount caught (the u8) 
        
        Create a variable called 'salmon' containing value associated with the key \"Salmon\"
        Print it like this: \"Today we caught _ salmon!\""
    ).replace("\n", "\n    "),
    None,
    Some("** THIS IS THE LAST TASK **")
);



// <=
// >=
// -=
// +=

// Unnamed struct/enum fields

// Hashmaps x
// Vectors x
// Tuples x

// If Else
// If Let

// Some x
// None x

// Loop
// While loop
// For in loop

// Generic types
// Traits
// Mut

// Borrowing and Ownership
// Lifetimes

// Const x
// Macros

// Break
// Continue

// Async
// Await
