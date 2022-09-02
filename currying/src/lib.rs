use std::iter::once;
use syn::{
  parse_macro_input,
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

#[proc_macro_attribute]
pub fn currying_fn(_: TokenStream, item: TokenStream) -> TokenStream {
  let _item = item.clone();
  let fun = parse_macro_input!(_item as ItemFn);
  let generated_fn = gen_curried_fn(fun);
  item.into_iter()
    .chain(generated_fn.into_iter())
    .collect()
}

fn gen_curried_fn(fun: ItemFn) -> TokenStream {
  let sig = gen_curried_fn_sign(&fun);
  let block = gen_curried_fn_block(&fun);
  ItemFn {
    sig,
    block: Box::new(block),
    attrs: vec![],
    ..fun
  }.to_token_stream().into()
}

fn gen_curried_fn_sign(fun: &ItemFn) -> Signature {
  let mut inputs = fun.sig.inputs.clone().into_iter();
  let ident = Ident::new(&format!("{}_curried", fun.sig.ident), Span::call_site());
  let input_type = match inputs.next().unwrap() {
    FnArg::Typed(ty) => ty,
    _ => panic!("")
  };
  let return_type = gen_curried_fn_return_type(fun);
  Signature {
    inputs: Punctuated::from_iter(
      once(FnArg::Typed(PatType {
        pat: Box::new(PatIdent {
          attrs: vec![],
          by_ref: None,
          mutability: None,
          subpat: None,
          ident: parse_quote! {a0}
        }.into()),
        ..input_type
      }))
    ),
    ident,
    output: return_type,
    ..fun.sig.clone()
  }
}

fn gen_curried_fn_return_type(fun: &ItemFn) -> ReturnType {
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

fn gen_curried_fn_block(fun: &ItemFn) -> Block {
  let ident = &fun.sig.ident;
  let len = fun.sig.inputs.len();
  let inputs_ident = (0..len)
    .map(|i| format!("a{}", i))
    .map(|ident| {
      Ident::new(&ident[..], Span::call_site())
    });
  let params: Punctuated<Ident, Token![,]> = Punctuated::from_iter(inputs_ident.clone());
  let fn_call: Expr = parse_quote! {#ident(#params)};
  let body = fun.sig.inputs
    .clone()
    .into_iter()
    .skip(1)
    .map(|arg| match arg {
      FnArg::Typed(ty) => ty.ty,
      _ => panic!("Unsupported input type: Recievers")
    })
    .zip(inputs_ident.skip(1))
    .rfold(fn_call, |acc, (ty, arg)| {
      parse_quote! {
        Box::new(move |#arg: #ty| { #acc })
      }
    });
  Block {
    brace_token: Brace {
      span: Span::call_site(),
    },
    stmts: vec![Stmt::Expr(body)],
  }
}
