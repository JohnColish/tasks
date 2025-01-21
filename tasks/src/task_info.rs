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
    describe_type("Functions(fn)",
        "Functions are used to define a specific set of actions or logic that can
        be reused by using/calling the function. Every function has five key parts",
        (true, &[
        "pub fn: This is the keyword used to declare a public function.",
        "Name: The name of the function is like a label or identifier,
            you use it when you want to call the function.",
        "Parameters: These are optional variables used by the function to do its job.
            They are listed inside parentheses after the name, in the form of \"name: Type\".",
        "Return Type: An optional Type if the function returns a value,
            the return type is declared after the parameters using \"-> Type\"",
        "Function Body: The body contains the code that performs the function's logic.
            It is enclosed in curly braces {}, where the parameters can be used
            to compute the result, and the return type (if there is one) is returned."
        ]),
        &[(
            "For a function that accepts an &str and returns u8",
            "pub fn function_name(my_str: &str) -> u8 {
                //function body
            }"
        ), (
            "For a function that accepts a &str and u8 and returns nothing",
            "pub fn function_name(my_name_str: &str, my_favorite_u8: u8) {
                //function body
            }"
        ), (
            "For a function that accepts no parameters and returns nothing",
            "pub fn function_name() {
                //function body
            }"
        )]
    ),
    describe_function("hello_everyone", &[], None,
        "print! 'Hello, Everyone!'"
    ),
    None,
    None
);

make_test_info!(
    task1_3_info, "&str Type and Formatting Strings",
    describe_type("&str",
        "A &str is a Type that is easy to use but cannot be easily stored.
        Its value is declared by using double quotes",
        (true, &[]),
        &[("For the string containing Hello, World!", "\"Hello, World!\"")],
    )+TYPE_SEP+
    &describe_type("Formatting String",
        "A formatting string is a &str used in functions like print!.
        It can contain plain text, such as \"Hello, everyone\".
        It can also include various symbols to better format your print.
        For example, you can type {} anywhere in the string and it will
        act as a placeholder. You can then fill the placeholder by giving
        print! another parameter. This additional parameter will be filled
        into the print output where the placeholder is.",
        (true, &[]),
        &[(
            "To print a dog breed out like this: \"Hello, Golden Retriever!\"",
            "print!(\"Hello, {}!\", \"Golden Retriever\")"
        )],
    ),
    describe_function("hello_pet", &[], None,
        "print 'Hello, Benji!' using a placeholder like the example",
    ),
    None,
    None
);

make_test_info!(
    task1_4_info, "Function Parameters",
    describe_type("Parameters",
        "A parameter is like a placeholder that a function uses.
        When you define a function, you list the parameters inside
        parentheses () after the function's name, specifying their name and
        Type (like u8 or &str).

        However, you don’t give them actual values at this point. Instead,
        when the function is used or called, the caller provides the specific
        values for these placeholders. This allows the function to work with
        different inputs and produce results based on those inputs.

        Parameters make functions more flexible and reusable.
        ",
        (true, &[]),
        &[(
            "For a function with a &str and u8 as params",
            "my_function(a_random_str: &str, my_favorite_u8: u8) {
                //Function body
            }",
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
        "Functions are useful because they let us reuse code or logic without
        rewriting it. To use, or call, a function, write its name followed by
        parentheses (). If the function has parameters, you provide the values
        inside the parentheses when calling it.
        ",
        (true, &[]),
        &[
            ("To call a function",
            "function_name(param_value);"),
            ("To call a function that requires a u8",
            "function_name(18);"),
            ("To call a function that requires a String",
            "function_name(\"Hello\");")
        ],
    ),
    describe_function("hello_mitch", &[], None,
        "use your previous function 'hello_user' with 'mitch' as the parameter
        to print 'Hi, my name is Mitch'"
    ),
    Some( "Do not use print! directly in this function"),
    None
);

make_test_info!(
    task1_6_info, "Formal Hello",
    describe_type("Printing Multiple Parameters",
        "print! can print multiple variables, but each variable requires
        its own set of {} to specify its position in the text.

        Additionally, functions can accept more than one parameter.
        When the function is called the values must be passed in the same
        order as when they're declared.
        ",
        (true, &[]),
        &[
            (
            "To print two variables, name and age",
            "'Hello, my name is Mitch and I am 38 years old'",
            "print!(\"Hello, my name is {} and I am {} years old\", name, age);"
            ),
            (
            "To call a function with multiple parameters",
            "pub fn a_and_b(a: u8, b: u8) {
                print!(\"this is a: {}\", a);
                print!(\"this is b: {}\", b);
            }

            a_and_b(2, 5); //Will print \"this is a: 2\", \"this is b: 5\"
            a_and_b(5, 2); //Will print \"this is a: 5\", \"this is b: 2\""
            )
        ],
    ),
    describe_function("formal_hello", &["first_name: &str", "last_name: &str"], None,
        "print 'Hi, my name is ' followed by first_name then last_name with a space in between"
    ),
    None,
    None
);

make_test_info!(
    task1_7_info, "Parameter Reuse",
    describe_type("Reusing Parameters",
        "Parameters and variables can be used multiple times if you
        'borrow' them. The borrow symbol is denoted by &. We will discuss
        borrowing any type later but notice that the '&str' has the
        borrow symbol and can there be used mulitple times.",
        (true, &[]),
        &[],
    ),
    describe_function("hello_twice", &["first_name: &str", "last_name: &str"], None,
        "use your previous function 'hello_user' to print the first name,
        then use 'formal_hello' to print both first_name and last_name"
    ),
    Some("Do not use print! directly in this function"),
    None
);

// ---------------- SECTION TWO ---------------- //

make_test_info!(
    task2_1_info, "First number Type: u8",
    describe_type("u8",
        "A u8 is very small number that can only be between 0 and 255.
        A u8, commonly referred to as a byte, cannot be negative.
        It is preferred to use this number type when possible.
        u8s can be printed the same way strings can be.",
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
        "Functions can return values that are a result of the logic inside.
        If a function returns a type, after calling a function it becomes
        the value it returned.",
        (false, &[
        "When defining (creating) a function, follow the parentheses () with an arrow ->,
            followed by the type of the return value (e.g., u8, &str, etc.).",
        "The return value can be specified with the return keyword, but this
            is only needed when you trying to return a value before the function
            is finished running. The proper way to return a value is to have it
            be on the last line of your function without a ';'"
        ]),
        &[
            ("You can define the return type of a function",
            "fn my_name() -> &str {
                \"Mitch\"
            }"),
            ("You can print a function value just like any variable",
            "print!(\"Hello, my name is {}\", my_name());")
        ],
    ),
    describe_function("my_age", &[], Some("u8"),
        "return your age"
    ),
    None,
    Some("This is an exception to the hard code rule where you can type your age into the function")
);

make_test_info!(
    task2_3_info, "Using a Returned Value",
    describe_type("Returned Values",
        "If a function has a return type, calling the function turns it into the value.",
        (true, &[]),
        &[(
            "For example, if a function called my_name() returns a &str,
            you can call it and then print it just like any other &str",
            "print!(\"Hello, my name is {}\", my_name());"
        )],
    ),
    describe_function("say_my_age", &[], None,
        "print 'My age is ' followed by the result of my_age()"
    ),
    Some("Do not set your age directly. Use my_age() instead"),
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
        "subtract 'b' from 'a' and return the result"
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
        corresponding \"=>\" is ran. A pattern matches if the value == pattern",
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

make_test_info!(
    task4_1_info, "First Structure",
    describe_type("Structures",
        "Structures are exactly like Types but they can be constructed by anyone. All
        Structures consist of 5 parts",
        (true, &[
            "pub struct: Is the keyword to declare a structure",
            "Name: The name of a structure is always CammelCase, Meaning the first
            letter of every word is capitalized and there are no spaces or underscores.",
            "fields: Fields are the variables that the structure has inside of it.
                All fields must be prefixed with pub as well!",
            "Constructors: These are functions that may accept parameters but always
            return the structure",
            "Methods: These are functions that can be run on a structure, they can accept parameters, but they also have access to all the fields of a structure."
        ]),
        &[
            (
            "Format",
            "pub struct Name {
                pub field_name: FieldType,
                pub field_name: FieldType,
            }

            //Constructors and Methods go in an implmentation shown later
            "
            ),
            (
            "Example",
            "pub struct Date {
                pub year: u8,
                pub month: u8,
                pub day: u8,
            }"
            ),
        ],
    ),
    describe_structure(
        "DriversLicense",
        &["pub issued: u8", "pub expires: u8"],
        &[],
        &[],
    ),
    None,
    None
);

make_test_info!(
    task4_2_info, "Brace Constructor",
    describe_type("Brace '{}' constructor",
        "The brace constructor is used to create an instance of your structure. Previously
        you described the Type DriversLicense which is like any other type &str or u8 etc.
        But now you are assigning it a value like \"hello, world\" to &str.",
        (false, &[]),
        &[
            (
            "Format",
            "pub struct Name {
                pub field_name: FieldType,
            }

            let my_struct: Name = Name{field_name: value);"
            ),
            (
            "Example",
            "pub struct Date {
                pub year: u8,
                pub month: u8,
                pub day: u8,
            }

            let my_date = Date{year: 24, month: 12, day: 25};"
            ),
        ],
    ),
    describe_function(
        "my_new_dl", &[], Some("DriversLicense"),
        "Create a new drivers license with the brace constructor, issued is 4 and expires is 12"
    ),
    None,
    None
);

make_test_info!(
    task4_3_info, "Using Fields",
    describe_type("Structure Fields",
        "Structure fields are exactly like variables, contained inside of the structure.
        To 'access' them you need to state the structure name followed by '.' and the field name.",
        (true, &[]),
        &[
            (
            "Format",
            "pub struct Name {
                pub field_name: FieldType,
            }

            let my_struct: Name = Name{field_name: field_value};

            print!(\"my field contains {}\", my_struct.field_name);"
            ),
            (
            "Example",
            "pub struct Date {
                pub year: u8,
                pub month: u8,
                pub day: u8,
            }

            let my_date = Date{year: 1, month: 12, day: 14};

            print!(\"The year is {}\", my_date.year); //Prints \"The year is 1"
            ),
        ],
    ),
    describe_function(
        "print_drivers_license",
        &["a_dl: DriversLicense"],
        None,
        "print the issued year and the expires year of a_dl with a '-' inbetween",
    ),
    None,
    None
);

make_test_info!(
    task4_4_info, "Constructors",
    describe_type("Structure Constructors",
        "Constructors are functions that are attached to the Structure Type,
        To attach constructors or methods to a Structure Type you must place them
        in an (impl)ementation block for the Structure. A Structure can only have one
        impl block but you can place any methods or constructors inside",
        (false, &[
            "Constructors are used to build or construct the structure often limiting or providing
            data for the structure.",
            "Constructors always go at the top of the (impl)ementation block.",
            "Constructors are almost always named 'new' and sometimes 'from'",
            "Constructors must be called on the Structure TypeName using '::' to call it",
            "'::' is a Path Seperator used for accessing the inside of Types and Modules"
        ]),
        &[
            (
            "Format",
            "pub struct Name {
                pub field_name: FieldType,
            }

            impl Name {
                pub fn new(parameter_name: Type) -> Name {
                    //Function body
                }
            }

            let my_struct: Name = Name::new(parameter_value);"
            ),
            (
            "Example",
            "pub struct Date {
                pub year: u8,
                pub month: u8,
                pub day: u8,
            }

            impl Date {
                pub fn new(my_year: u8) -> Date {
                    Date{year: my_year, month: 12, day: 25}
                }
            }

            let christams_of_24 = Date::new(24);"
            ),
        ],
    ),
    "Add a constructor to an impl block for DriversLicense:\n        ".to_string()
    +&describe_function(
        "new", &["issued: u8"], Some("DriversLicense"),
        "Create a new DriversLicense where issued is given from the parameter
        and expires is issued plus 8"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task4_5_info, "Methods",
    describe_type("More Operators",
        "More Operators that will be handy in this task each of these takes two numbers and
        returns a bool",
        (false, &[
            "> (Greater than): checks if one number is greater than the other",
            "< (Less than): checks if one number is less than the other",
            "<= (Less than or equal): checks if one number is less than or equal to the other",
            ">= (Greater than or equal): checks if one number is greater than or equal to the other",
        ]),
        &[
            ("(5 > 5) becomes", "false"),
            ("(5 > 6) becomes", "false"),
            ("(5 > 3) becomes", "true"),
            ("(5 < 5) becomes", "false"),
            ("(5 < 6) becomes", "true"),
            ("(5 < 3) becomes", "false"),
            ("(5 <= 5) becomes", "true"),
            ("(5 <= 6) becomes", "true"),
            ("(5 <= 3) becomes", "false"),
            ("(5 >= 5) becomes", "true"),
            ("(5 >= 6) becomes", "false"),
            ("(5 >= 3) becomes", "true"),
        ],
    )+TYPE_SEP+
    &describe_type("Structure Methods",
        "Methods are functions that can be run on an instance of a structure.",
        (false, &[
            "Methods are functions in the impl block that have a self parameter",
            "self is a reserved parameter name that allows this method to be run
            on an instance of the Structure",
            "Methods are called by doing struct_variable.method_name(params)"
        ]),
        &[
            (
            "Format",
            "pub struct Name {
                pub field_name: FieldType,
            }

            impl Name {
                pub fn my_method(self: Name, parameter_name: Type) -> ReturnType {
                    //Function body
                }
            }

            let my_struct: Name = Name{field_name: field_value};

            let my_value: ReturnType = my_struct.my_method(parameter_value);"
            ),
            (
            "Example",
            "pub struct Date {
                pub year: u8,
                pub month: u8,
                pub day: u8,
            }

            impl Date {
                pub fn months_till_christmas(self: Date) -> u8 {
                    12 - self.month
                }
            }

            let my_date = Date{year: 1, month: 7, day: 14};

            my_date.months_till_christmas() == 5;"
            ),
        ],
    ),
    "Add a method to the bottom of the  impl block for DriversLicense:\n        ".to_string()
    +&describe_function(
        "is_valid", &["self: DriversLicense", "current_year: u8"], Some("bool"),
        "Return true if current_year is greater or equal to issued, and is less than expires"
    ).replace("\n", "\n    "),
    None,
    None
);

make_test_info!(
    task4_6_info, "Matching on Types",
    describe_type("'_' WildCard",
        "In section 3 we matched on the 'bool' type which only had two values 'true' or 'false'.
        But the match statement can be used on most any Type. But if you were to match on the u8
        that would be 128 branches of the match statement, because a u8 can be 0 to 127.
        There are two solutions, the first is the wildcard pattern '_'. This will run no matter
        what the value is.",
        (true, &[]),
        &[
            (
            "Example",
            "match 24 {
                3 => {
                    print!(\"I am 3!\");
                },
                18 => {
                    print!(\"18 years old!\");
                },
                _ => {
                    print!(\"I am not 3 or 18 years old.\");
                }
            }"
            )
        ],
    ),
    describe_function("print_apples", &["apples: u8"], None,
        "Use a match statement on age to print:
            if apples is 1 print \"One apple\",
            if apples is 2 print \"Two apples\",
            if apples is 3 print \"Three apples\",
            Otherwise print \"More than three apples\""
    ),
    None,
    None
);

make_test_info!(
    task4_7_info, "Matching on Types",
    describe_type("Variable Catch",
        "The second solution is using a variable to catch the value of the variable if none
        of the other match branches succeeded.",
        (true, &[]),
        &[
            (
            "Example",
            "match 20 {
                3 => {
                    print!(\"I am 3!\");
                },
                18 => {
                    print!(\"18 years old!\");
                },
                age => {
                    print!(\"{} years am I\", age);
                }
            }//Prints \"20 years am I\""
            ),
        ],
    ),
    describe_function("print_oranges", &["oranges: u8"], None,
        "Use a match statement on age to print:
            if oranges is 1 print \"One orange\",
            if oranges is 2 print \"Two oranges\",
            if oranges is 3 print \"Three oranges\",
            Otherwise print the number of oranges followed by \" oranges\""
    ),
    None,
    None
);

make_test_info!(
    task4_8_info, "Matching on Structures",
    describe_type("Structure Matching",
        "Not only can you match on regular types but you can also match on entire Structures.
        Matching on a structure uses the braces to build the patterns. Match statements
        always start from top to bottom. If more than two branches would be valid it always
        takes the top most one.",
        (true, &[]),
        &[
            (
            "Example",
            "pub struct Date {
                pub year: u8,
                pub month: u8,
                pub day: u8,
            }

            let my_date: Date = Date{year: 1, month: 8, day: 24};

            match my_date {
                Date{year: 24, month: 12, day: 25 => {
                    print!(\"It's Christmas of the year '24\");
                },
                Date{year: _, month: 1, day: 1 => {
                    print!(\"It's New Years! I don't know which year though!\");
                },
                Date{year: the_year, month: 1, day: 1 => {
                    print!(\"It's New Years, of the year {}!\", the_year);
                },
                _ => {
                    print!(\"It Someday!!!!\");
                }
            }"
            ),
        ],
    ),
    "Add a method to the bottom of the  impl block for DriversLicense:\n        ".to_string()
    +&describe_function("examine", &["self: DriversLicense"], None,
        "Use a match statement on self to print:
            If the issued is 20, and the expires is 28 print:
                \"Hey that's my drivers license\",
            If the issued is 22, no matter the expires(wildcard):
                \"Drivers license issued during covid\",
            If the expires is 24:
                \"Expires on the 24 and was issued on \" followed by the issued year
            Otherwise:
                \"No comment\""
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
    "Add an impl block to the Color enum and add a method to it:\n        ".to_string()
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
    task5_5_info, "Impl TypeParameter",
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
    task5_6_info, "Impl TypeParameter",
    describe_type("TypeParam Method",
        "Another downside is that returning a value with a TypeParameter is much harder,
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


