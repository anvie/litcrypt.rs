//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(slice_patterns)]

extern crate syntax;
extern crate syntax_pos;
extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_serialize as serialize;
extern crate aster;
extern crate xor;

use std::env;

use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax_pos::Span;
use rustc_plugin::Registry;
// use aster::invoke::Invoke;
use aster::path::PathBuilder;

use serialize::base64::{STANDARD, ToBase64};


fn expand_litcrypt(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument should be a single literal text, but got {} arguments", args.len()));
        return DummyResult::any(sp);
    }

    let text = match args[0] {
        TokenTree::Token(_, token::Literal(token::Lit::Str_(name), _)) => name.to_string(),
        _ => {
            cx.span_err(sp, "argument should be a single literal text");
            return DummyResult::any(sp);
        }
    };

    let xor_encrypt_key = match env::var("XOR_ENCRYPT_KEY"){
        Ok(a) => a,
        Err(_) => {
            cx.span_err(sp, "you needs to specify encrypt key via XOR_ENCRYPT_KEY environment variable.");
            return DummyResult::any(sp);
        }
    };

    let encrypted = xor::xor(text.as_bytes(), xor_encrypt_key.as_bytes());
    let _encrypted = encrypted.to_base64(STANDARD);

    println!("[litcrypt] encrypted: `{}` -> `{}`", text, _encrypted);

    let path = PathBuilder::new()
                .span(sp)
                .global()
                .ids(&["xor","decrypt"])
                .build();

    let builder = aster::AstBuilder::new().span(sp);

    let _lit = builder.expr().lit().str(_encrypted.as_str());
    let _key = builder.expr().lit().str(xor_encrypt_key.as_str());
    let _expr = builder.expr().call().build_path(path).with_arg(_lit).with_arg(_key).build();

    MacEager::expr(_expr)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("lc", expand_litcrypt);
}
