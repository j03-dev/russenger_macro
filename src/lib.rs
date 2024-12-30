extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// The `#[action]` proc macro is used to create a new action.
///
/// An action is a struct that implements the `Action` trait, which has two methods: `execute` and `path`.
///
/// The `execute` method is where you define how to handle the user input. It receives two parameters: `res` and `req`. `res` is a `Res` struct that you can use to send responses to the user, and `req` is a `Req` struct that contains the user input.
///
/// The `path` method returns the name of the action as a string.
///
/// # Examples
///
/// Creating a new action that sends a greeting message when the user input is "Hello":
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Main(res: russenger::prelude::Res, req: russenger::prelude::Req) {
///     let message: String = req.data.get_value();
///     if message == "Hello" {
///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
///     }
/// }
/// ```
///
/// This macro simplifies the process of creating a new action by automatically generating the struct and implementing the `Action` trait for it.
#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input = parse_macro_input!(item as ItemFn);

    // Extract function components
    let sig = input.sig;
    let vis = input.vis; // Function visibility
    let ident = sig.ident; // Function name
    let inputs = sig.inputs; // Function parameters
    let block = input.block;

    // Generate a new function with the proper async wrapping
    let expanded = quote! {
        #vis fn #ident(#inputs) -> std::pin::Pin<Box<dyn std::future::Future<Output = error::Result<()>>>> {
            Box::pin(async move #block)
        }
    };

    TokenStream::from(expanded)
}
