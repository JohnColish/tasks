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
            "pub fn function_name(param_name: &str) -> u8 {
                //function body
            }"
        ), (
            "For a function that accepts a &str and u8 and returns nothing",
            "pub fn function_name(param_name: &str, second_param_name: u8) {
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
        "A &str is a Type that is easy to use, but cannot be easily stored.
        It's value is declared by using double quotes",
        (true, &[]),
        &[("For the string containing Hello, World!", "\"Hello, World!\"")],
    )+TYPE_SEP+
    &describe_type("Formatting String",
        "A formatting string is a &str used in by functions like print!.
        It can contain plain text, such as \"Hello, everyone\".
        It can also include various symbols to better format your print.
        For example you can type {} anywhere in the string and it will
        act as a place holder. You can then fill the placeholder by giving
        print! another paramater and it will fill it in.",
        (true, &[]),
        &[(
            "To print a dog breed out like this: \"Hello, Golden Retreiver!\"",
            "print!(\"Hello, {}!\", \"Golden Retreiver\")"
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
        "A parameter is like a placeholder that a function use
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
            "function_name(paramater: Type, second_parameter: Type)",
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
        rewriting it. To use a function, you call it by writing its name
        followed by parentheses (). If the function has parameters, you provide
        the values inside the parentheses when calling it.
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

        Additionally functions can accept more than one parameter, and when
        the function is called the values must be passed in the same order
        as when they're declared.
        ",
        (true, &[]),
        &[
            (
            "To print two variables, name and age, like this:
            'Hello, my name is Mitch and I am 38 years old",
            "print!(\"Hello, my name is {} and I am {} years old\", name, age);"
            ),
            (
            "Calling a function with multiple parameters",
            "pub fn a_and_b(a: u8, b: u8) {
                print!(\"this is a: {}\", a);
                print!(\"this is b: {}\", b);
            }

            a_and_b(2, 5); //Will print 25 but
            a_and_b(5, 2); //Will print 52"
            )
        ],
    ),
    describe_function("formal_hello", &["first_name: &str", "last_name: &str"], None,
        "print 'Hi, my name is ' followed by both of the parameters"
    ),
    None,
    None
);

make_test_info!(
    task1_7_info, "Paramater Reuse",
    describe_type("Reusing Parameters",
        "Paramaters and variables can be used multiple times if you
        'borrow' them, The borrow symbol is denoted by &. We will discuss
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
        A u8 cannot be negitive, commonly referred to as a byte.
        It is preferred to use this number type when possible.
        u8s can be printed in the same way strings can be.",
        (true, &[]),
        &[],
    ),
    describe_function("say_age", &["age: u8"], None,
        "print 'My age is ' followed by the parameter"
    ),
    None,
    None
);

make_test_info!(
    task2_2_info, "First Return Type",
    describe_type("Function Return Types",
        "Functions can return values that are a result of the logic inside.
        If a function returns a type then after calling a function it becomes
        the value it returned.",
        (false, &[
        "When defining (creating) a function, follow the parentheses () with an arrow ->,
            followed by the type of the return value (e.g., u8, &str, etc.).",
        "The return value can be specified with the return keyword, but this
            is only needed when you trying to return a vaule before the function
            is finished running. The proper way to return a value is to have it
            be on the last line of your function with out a ';'"
        ]),
        &[
            ("You can define the return type of a function like this",
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
    "",
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
    Some("Do not use print! or + in 'formal_greet' directly use 'add'"),
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

//  make_test_info!(
//      task4_1_info, "First Structure",
//      describe_type("Structures",
//          "Structures are exactly like Types but they can be constructed by anyone. All
//          Structures consist of 5 parts",
//          (true, &[
//              "pub struct: Is the keyword to declare a structure",
//              "Name: The name of a structure is always CammelCase, Meaning the first
//              letter of every word is capitalized and there are no spaces or underscores.",
//              "fields: Fields are the variables that the structure has inside of it.",
//              "Constructors: These are functions that may accept parameters but always
//              return the structure",
//              "Methods: These are functions that can be run on a structure, they can accept parameters, but they also have access to all the fields of a structure."
//          ]),
//          &[
//              (
//              "Format",
//              "pub struct Name {
//                  pub field_name: FieldType,
//                  pub field_name: FieldType,
//              }

//              //Constructors and Methods go in an implmentation shown later
//              "
//              ),
//              (
//              "Example",
//              "pub struct Date {
//                  pub year: u8,
//                  pub month: u8,
//                  pub day: u8,
//              }"
//              ),
//          ],
//      ),
//      describe_structure(
//          "DriversLicense",
//          &["issued: u8", "expires: u8"],
//          &[],
//          &[],
//      ),
//      None,
//      None
//  );

//  make_test_info!(
//      task4_2_info, "Brace Constructor",
//      describe_type("Brace '{}' constructor",
//          "The brace constructor is used to create an instance of your structure. Previously
//          you described the Type DriversLicense which is like any other type &str or u8 etc.
//          But now you are assigning it a value like \"hello, world\" to &str.",
//          (false, &[]),
//          &[
//              (
//              "Format",
//              "pub struct Name {
//                  pub field_name: FieldType,
//              }

//              let my_struct: Name = Name{field_name: value);"
//              ),
//              (
//              "Example",
//              "pub struct Date {
//                  pub year: u8,
//                  pub month: u8,
//                  pub day: u8,
//              }

//              let my_date = Date{year: 24, month: 12, day: 25};"
//              ),
//          ],
//      ),
//      describe_function(
//          "my_new_dl", &[], Some("DriversLicense"),
//          "Create a new drivers license with the brace constructor, issued is 4 and expires is 12"
//      ),
//      None,
//      None
//  );

//  make_test_info!(
//      task4_2_info, "Using Fields",
//      describe_type("Structure Fields",
//          "Structure fields are exactly like variables, contained inside of the structure.
//          To 'access' them you need to state the structure name followed by '.' and the field name.",
//          (true, &[]),
//          &[
//              (
//              "Format",
//              "pub struct Name {
//                  pub field_name: FieldType,
//              }

//              let my_struct: Name = Name{field_name: field_value};

//              print!(\"my field contains {}\", my_struct.field_name);"
//              ),
//              (
//              "Example",
//              "pub struct Date {
//                  pub year: u8,
//                  pub month: u8,
//                  pub day: u8,
//              }

//              let my_date = Date{year: 1, month: 12, day: 14};

//              print!(\"The year is {}\", my_date.year); //This year is 1"
//              ),
//          ],
//      ),
//      describe_function(
//          "print_drivers_license",
//          &["a_dl: DriversLicense"],
//          None,
//          "print the issued year and the expires year of a_dl with a '-' inbetween",
//      ),
//      None,
//      None
//  );

//  make_test_info!(
//      task4_3_info, "Constructors",
//      describe_type("Structure Constructors",
//          "Constructors are functions that are attached to the strucure Type,
//          These are used to build or construct the structure often limiting or providing
//          data for the structure.",
//          (false, &[
//              "Constructors always go at the top of the implementation(impl) block.",
//              "Constructors are almost always named 'new' and sometimes 'from'",
//              "Constructors must be called on the Structure TypeName using '::' to call it",
//              "'::' is refered to a TypePath used for accessing the inside of a Type"
//          ]),
//          &[
//              (
//              "Format",
//              "pub struct Name {
//                  pub field_name: FieldType,
//              }

//              impl Name {
//                  pub fn new(parameter_name: Type) -> Name {
//                      //Function body
//                  }
//              }

//              let my_struct: Name = Name::new(parameter_value);"
//              ),
//              (
//              "Example",
//              "pub struct Date {
//                  pub year: u8,
//                  pub month: u8,
//                  pub day: u8,
//              }

//              impl Date {
//                  pub fn new(my_year: u8) -> Date {
//                      Date{year: my_year, month: 12, day: 25}
//                  }
//              }

//              let christams_of_24 = Date::new(24);"
//              ),
//          ],
//      ),
//      "Add a construtor to the top of the impl block for DriversLicense:\n        ".to_string()
//      +&describe_function(
//          "new", &["issued: u8"], Some("DriversLicense"),
//          "Create a new DriversLicense where issued is given from the parameter
//          and expires is issued plus 8"
//      ).replace("\n", "\n    "),
//      None,
//      None
//  );

//  make_test_info!(
//      task4_4_info, "Methods",
//      describe_type("Structure Methods",
//          "Methods are functions that can be run on a structure. Methods go in
//          a seperate block called an implementation. A structure can have only one
//          implementation. All your methods and constructors must be placed inside.",
//          (true, &[]),
//          &[
//              (
//              "Format",
//              "pub struct Name {
//                  pub field_name: FieldType,
//              }

//              impl Name {
//                  pub fn my_method(my_name: &Name, parameter_name: Type) -> ReturnType {
//                      //Function body
//                  }
//              }

//              let my_struct: Name = Name{field_name: field_value};

//              my_struct.my_method(parameter_value);"
//              ),
//              (
//              "Example",
//              "pub struct Date {
//                  pub year: u8,
//                  pub month: u8,
//                  pub day: u8,
//              }

//              impl Date {
//                  pub fn print(a_date: &Date) {
//                      print!(\"{}/{}/{}\", a_date.month, a_date.day, a_date.year);
//                  }
//              }

//              let my_date = Date{year: 1, month: 12, day: 14};

//              my_date.print();"
//              ),
//          ],
//      ),
//      describe_structure(
//          "DriversLicense",
//          &["issued: u8", "expires: u8"],
//          &[],
//          &[describe_function(
//              "print", &["a_drivers_license: &DriversLicense"], None,
//              "print 'Issued on {}, Expires on {}' with the years inserted"
//          )]
//      ),
//      None,
//      None
//  );



//  make_test_info!(
//      task4_4_info, "Constructors",
//      describe_type("Structure Constructors",
//          "Constructors are functions that are attached to the strucure Type,
//          These are used to build or construct the structure often limiting or providing
//          data for the structure.",
//          (false, &[
//              "Constructors always go at the top of the impl block.",
//              "Constructors are almost always named 'new' and sometimes 'from'",
//              "Constructors must be called on the Structure TypeName using '::' to call it",
//              "'::' is refered to a TypePath used for accessing the inside of a Type"
//          ]),
//          &[
//              (
//              "Format",
//              "pub struct Name {
//                  pub field_name: FieldType,
//              }

//              impl Name {
//                  pub fn new(parameter_name: Type) -> Name {
//                      //Function body
//                  }
//              }

//              let my_struct: Name = Name::new(parameter_value);"
//              ),
//              (
//              "Example",
//              "pub struct Date {
//                  pub year: u8,
//                  pub month: u8,
//                  pub day: u8,
//              }

//              impl Date {
//                  pub fn new(year: u8, month: u8, day: u8) -> Date {
//                      Date{year: year, month: month, day: day}
//                  }
//              }

//              let my_date = Date::new(1, 12, 14);"
//              ),
//          ],
//      ),
//      "Add a construtor to the top of the impl block for DriversLicense:\n        ".to_string()
//      +&describe_function(
//          "new", &["issued: u8"], Some("DriversLicense"),
//          "Create a new DriversLicense where issued is given from the parameter
//          and expires is issued plus 8"
//      ).replace("\n", "\n    "),
//      None,
//      None
//  );

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

//  make_test_info!(
//      task4_7_info, "Matching on Types",
//      describe_type("More Matching",
//          "In section 3 we matched on the 'bool' type which only had to values 'true' or 'false'.
//          But the match statement can be used on any Type. But if you were to match on the u8
//          that would be 128 branches of the match statement, because a u8 can be 0 to 127.
//          There are two solutions:",
//          (true, &[
//              "WildCard(_): The wild card allows you to ignore the value and execute some code
//              as a fall back",
//              "Catching it in a Varibale: You can specify a variable name and no matter what it is
//              it will be stored there and you can use it in a seperate way."
//          ]),
//          &[
//              (
//              "Using WildCard(_)",
//              "let my_age: u8 = 18;

//              match my_age {
//                  3 => {
//                      print!(\"I am 3!\");
//                  },
//                  18 => {
//                      print!(\"18 years old!\");
//                  },
//                  _ => {
//                      print!(\"I don't know how old I am but I am not 3 or 18 years old.\");
//                  }
//              }"
//              ),
//              (
//              "Using a Variable",
//              "let my_age: u8 = 18;

//              match my_age {
//                  3 => {
//                      print!(\"I am 3!\");
//                  },
//                  18 => {
//                      print!(\"18 years old!\");
//                  },
//                  age => {
//                      print!(\"I don't know how old I am but here is my age: {}\", age);
//                  }
//              }"
//              ),
//          ],
//      ),
//      describe_function("print_age", &["age: u8"], None,
//          "Use a match statement on age to print:
//              If the age is 10 print \"You are ten years old!\",
//              Otherwise print \"I don't know how old you are but you are alive!\""
//      ),
//      None,
//      None
//  );

//  make_test_info!(
//      task4_8_info, "Matching on Structures",
//      describe_type("Advanced Matching",
//          "Not only can you match on regular types but you can also match on Structures.
//          WildCard(_) and Variable Matching are possible too!",
//          (true, &[]),
//          &[
//              (
//              "Example",
//              "pub struct Date {
//                  pub year: u8,
//                  pub month: u8,
//                  pub day: u8,
//              }

//              let my_date: Date = Date{year: 1, month: 8, day: 24};

//              match my_date {
//                  Date{year: 24, month: 12, day: 25 => {
//                      print!(\"It's Christmas of the year '24\");
//                  },
//                  Date{year: _, month: 1, day: 1 => {
//                      print!(\"It's New Years! I don't know which year though!\");
//                  },
//                  Date{year: the_year, month: 1, day: 1 => {
//                      print!(\"It's New Years, of the year {}!\", the_year);
//                  },
//                  _ => {
//                      print!(\"It Someday!!!!\");
//                  }
//              }"
//              ),
//          ],
//      ),
//      describe_function("examine_tombstone", &["a_tombstone: &Tombstone"], None,
//          "Use a match statement on a_tombstone to print:
//              If the birth_year is 12 and the death_year 92 print
//                  \"He was born in the year '12 and lived 80 years!\",
//              If death_year is 30 print
//                  \"I don't know when they were born but they died in the year '30\",
//              Otherwise print \"I don't know when they were born but they died in the year {}\" insert the years"
//      ),
//      None,
//      None
//  );

//  //  make_test_info!(
//  //      task5_1_info, "Enumerators",
//  //      describe_type("Enumerator",
//  //          "An enumerator is a Type that can be one of a set of sub-structs.",
//  //          (false, &[
//  //              "An enumerator can be matched on, to figure out what sub-structure it is",
//  //              "Enumerators have an impl block that can contain methods and constructors",
//  //              "Enumerator sub-structures cannot have their own methods or constructors",
//  //              "Enumerator sub-structures are not valid types on their own",
//  //              "To declare an Enum you use the TypePath Seperator '::' to choose the variant"
//  //          ]),
//  //          &[
//  //              (
//  //              "Example",
//  //              "pub enum Color {
//  //                  Blue,
//  //                  Green,
//  //                  Red
//  //              }

//  //              let my_blue: Color = Color::Blue;
//  //              let my_red: Color = Color::Red;"
//  //              ),
//  //          ],
//  //      ),
//  //      describe_function("examine_tombstone", &["a_tombstone: &Tombstone"], None,
//  //          "Use a match statement on a_tombstone to print:
//  //              If the birth_year is 12 and the death_year 92 print
//  //                  \"He was born in the year '12 and lived 80 years!\",
//  //              If death_year is 30 print
//  //                  \"I don't know when they were born but they died in the year '30\",
//  //              Otherwise print \"I don't know when they were born but they died in the year {}\" insert the years"
//  //      ),
//  //      None,
//  //      None
//  //  );
