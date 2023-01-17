extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, parse_quote, LitInt, ItemFn, Stmt};
use french_numbers::french_number;
use quote::{quote, quote_spanned};

#[proc_macro]
pub fn french(item: TokenStream) -> TokenStream {
  let litint = parse_macro_input!(item as LitInt);
  let pval = litint.base10_parse::<u32>();
  if pval.is_err() {
    let span = Span::call_site();
    let token = quote_spanned!{span => 
      compile_error!("This macro only works with integer literals (LitInt).");
    };
    return token.into();
  }
  let val = pval.unwrap();
  let fr_n = french_number(&val);
  let token = quote!(format!("{} ({})", #fr_n, #val));
  token.into()
}

#[proc_macro]
pub fn log_function(item: TokenStream) -> TokenStream {

  let mut func = parse_macro_input!(item as ItemFn);
  let f_name = &func.sig.ident;           // Name of the fonction

  // Statements to add to the field stmts of func.block
  let stmt1: Stmt = parse_quote!{
    println!("Entering function {}", stringify!(#f_name));
  };
  let stmt2: Stmt = parse_quote!{
    let start = ::std::time::Instant::now();
  };
  // Statement to add to the fiels stmts of block
  let stmt3: Stmt = parse_quote!{
    println!("Leaving function {} after {:#?}", stringify!(#f_name), start.elapsed());
  };

  // Insert the statements at their corresponding place
  // func.block.stmts is the vector with each statement of the function
  func.block.stmts.insert(0, stmt1);
  func.block.stmts.insert(1, stmt2);
  func.block.stmts.insert(func.block.stmts.len() - 1, stmt3); // Last statement for the function's return value

  quote!(#func).into()

}
