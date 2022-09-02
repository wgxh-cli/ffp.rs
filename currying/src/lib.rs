use proc_macro::TokenStream;
use syn::{
  parse_macro_input,
  ItemFn,
};

pub(crate) mod fun;
pub(crate) mod closure;

use fun::gen_curried_fn;

#[proc_macro_attribute]
pub fn currying_fn(_: TokenStream, item: TokenStream) -> TokenStream {
  let origin = item.clone();
  let fun = parse_macro_input!(origin as ItemFn);
  let generated_fn = gen_curried_fn(fun);
  item.into_iter()
    .chain(generated_fn.into_iter())
    .collect()
}
