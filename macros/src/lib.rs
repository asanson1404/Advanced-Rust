extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, LitInt};
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
