use super::*;
make_game_info!(
    galaga_1_info, "This is your first Galaga task!",
    "Testing a task will either succeed or give you an error.
    If it gives you an error, you'll see the description of the task and
    the reason for the failure. You can see task 1 currently has an
    error shown at the bottom.

    left: 'Goodbye, World!'
    right: 'Hello, World!'", "",
    describe_goal("Nothing")
);

make_game_info!(
    galaga_2_info, "Main Function",
    describe_type("Main Function",
        "
        The main() function is the starting point of all Rust programs
        
        Rust will look for the main() function and run it first",
        (true, &[]),
        &[]
    ),
    describe_async_function("main", &[], None,
        "Create a println! statement that says 'My first main!'"
    ),
    describe_goal("You should see the text \"My first main!\"")
);


make_game_info!(
    galaga_3_info, "First Galaga Function",
    describe_type("Run Game Function",
        "
        We need to create a public async function where our game will run.",
        (true, &[]),
        &[(
            "The course hasn't covered async functions yet.
        We will talk more about this later. 
        Add 'async' between 'pub' and 'fn'",
        "pub aysnc fn make_sandwich() {}"
        )]
    ),
    describe_async_function("run_game", &[], None,
        "Create a println! statement that says 'Running game...'"
    )+"\n\nEXTRA:\n

    Delete the 'My first main!' println! in your main() function
    Call this run_game() function inside your main() function body",
    describe_goal("You should see the text \"Running game...\"")
);

make_game_info!(
    galaga_4_info, "Start Game Board",
    "\n
    Now, let's get a gameboard on the screen.
    We will do this by using println! statments",
    describe_addition("run_game",
        "Delete the 'Running game...' print statement.

    Create a print! statement that says '+'
    
    This will be the top left corner"
    ),
    describe_goal("You should see a single \"+\"")
);

make_game_info!(
    galaga_5_info, "Game Board Cont.",
    describe_type("The Top Border",
        "
        Now, let's create the top border of our gameboard",
        (true, &[]),
        &[]
    ),
    "Create a global const variable called COLUMNS and set it to 20;\n\n
    ".to_string()+&describe_addition("run_game",
        "Create a loop for _ in COLUMNS
        
    Inside the for loop, create a print! and print \"-\"
    
    After the for loop, create a println! statment for \"+\""
    ),
    describe_goal("You should see  \"+--------------------+\"")
);

make_game_info!(
    galaga_6_info, "Game Board Cont.",
    "The Walls",
    "Create a global const variable called ROWS and set it to 10;\n\n
    ".to_string()+&describe_addition("run_game",
        "Create a loop for row in ROWS
        
    Inside the for loop, create a print! statement for \"|\"
    
    Underneath the print statment, create another for loop for col in COLUMNS
    inside this for loop, print a space \" \"
    
    Inside your ROWS loop, underneath the COLUMNS loop,
    create a println! statement for \"|\"
    "
    ),
    describe_goal("You should see:

    +--------------------+
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |")
);

make_game_info!(
    galaga_7_info, "Game Board Cont.",
    "The Bottom Border",
    describe_addition("run_game",
        "At the bottom of this function,

    Create a print! statement for \"+\"

    Create a for loop for _ in COLUMNS
    
    Inside the for loop, create a print! and print \"-\"
    
    After the for loop, create a println! statment for \"+\"
    "
    ),
    describe_goal("You should see: 

    +--------------------+
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    +--------------------+")
);

make_game_info!(
    galaga_8_info, "The Player",
    "\n
    Now, let's get our player on the screen.
    We will use the ^ symbol to represent the player",
    describe_structure(
        "Coords",
        &["pub x: u8", "pub y: u8"],
        &[],
        &[],
    ).to_string()+"\n\n"
    +&describe_addition("run_game",
        "At the top of this function,

    Create a variable called player_positon
    Set player_position to an instance of Coords
    Where x is 10 and y is 8

    Inside the ROWS loop, inside the COLUMNS loop
    Create a variable called current_position 

    (This is the position of the space currently being printed)
    
    Set current_position to an instance of Coords
    Where x is the current row (row) and y is the current column (col)
    "
    ),
    describe_goal("Ignore all YELLOW warnings.
    
    You should see: 

    +--------------------+
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    +--------------------+")
);

make_game_info!(
    galaga_9_info, "The Player",
    "\n
    Now, let's get our player on the screen.
    We will use the ^ symbol to represent the player",
    describe_structure(
        "Coords",
        &["pub x: u8", "pub y: u8"],
        &[],
        &[],
    ).to_string()+"\n\n"
    +&describe_addition("run_game",
        "At the top of this function,

    Create a variable called player_positon
    Set player_position to an instance of Coords
    Where x is 10 and y is 8

    Inside the ROWS loop, inside the COLUMNS loop
    Create a variable called current_position 

    (This is the position of the space currently being printed)
    
    Set current_position to an instance of Coords
    Where x is the current row (row) and y is the current column (col)
    "
    ),
    describe_goal("Ignore all YELLOW warnings.
    
    You should see: 

    +--------------------+
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    |                    |
    +--------------------+")
);

// print!("           +");
// for _ in 0..COLUMNS {
//     print!("-");
// }
// println!("+           ");

// for row in 0..ROWS {
//     print!("           |");
//     for col in 0..COLUMNS {
//     let current_position = Cords(col, row);
//         // Create a variable called current_position
//         // Set current_position to be a Cord struct where
//         // the x is col
//         // the y is row

//         // Change this statememnt to compare player_position to current_position
//         if player_position == current_position {
//             print!("^");
//         } else {
//             print!(" ");
//         }
//     }
//     println!("|           ");
// }

// print!("           +");
// for _ in 0..COLUMNS {
//     print!("-");
// }
// println!("+           ");