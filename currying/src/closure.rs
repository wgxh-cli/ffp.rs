use proc_macro::TokenStream;
use syn::{
  ExprClosure,
  Expr,
  ReturnType
};

fn gen_curried_closure(origin: ExprClosure) -> TokenStream {
  todo!()
}

fn gen_curried_closure_sig(origin: &ExprClosure) -> ExprClosure {
  ExprClosure {
    ..origin.clone()
  }
}
