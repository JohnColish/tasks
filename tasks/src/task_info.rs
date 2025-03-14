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
    describe_type("HashMaps",
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
            "let mut monkeys: HashMap<&str, &str> = HashMap::new();

            monkeys.insert(\"Terence\", \"Capuchin\");
            monkeys.insert(\"Alex\", \"Proboscis\");
            "
            ),
            (
            "Explanation",
            "let mut monkeys: - the start of a mutable variable
            
            HashMap<&str, &str> - the type will be a HashMap where the key and value are &str
            
            HashMap::new() - creates an new, empty hashmap
            
            monkeys.insert() - this is the method used to add the key and value pair

            \"Terence\" - this is the key
            \"Capuchin\" - this is the value"
            ),
            (
                "Help",
                "HashMaps are not a type that comes by default. 
            Add ' use std::collections::HashMap; ' to the top of your tasks.rs file"
            )
        ],
    ),
    describe_function("soccer_player", &[], Some("HashMap<u8, u8>"),
        "Create a mutable variable called 'players' with type HashMap<u8, u8>
        Set 'players' to a new HashMap
        Insert the key 12 with value 3 (12 being player's number, 3 being goals)
        Return players"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_3_info, "Getting HashMap Values",
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
    describe_function("salmon_count", &["todays_catch: HashMap<&str, u8>"], None,
        "todays_catch is a hashmap of fish species (the &str) and amount caught (the u8) 
        
        Create a variable called 'salmon' containing value associated with the key \"Salmon\"
        Print it like this: \"Today we caught _ salmon!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_4_info, "More Operators",
    describe_type("Efficient Adding, Subtracting",
        "
        Instead of subtracting from a variable like this:

        let mut cookies = 2;
        cookies = cookies - 1;
        
        We can use the -= operator

        let mut cookies = 2;
        cookies -= 1;


        Instead of adding to a variable like this:

        let mut pencils = 2;
        pencils = pencils + 1;
        
        We can use the += operator

        let mut pencils = 2;
        pencils += 1;
        ",
        (true, &[]),
        &[],
    ),
    describe_function("make_n_drink", &["mut oranges: u8", "mut orange_juice: u8"], Some("(u8, u8)"),
        "Subtract 1 from oranges
        Add 1 to orange_juice
        
        Return a tuple containing oranges and orange_juice"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_5_info, "Your First Loop",
    describe_type("loop {}",
        "
        To run the same chunk of code over and over again, 
        ou can wrap the code in a loop {}",
        (true, &[]),
        &[
            (
            "Example",
            "loop {
                print!(\"Hey, how's it going?\")
            }"
            ),
            (
            "Explanation",
            "This will print \"Hey, how's it going?\" over and over again,
            until you quit the program"
            )
        ],
    ),
    describe_function("car_horn", &[], None,
        "Use a loop to print \"BEEP!\" constantly"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_6_info, "Second Loop Style",
    describe_type("For In Loops",
        "
        This is the most popular form of loop, and is used to run code a specific number of times",
        (true, &[]),
        &[
            (
            "Example",
            "let cookies = 5;
            
            for cookie in 0..=cookies {
                print!(\"Mmmm... Yummy cookies!\");
            }"
            ),
            (
            "Explanation",
            "let cookies = 5; - Created a variable with value 5

            for cookie in 0..=cookies - This is like saying
                \"For each cookie in cookies, starting at zero - run this code\"

            0..=cookies - Loop will run from 0 until the count of cookies

            for cookie in - Creates a variable called cookie that contains 
            the number representing which round of the loop we are currently in

            (The first loop, cookie will be 0. Second loop, cookie will be 1. Etc.)

            print!(\"Mmmm... Yummy cookies!\"); - Print statement

            This loop will run 6 times
            "
            )
        ],
    ),
    describe_function("throw_snowballs", &["snowballs: u8"], None,
        "For each snowball in snowballs, print \"Snowball!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_7_info, "For Loops",
    describe_type("For In Variables",
        "
        We can use the variable containing the number representing which round of the
        loop we are currently in, inside our loop",
        (true, &[]),
        &[
            (
            "Example",
            "let cookies = 5;
            
            for cookie in 0..=cookies {
                print!(\"I have eaten {} cookies!\", cookie);
            }"
            ),
            (
            "Explanation",
            "print!(\"I have eaten {} cookies!\", cookie); - Uses cookie variable

            This will print:
            
            \"I have eaten 0 cookies!\" on the first loop
            \"I have eaten 1 cookies!\" on the second loop
            \"I have eaten 2 cookies!\" on the third loop
            \"I have eaten 3 cookies!\" on the fourth loop
            \"I have eaten 4 cookies!\" on the fifth loop
            \"I have eaten 5 cookies!\" on the sixth loop
            "
            )
        ],
    ),
    describe_function("feed_capybara", &["capybaras: u8"], None,
        "For each capybara in capybaras, print \"I have fed {} capybaras!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task7_8_info, "For Loops",
    describe_type("0, 1 Indexing",
        "
        Since Rust uses 0 indexing, our loops will start at 0
        
        Which means when our snowballs variable is 4, \"Snowball!\" will get printed 5 times
        
        To fix this, we can change the start number to 1.
        
        1..=snowballs instead of 0..=snowballs
        
        This will print \"Snowball!\" 4 times (which is what we want)",
        (true, &[]),
        &[
            (
            "Example",
            "let cookies = 5;
            
            for cookie in 1..cookies {
                print!(\"I have eaten {} cookies!\", cookie);
            }"
            ),
            (
            "This will print",
            "
            \"I have eaten 1 cookies!\" on the first loop
            \"I have eaten 2 cookies!\" on the second loop
            \"I have eaten 3 cookies!\" on the third loop
            \"I have eaten 4 cookies!\" on the fourth loop
            \"I have eaten 5 cookies!\" on the fifth loop
            "
            )
        ],
    ),
    describe_function("feed_penguin", &["penguins: u8"], None,
        "Start at 1 and for each penguin in penguins, print \"I have fed {} penguins!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_1_info, "If Statements",
    describe_type("If",
        "If Statements are like match statements, but specifically for booleans
        ",
        (true, &[]),
        &[
            (
            "Example",
            "if 5 > 5 {
                print!(\"5 is greater than 5!\");
            } else {
                print!(\"5 is not greater than 5!\");
            }
            "
            ),
            (
            "Explanation",
            "This is like the true arm of a match statement:
            
            if 5 > 5 {
                print!(\"5 is greater than 5!\");
            }
            
            This is like the false arm of a match statement:
            
            else {
                print!(\"5 is not greater than 5!\");
            }"
            )
        ],
    ),
    describe_function("if_west", &["west: bool"], None,
        "If west, print \"Going west!\" else print \"Not going west!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_2_info, "If Statements",
    describe_type("If Else",
        "We can 'chain' if statements to check multiple booleans
        ",
        (true, &[]),
        &[
            (
            "Example",
            "if west {
                print!(\"Going west!\");
            } else if east{
                print!(\"Going east!\");
            } else {
                print!(\"Not going east or west!\");
            }
            "
            ),
            (
            "Explanation",
            "This checks if we are going west:
            
            if west {
                print!(\"Going west!\");
            }

            If we are not going west, check if we are going east:

            else if east{
                print!(\"Going east!\");
            }

            Otherwise:

            else {
                print!(\"Not going east or west!\");
            }
                
            "
            )
        ],
    ),
    describe_function("take_order", &["request: &str"], None,
        "Use chained if else statements to check:
        
        if request == \"water\" - print \"Here is your water!\"
        otherwise, check if request == \"soda\" - print \"Here is your soda!\"
        otherwise, check if request == \"juice\" - print \"Here is your juice!\"
        otherwise, print \"We don't have that here.\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_3_info, "If Statements",
    describe_type("let if",
        "We can use if statements to set the value of a variable
        ",
        (true, &[]),
        &[
            (
            "Example",
            "let cheese: &str = if cheddar_in_stock {
                \"Cheddar Cheese\"
            } else {
                \"Swiss Cheese\"
            };
            "
            ),
            (
            "Explanation",
            "let cheese: &str = - Creating a variable called cheese
            
            if cheddar_in_stock {} - If cheddar_in_stock is true, set cheese to equal \"Cheddar Cheese\"
            
            else {}; - Otherwise, set cheese to equal \"Swiss Cheese\""
            )
        ],
    ),
    describe_function("give_gift", &["toys_in_stock: bool"], None,
        "create a variable called gift, with type &str
        
        if toys_in_stock, set the value to \"trucks\"
        otherwise, set the value to \"candy\"
        
        print gift like this: \"Happy Birthday! I got you _\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_4_info, "If Statements",
    describe_type("if let Some()",
        "We can use if statements to use an Option<> value if it's Some
        ",
        (true, &[]),
        &[
            (
            "Example",
            "let mystery_box: Option<&str> = Some(\"Golden Ticket\");
            
            if let Some(prize) = mystery_box {
                print!(\"You got a {}!\", prize);
            } else {
                print!(\"Sorry, the box was empty.\")
            }"
            ),
            (
            "Explanation",
            "let mystery_box: Option<&str> - Created variable that will hold optional text (&str)

            if let Some(prize) = mystery_box 

                - If mystery_box contains a Some() value, unwrap the Some() from around the value
                Then, put the value inside a variable called prize

                print!(\"You got a {}!\", prize); - print the prize variable

            Otherwise, mystery_box contained None - print!(\"Sorry, the box was empty.\")
            "
            )
        ],
    ),
    describe_function("hunting_rifle", &["magazine: Option<u8>"], None,
        "Check if magazine contains a value using Some(). 
        
        If it does, print the number of bullets 
        left in this format: \"My gun has _ bullets left\".
         
        Otherwise, print: \"My gun has no bullets left\"."
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_5_info, "Understanding Ownership",
    describe_type("Ownership",
        "
        When a program runs, it stores all values in the computer's memory. 
        But, once a value is no longer needed, Rust automatically 
        cleans it up to free space for other data. 

        This process is called deallocating memory.

        Every value has a single owner. 
        When the owner goes out of scope, Rust automatically deallocates the memory.

        So far, we’ve used the &str type to work with text. 
        However, we cannot return a value of type &str from a 
        function.

        An &str is not the actual text itself. 
        It’s more like a label that points to where the text is stored in memory. 
        
        It's like writing on a sticky note, “The book is on the shelf.” 
        This sticky note isn’t the book; it just tells you where to find it.

        Because &str only refers to a string rather than owning it, 
        returning an &str from a function can be a problem. 

        To return a piece of text, we have to use the type String.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "fn get_joke() -> String {
                String::from(\"Why don’t skeletons fight each other? They don’t have the guts.\")
            }
                "
            ),
            (
            "Explanation",
            "fn get_joke() -> String  - We create a function with the return type String

            String::from() - Creates a string from the given piece of text"
            )
        ],
    ),
    describe_function("mission_log", &[], Some("String"),
        "Use String::from() and return:
        \"Log: Oxygen levels stable. Aliens unknown.\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_6_info, "Understanding Borrowing",
    describe_type("Borrowing",
        "
        Sometimes, you want to use a value without taking ownership of it. 
        You can let a function or another part of the program temporarily 
        use a value without giving away ownership. This is called borrowing.

        References (or &) are how we borrow values in Rust. 
        A reference is a way for the program to point to the 
        value without taking ownership of it or moving it.

        Think of it like giving someone a note that says \"The book is on my shelf\".
        You’re not giving them the actual book, just the information they need to find it.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub fn print_temperature(temp: &u8) {
                print!(\"The current temperature is: {}c\", temp);
            }
            "
            ),
            (
            "Explanation",
            "pub fn print_temperature - creates a function

            temp: &u8 - Temp is a reference to a u8 (number)

            print!(\"The current temperature is: {}c\", temp); - We print the number"
            )
        ],
    ),
    describe_function("dogs_in_park", &["dogs: &u8"], None,
        "Print dogs like this: \"There are _ dogs playing in the park!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_7_info, "Modifying a borrowed value",
    describe_type("Dereferencing",
        "
        When borrowing a value, you might want to modify the value it points to.

        This is done by \"following the reference\" to 
        get to the actual data. This is called dereferencing.

        To do this, we use the * operator. 
        This tells Rust, \"Give me the actual value, not just the reference.\"
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub fn squish_bug(bugs: &mut u8) {
                *bugs -= 1;
                print!(\"SQUISH! {} bugs left.\", bugs);
            }
            "
            ),
            (
            "Explanation",
            "pub fn squish_bug - creates a function

            bugs: &mut u8 - We take in a mutable reference to a number

            *bugs -= 1; - Dereference bugs and subtract 1

            print!(\"SQUISH! {} bugs left.\", bugs); - We print the remaining bugs"
            )
        ],
    ),
    describe_function("pop_balloon", &["balloons: &mut u8"], None,
        "Dereference balloons and subtract 1

        Print balloons like this: \"POP! There are now _ balloons left.\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task8_8_info, "Char Type",
    describe_type("char",
        "
        Char is a data type that only contains a single character

        Characters are denotated by single quotes ''

        Characters can be letters, numbers, symbols, emojis, or glyphs
        ",
        (true, &[]),
        &[
            (
            "Example",
            "fn first_letter() {
                let my_first_letter: char = 'a';
                print!(\"Hey Buddy, today we're learning {}\", my_first_letter);
            }
            "
            ),
            (
            "Explanation",
            "let my_first_letter: char = 'a'; - A variable containing the character 'a'

            print!(\"Hey Buddy, today we're learning {}\", my_first_letter);
                - Printing the variable"
            )
        ],
    ),
    describe_function("knight_move", &[], None,
        "Create a variable called piece.
        Set the type to char and the value to 'K'
        
        Print piece like this: \"_ to C7!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task9_1_info, "Importing Crates",
    describe_type("Crates",
        "
        Rust allows us to add extra functionality to our programs using crates. 
        A crate is like a toolbox filled with useful code written by other developers.
        ",
        (true, &[]),
        &[
            (
            "Example - Let's add the crate 'rand' to generate random numbers.",
            "In the terminal, we will run the command 'cargo add rand'

            Then, at the top of our tasks.rs file, we add the line 'use rand::Rng;
            
            pub fn random_number() {
                let mut rng = rand::thread_rng(); 
                let number: u8 = rng.gen_range(1..=10);
                println!(\"Your random number is: {}\", number);
            }
            "
            ),
            (
            "Explanation",
            "Running the command 'cargo add rand' will add the 
            latest version of rand to our dependencies.

            You can see all your dependencies in the Cargo.toml file

            At the top of our file, we add 'use rand::Rng;' this tells 
            the compiler that we want to use the Rng part of the rand crate in this file.

            Rng let's us create a random number generator.
            
            let mut rng = rand::thread_rng(); - Create a random number generator

            let number: u8 = rng.gen_range(1..=10); - Generate a number between 1 and 10

            println!(\"Your random number is: {}\", number); - Print your random number
            "
            )
        ],
    ),
    "We will use the rot13 crate to encode our message with the rot13 cipher.

    In the terminal, run 'cargo add rot13'

    Don't forget to add the import 'use rot13::rot13;' to the top of your file;
    \n".to_owned()
    +&describe_function("encode_message", &["message: &str"], None,
        "Create a variable called encoded, and set it to rot13(message);
            - encoded is now the message parameter in code
        
        Print encoded like this: \"I have a secret to tell you. _\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task9_2_info, "Crate features",
    describe_type("--features",
        "
        Some crates have optional features that can be enabled to add extra functionality. 
        By default, these features are disabled to reduce unnecessary code.

        To enable a feature in a crate, you use the --features flag when adding the crate.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "In the terminal, we run the command 'cargo add random-word --features de'


            use random_word::Lang;
            
            pub fn a_german_word() {
                let my_word = gen(Lang::De);
                print!(\"Your random word is: {}\", my_word);
            }
            "
            ),
            (
            "Explanation",
            "Running the command 'cargo add random-word --features de' 
            will add the latest version of random-word crate with the de features
            to our dependencies.

            You can see all your dependencies in the Cargo.toml file

            At the top of our file, we add 'use random_word::Lang;' 
            this tells the compiler that we want to use the Lang and 
            parts of the crate in this file.


            pub fn a_german_word() - creates a function

            let my_word = gen(Lang::De);
                - Grabs a random word in the German language

            print!(\"Your random word is: {}\", my_word); - Print your random word
            "
            )
        ],
    ),
    "We will use the random-word crate to generate a random french word.

    In the terminal, run 'cargo add random-word --features fr'

    Don't forget to add the import 'use random_word::Lang;'
    \n".to_owned()
    +&describe_function("french_word", &[], None,
        "Create a variable called my_word and set it to random french word
        
        Print my_word like this: \"I'm so french. _!\""
    ).replace("\n", "\n    "),
    None,
    None
);


make_test_info!(
    task9_3_info, "The Result Type",
    describe_type("Result",
        "
        The Result type is used for error handling. 

        It is an enum that represents either a successful outcome (Ok) 
        or an error (Err). 

        The Result type is used by functions to return either a value on success 
        or an error on failure.

        Result type is defined in this format: Result<u8, String>

        The first type represents which type the Ok value will be
        The second type represents which type the Err value will be

        The example shows u8 and String, but these can be any type.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub fn buy_bread(count: u8, in_stock: u8) -> Result<u8, String> {
                match count <= in_stock {
                    true => Ok(count),
                    false => Err(String::from(\"Not enough loaves for sale.\"))
                }
            }"
            ),
            (
            "Explanation",
            "pub fn buy_bread(count: u8, in_stock: u8) - Creating a function requiring two u8s

            Result<u8, String> 
                - Returns a Result where the Ok value will be a u8 
                  and the Err value will be a String

            match count <= in_stock - Match count is less than or equal to in_stock

            true => Ok(count) - true returns the Ok() Result variant, with count as the value

            false => Err(String::from(\"Not enough loaves for sale.\")) 
                - false returns the Err Result variant
            "
            )
        ],
    ),
    describe_function("cook_chicken", &["temp: u8"], Some("Result<String, String>"),
        "Use a match statment to check if temp >= 165
        
        true, return Ok(String::from(\"Chicken is cooked!\"))
        false, return Err(String::from(\"Gross! Chicken is raw.\"))
        "
    ).replace("\n", "\n    "),
    None,
    None
);


make_test_info!(
    task9_4_info, "Empty Ok Type",
    describe_type("Ok(())",
        "
        Err() is required to have a defined type and a value.

        But Ok() does not need to have a defined type or a value.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub fn send_email(message: &str) -> Result<(), String> {
                match message.is_empty() {
                    false => Ok(()),
                    true => Err(String::from(\"Invalid Email. Message is empty\"))
                }
            }"
            ),
            (
            "Explanation",
            "pub fn send_email(message: &str) - Creating a function requiring an &str

            Result<(), String> 
                - Returns a Result where the Ok doesn't have a value 
                  and the Err value will be a String

            match message.is_empty() - Check if message is an empty string

            false => Ok(()) - false returns the Ok() Result variant, with no value - ()

            true => Err(String::from(\"\")) - true returns an error message
            "
            )
        ],
    ),
    describe_function("borrow_book", &["title: &str"], Some("Result<(), String>"),
        "Use a match statment to check if title == \"Starman Jones\"
        
        true, return Ok(())
        false, return Err(String::from(\"Book not available\"))
        "
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task9_5_info, "Using Results",
    describe_type("Result",
        "",
        (true, &[]),
        &[
            (
            "Example",
            "pub fn buy_bread(count: u8, in_stock: u8) -> Result<u8, String> {
                match count <= in_stock {
                    true => Ok(()),
                    false => Err(\"Not enough loaves for sale\")
                }
            }
                
            pub fn shopping() {
                let result = buy_bread(4, 12);

                match result {
                    Ok(_) => print!(\"I bought 4 bread loaves.\"),
                    Err(error) => print!(\"I couldn't buy bread. {}, error),
                }
            }
            
            "
            ),
            (
            "Explanation",
            "pub fn shopping() - Creating a new function

            let result = buy_bread(4, 12); - Run buy_bread and store the result in a variable

            match result - Creating a match statement for the result variable

            Ok(_) - If the variant was Ok, we print \"I bought 4 bread loaves.\"

            Err(error) - If the variant was Err, we catch the value in a variable called error and print
            "
            )
        ],
    ),
    describe_function("reserve_book", &[], None,
        "Create a variable called my_book with type &str
        and set the value to the title of your favorite book
        
        Run your borrow_book function pass in my_book as the parameter,
        and store the result in a variable called 'result'

        match on result
        
        Ok(_), print my_book: \"Successfully reserved _\"
        Err(error), print error: \"Error: _\"
        "
    ).replace("\n", "\n    "),
    None,
    None
);


make_test_info!(
    task9_6_info, "? Operator",
    describe_type("The ?",
        "The ? operator is used to handle errors in Rust, specifically with Result types. 
        
        When applied to a Result, the ? operator automatically propagates errors if the result is an Err, 
        and it lets you continue with the value inside the Ok variant if it’s successful. 
        
        It helps simplify error handling without the need for explicit match statements.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "fn check_ingredient(ingredient: &str) -> Result<String, String> {
                match ingredient {
                    \"coffee\" => Ok(\"freshly ground coffee\".to_string()),
                    \"milk\" => Ok(\"whole milk\".to_string()),
                    \"sugar\" => Ok(\"cane sugar\".to_string()),
                    _ => Err(\"Ingredient out of stock!\".to_string()),
                }
            }

            fn prepare_coffee() -> Result<String, String> {
                let coffee = check_ingredient(\"coffee\")?;
                let milk = check_ingredient(\"milk\")?;
                let sugar = check_ingredient(\"sugar\")?;
                
                Ok(\"Your coffee is ready!\".to_string())
            }
            "
            ),
            (
                "Explanation",
            " 
                let coffee = check_ingredient(\"coffee\")?; 
                    - If coffee returns the Ok variant, set this variable to the content of Ok
                    - If coffee returns the Err variant, this function will return the Err

                .to_string() this is a method that will convert an &str to a String
            "
            )
        ],
    ),
    describe_function("eat_chicken", &[], Some("Result<(), String>"),
        "Declare a variable named chicken.

        Set the value to the result of calling cook_chicken, 
        passing 170 as the temp parameter.
        
        Use the ? operator to handle errors instead of 
        manually matching the result with a match statement.

        Print chicken: \"Yummy, yummy! _\"
        
        Return Ok(())"

    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task9_7_info, "Asyncronous (Async) Functions",
    describe_type("Async",
        "
        Async functions let our program wait for something to finish without stopping everything else.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "use tokio::time::{sleep, Duration};

            pub async fn boil_water() {
                print!(\"Boiling water...\");
                sleep(Duration::from_secs(3)).await;
                print!(\"Water is 212F\");
            }"
            ),
            (
            "Explanation",
            "
            use tokio::time::{sleep, Duration};
                - We import sleep and Duration, tokio is not a default crate. 
                  We must run 'cargo add tokio --features time'

            pub async fn boil_water() - We create an async function called boil_water

            print!(\"Boiling water...\"); - Create a print

            sleep(Duration::from_secs(3)).await; - Uses tokio and duration to wait for 3 seconds

            print!(\"Water is 212F\"); - Prints after the 3 second wait
            "
            )
        ],
    ),
    "Add tokio as a dependency (cargo add tokio)

    Add imports for Duration and sleep to the top of tasks.rs
    \n".to_owned()
    +&describe_function("brew_coffee", &[], None,
        "print \"Brewing coffee...\".

        run sleep to wait for 5 seconds

        then, print \"Coffee brewed!\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task9_8_info, "The Await Method",
    describe_type(".await",
        "
        To pause and wait for an async function to finish 
        before moving on, we use the .await method.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub async fn boil_water() {
                print!(\"Boiling water...\");
                sleep(Duration::from_secs(3)).await;
                print!(\"Water is 212F\");
            }

            pub fn make_tea() {
                boil_water().await;
                print!(\"Steep tea...\");
            }
            "
            ),
            (
            "Explanation",
            "pub fn make_tea - Creating a function called make_tea

            boil_water().await; - We call our boil_water function and wait for it to finish

            print!(\"Steep tea...\"); - After the water is boiling, we can steep our tea"
            )
        ],
    ),
    describe_function("serve_customer", &[], None,
        "Run brew_coffee and wait for it to finish
        
        Then, print \"Here's fresh coffee!\'"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task10_1_info, "Type Parameters",
    describe_type("Type Parameters",
        "
        Type parameters allow functions and structs to work with multiple types 
        instead of a single, fixed type. 
        
        This makes code more flexible and reusable.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub fn announce_message<T: std::fmt::Display>(message: T) {
                print!(\"Attention: {:?}\", message);
            }

            pub fn yell() {
                announce_message::<&str>(\"The event is starting soon!\");
            }"
            ),
            (
            "Explanation",
            "pub fn announce_message - The start of a new function

            <T: std::fmt::Display> 
                - This tells the compiler that we will define the type, when calling the function  
                - But, it must implement Display (can be printed)
                - T can be any type. u8, &str, etc. as long as it can be printed

            (message: T) - This function requires a parameter with the defined type

            print!(\"Attention: {}\", message); - Printing the parameter

            announce_message::<&str>()
                - In our yell function, we run announce_message and define the T type as &str
            "
            )
        ],
    ),
    describe_function("tell_mark<T>", &["message: T"], None,
        "print the message parameter: \"Hey Mark, I gotta tell you: _.\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task10_2_info, "Type Parameters with Structs",
    describe_type("Type Parameters",
        "
        We can use type parameters for struct fields aswell.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub struct Pet<T> {
                pub name: String,
                pub age: T,
                pub breed: String,
            }

            impl<T> Pet<T> {
                pub fn new(name: String, age: T, breed: String) -> Self {
                    Pet { name, age, breed }
                }

                pub fn get_age(&self) -> T {
                    self.age
                }
            }"
            ),
            (
            "Explanation",
            "pub struct Pet<T> - We create a structure called Pet with a type parameter

            age: T - The age field has the T type

            impl<T> Pet<T> - The T type must be specified here as well"
            )
        ],
    ),
    describe_structure(
        "Reindeer<T>",
        &["
        pub name: String", "pub age: T", "pub favorite_candy: String"],
        &[],
        &[],
    )+&"
    \n\n\nCreate an impl block for Reindeer. Inside the Reindeer impl block:\n\n".replace("\n", "\n    ")
    +&describe_function(
        "new", &["name: String", "age: T", "favorite_candy: String"], Some("Reindeer"),
        "Return an instance of Reindeer using the parameters for the field values"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task10_3_info, "Type Parameters with Enums",
    describe_type("Type Parameters",
        "
        We can also use type parameters for enum fields.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "pub enum Transmission<T> {
                Message(T),
                SignalLost,
                Coordinates(u8, u8),
            }

            impl<T> Transmission<T> {
                fn send_message(message: T) -> Self {
                    Transmission::Message(message)
                }
            }"
            )
        ],
    ),
    describe_enum(
        "MysteryBox<T>",
        &["
        Contains(T),", "Empty"],
        &[],
        &[],
    )+&"
    \n\nCreate an impl block for MysteryBox. Inside the MysteryBox impl block:\n\n".replace("\n", "\n    ")
    +&describe_function(
        "new", &["item: Option<T>"], Some("MysteryBox<T>"),
        "Create a match statement for item
        
        If Some(content), return MysteryBox::Contains(content)
        If None, return MysteryBox::Empty"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task10_4_info, "Traits",
    describe_type("Traits",
        "
        Instead of defining the same method or function for different types (Structs, Enums, etc)
        Traits allow us to create and implement it for any type that needs it.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "
            pub trait Noise {
                fn yell(&self) {
                    print!(\"Get off my lawn!\");
                }
            }

            pub struct Neighbor;
            pub struct OldMan;

            impl Noise for Neighbor {}
            impl Noise for OldMan {}

            pub fn chaos() {
                let gerald = OldMan;
                let patricia = Neighbor;

                gerald.yell();
                patricia.yell();
            }
                "
            ),
            (
            "Explanation",
            "
            pub trait Noise - We create a trait called Noise
            
            fn yell(&self) 
                - We create a function called yell. Trait functions are not made public (pub)

            print!(\"Get off my lawn!\"); - yell() prints \"Get off my lawn!\"
            
            pub struct Neighbor;
            pub struct OldMan;
                - We create two structs that don't have any fields

            impl Noise for Neighbor {} - We implement the Noise trait for our structs
            impl Noise for OldMan {}

            pub fn chaos() - We create a function called chaos

            let gerald = OldMan; - We create two variables set to oour two structs
            let patricia = Neighbor;

            gerald.yell();
            patricia.yell();
                - We can now run yell() on both of our structs, 
                  without defining yell in each structs impl block
            "
            )
        ],
    ),
    "\n".to_owned()
    +&describe_structure("WashingMachine", &[], &[], &[])+&"\n\n"
    +&describe_structure("DryingMachine", &[], &[], &[])+&"\n"
    +&"
    \nCreate a trait called Machine. Inside the Machine trait block:\n\n".replace("\n", "\n    ")
    +&describe_function(
        "start", &["&self"], None,
        "print: \"Starting machine....\""
    ).replace("\n", "\n    ")
    +&"
    \n\n Implement Machine for WashingMachine and DryingMachine".replace("\n", "\n    "),
    None,
    Some("Implement Machine for WashingMachine and DryingMachine")
);


make_test_info!(
    task10_5_info, "Traits",
    describe_type("Traits",
        "
        Now, let's run our start function on our new structs
        ",
        (true, &[]),
        &[],
    ),
    describe_function(
        "clean_laundry", &["washer: WashingMachine", "dryer: DryingMachine"], None,
        "run start() on both washer and dryer"
    ).replace("\n", "\n    "),
    None,
    None
);


make_test_info!(
    task10_6_info, "Traits",
    describe_type("Traits",
        "
        We can write custom code for a specific implementation of a trait's function
        ",
        (true, &[]),
        &[
            (
            "Example",
            "
            pub trait Noise {
                fn yell(&self) {
                    print!(\"Get off my lawn!\");
                }
            }

            pub struct Neighbor;
            pub struct OldMan;
            pub struct Mother;

            impl Noise for Neighbor {}
            impl Noise for OldMan {}

            impl Noise for Mother {
                fn yell(&self) {
                    print!(\"Dinner's ready!\");
                }
            }

            "
            ),
            (
            "Explanation",
            "
            pub struct Mother; - We create a third struct, Mother

            impl Noise for Mother - This impl block can only contain functions defined in Noise

            fn yell(&self) 
                - Functions being implemented from a trait, 
                  must have the same name, parameters, and return type 
                  as the original definition
            
            print!(\"Dinner's ready!\"); - The function body, does not have to be the same."
            )
        ],
    ),
    "\n".to_owned()
    +&describe_structure("Oven", &[], &[], &[])+&"\n\n"
    +&"
    \nImplement Machine for Oven. Inside the impl block:\n\n".replace("\n", "\n    ")
    +&describe_function(
        "start", &["&self"], None,
        "print: \"Preheating to 450f....\""
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task10_7_info, "Derived Traits",
    describe_type("#[derive()]",
        "
        #[derive()] automatically implements 
        common traits for a struct or enum. 
        
        This reduces boilerplate code when working with 
        standard traits like Debug, Clone, and PartialEq.

        We will cover these traits and other common ones soon.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "
            #[derive(Clone)]
            struct Creature {
                name: String,
                power: u8,
            }

            pub fn two_dragons() -> (Creature, Creature) {
                let dragon = Creature {
                    name: String::from(\"Dragon\"),
                    power: 255,
                };

                let duplicate = dragon.clone();

                (dragon, duplicate)
            }
            "
            ),
            (
            "Explanation",
            "
            #[derive(Clone)] - Here we 'derive' the Clone trait for our Creature struct

            struct Creature - We create a struct with two fields

            pub fn two_dragons - A function that will return a tuple of two Creature instances
            
            let dragon = Creature - A variable called dragon, set to a Creature instance

            let duplicate = dragon.clone() - Because this struct implements Clone, we can make clones

            (dragon, duplicate) - Returning a tuple of dragon and duplicate (two Creature instances)
            "
            )
        ],
    ),
    "\n".to_owned()
    +&describe_structure("Fish", &["pub specie: String", "pub scales: u8"], &[], &[])+&"\n"
    +&"
    \n    Derived Traits:

        Clone\n\n\n"
    +&describe_function(
        "fish_duplicater", &["fish: Fish"], Some("(Fish, Fish)"),
        "Create a variable called duplicate set to a 
        clone of the fish parameter
        
        Return the original fish and the duplicate in a tuple"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task10_8_info, "Iterators",
    describe_type(".iter()",
        "
        Iterators allow you to traverse elements in a collection, such as vectors, 
        without needing to use indexing manually.
        ",
        (true, &[]),
        &[
            (
            "Example",
            "
            pub fn print_numbers() {
                let numbers = vec![1, 2, 3, 4, 5];

                for num in numbers.iter() {
                    print!(\"{}\", num);
                }
            }
            "
            ),
            (
            "Explanation",
            "
            let numbers = vec![1, 2, 3, 4, 5]; - We create a vector of 5 numbers

            for num in numbers.iter() - We loop through numbers

            print!(\"{}\", num); - Printing each number
            "
            )
        ],
    ),
    describe_function(
        "double_values", &["values: Vec<u8>"], None,
        "Create a for loop and interate through values
        multiply each value by 2, then print it"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task10_9_info, "Iterators",
    describe_type(".into_iter()",
        "
        The into_iter() method consumes the original collection and transforms 
        it into an iterator that takes ownership of the data.
        
        This means the original collection is no longer accessible 
        after calling into_iter(), but it can be used to process and 
        consume the elements of the collection.

        ",
        (true, &[]),
        &[],
    )+"\n    "
    +&describe_type(".map()",
        "
        The .map() method is an iterator method that allows you to apply a function 
        to each element in a collection, transforming it into a new value. 
        
        It creates a new iterator where each item is the result of applying 
        the given function to the corresponding item from the original collection.
        
        ",
        (true, &[]),
        &[],
    )+"\n    "
    +&describe_type(".collect()",
        "
        The collect() method is used to transform an iterator into a collection
        (e.g., Vec, HashMap, etc.). 
        
        It gathers the results of an iterator into a new data structure.

        ",
        (true, &[]),
        &[(
            "Example",
            "
            pub fn double_numbers() {
                let numbers = vec![1, 2, 3, 4, 5];
                let doubled: Vec<u32> = numbers.into_iter().map(|x| x * 2).collect();
            }
            "
            ),
            (
            "Explanation",
            "
            let numbers = vec![1, 2, 3, 4, 5]; - We create a vector of 5 numbers

            numbers.into_iter() - This method turns the vector of numbers, into an interator

            .map(|x| x * 2) 
                - This is a function that will run on each value.
                x is the variable holding the current item
                we multiply x by 2
            
            .collect() - We collect the iterator items back into a vector
            "
        )],
    ),
    describe_function(
        "increase_rations", &["rations: Vec<u8>"], Some("Vec<u8>"),
        "use .into_iter(), .map(), and .collect()
        on rations to multiply each item by 2 
        and return the items as a vector"
    ).replace("\n", "\n    "),
    None,
    Some("** THIS IS THE LAST TASK **")
);
