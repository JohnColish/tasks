use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn make_test(items: TokenStream) -> TokenStream {
    let mut items = items.into_iter();
    let feat = &items.next().unwrap().to_string();
    items.next();
    let expr = if let TokenTree::Group(e) = items.next().unwrap() {e.stream()} else {todo!()};
    let expr = expr.to_string();

    items.next();
    let next_feat = &items.next().unwrap().to_string();

    let mac = format!("
        #[cfg(feature = \"{feat}\")]
        #[allow(unused_must_use)]
        #[test]
        fn {feat}() {{
        	task_info::{feat}_info();
        	{expr}

            println!(\"
----------------------------------------------------------------------

CONGRATS YOU COMPLETED THE TASK!

Move on to the next task: cargo test --features={next_feat}_info

IGNORE EVERYTHING BELOW THIS

----------------------------------------------------------------------\");
        }}

        #[cfg(feature = \"{feat}_info\")]
        #[test]
        fn {feat}_info() {{
        	rules();
        	task_info::{feat}_info();
            println!(\"
----------------------------------------------------------------------

The above is your task information, this did not compile or run your code

To test this task run: cargo test --features={feat}

IGNORE EVERYTHING BELOW THIS

----------------------------------------------------------------------\");

        }}

    ");
    mac.parse().unwrap()
}
