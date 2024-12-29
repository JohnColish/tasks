use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn make_test(items: TokenStream) -> TokenStream {
    let mut items = items.into_iter();
    let feat = &items.next().unwrap().to_string();
    items.next();
    let expr = if let TokenTree::Group(e) = items.next().unwrap() {e.stream()} else {todo!()};
    let expr = expr.to_string();

    let mac = format!("
        #[cfg(feature = \"{feat}\")]
        #[test]
        fn {feat}() {{
        	rules();
        	task_info::{feat}_info();
        	{expr}
        }}

        #[cfg(feature = \"{feat}_info\")]
        #[test]
        fn {feat}_info() {{
        	rules();
        	task_info::{feat}_info();
        }}

    ");
    mac.parse().unwrap()
}
