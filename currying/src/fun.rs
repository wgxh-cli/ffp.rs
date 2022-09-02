use std::iter::once;
use syn::{
  parse_quote,
  PatIdent,
  ItemFn,
  Signature,
  ReturnType,
  FnArg,
  Block,
  Token,
  PatType,
  Type,
  punctuated::Punctuated,
  token::Brace,
  Expr,
  Ident,
  Stmt,
};
use quote::ToTokens;
use proc_macro2::Span;
use proc_macro::TokenStream;

pub fn gen_curried_fn(fun: ItemFn) -> TokenStream {
  let sig = gen_curried_fn_sign(&fun);
  let block = gen_curried_fn_block(&fun);
  ItemFn {
    sig,
    block: Box::new(block),
    attrs: vec![],
    ..fun
  }.to_token_stream().into()
}

pub fn gen_curried_fn_sign(fun: &ItemFn) -> Signature {
  let inputs = fun.sig.inputs.clone().into_iter();
  let ident = Ident::new(&format!("{}_curried", fun.sig.ident), Span::call_site());
  let return_type = gen_curried_fn_return_type(fun);
  Signature {
    inputs: inputs.take(1).collect(),
    ident,
    output: return_type,
    ..fun.sig.clone()
  }
}

pub fn gen_curried_fn_return_type(fun: &ItemFn) -> ReturnType {
  let output = match fun.sig.output.clone() {
    ReturnType::Type(_, ty) => ty,
    _ => panic!("")
  };
  let output: Type = parse_quote! {#output};
  let return_type = fun.sig.inputs
    .clone()
    .into_iter()
    .skip(1)
    .map(|a| {
      match a {
        FnArg::Typed(typ) => typ.ty,
        _ => panic!("Unsupported input type: Recievers")
      }
    })
    .rfold(output, |acc, x| {
      parse_quote! {
        Box<dyn Fn(#x) -> #acc>
      }
    });
  ReturnType::Type(Token![->](Span::call_site()), Box::new(return_type))
}

pub fn gen_curried_fn_block(fun: &ItemFn) -> Block {
  let fn_call: Block = *fun.block.clone();
  fun.sig.inputs
    .clone()
    .into_iter()
    .skip(1)
    .map(|arg| match arg {
      FnArg::Typed(ty) => ty,
      _ => panic!("Unsupported input type: Recievers")
    })
    .map(|ty| {
      (ty.pat, ty.ty)
    })
    .rfold(fn_call, |acc, (pat, ty)| {
      let a: Expr = parse_quote! {
        Box::new(move |#pat: #ty| { #acc })
      };
      Block {
        brace_token: Brace {
          span: Span::call_site(),
        },
        stmts: vec![Stmt::Expr(a)]
      }
    })
}
