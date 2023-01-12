extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitInt};
use french_numbers::french_number;
use quote::{quote, quote_spanned};

#[proc_macro]
pub fn french(item: TokenStream) -> TokenStream {
  let litint = parse_macro_input!(item as LitInt);
  let val = litint.base10_parse::<u32>().unwrap();
  let fr_n = french_number(&val);
  let token = quote!(format!("{} ({})", #fr_n, #val));
  token.into()
}
