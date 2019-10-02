//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(slice_patterns)]

// This link to rustc_driver is fix for
// Compiler error: "thread 'rustc' panicked at 'cannot access a scoped thread local variable without calling `set` first'".
// as discussed in https://github.com/rust-lang/rust/issues/62717
// extern crate rustc_interface;
extern crate rustc_driver;

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_pos;
extern crate xor;

use std::env;

use rustc_plugin::Registry;
use std::rc::Rc;
use syntax::ast::{Ident, LitKind};
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::parse::token::{self, TokenKind};
use syntax::tokenstream::TokenTree;
use syntax_pos::Span;

fn to_string(t: &token::Lit) -> String {
    let s = t.to_string();
    (&s[1..s.len() - 1]).to_string()
}

fn expand_litcrypt(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<dyn MacResult + 'static> {
    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!(
                "argument should be a single literal text, but got {} arguments",
                args.len()
            ),
        );
        return DummyResult::any(sp);
    }

    let text = match &args[0] {
        TokenTree::Token(t) => match t.kind {
            TokenKind::Literal(tt) => to_string(&tt),
            _ => {
                cx.span_err(sp, "invalid token kind");
                return DummyResult::any(sp);
            }
        },
        _ => {
            cx.span_err(sp, "argument should be a single literal text");
            return DummyResult::any(sp);
        }
    };

    let xor_encrypt_key = match env::var("XOR_ENCRYPT_KEY") {
        Ok(a) => a,
        Err(_) => {
            cx.span_err(
                sp,
                "you needs to specify encrypt key via XOR_ENCRYPT_KEY environment variable.",
            );
            return DummyResult::any(sp);
        }
    };

    let encrypted = xor::xor(text.as_bytes(), xor_encrypt_key.as_bytes());

    let encrypted_key: Vec<u8> = xor::xor(&xor_encrypt_key.as_bytes(), b"l33t");

    let rv = {
        cx.expr_call_global(
            sp,
            vec![Ident::from_str("xor"), Ident::from_str("decrypt_bytes")],
            vec![
                cx.expr_lit(sp, LitKind::ByteStr(Rc::new(encrypted))),
                cx.expr_lit(sp, LitKind::ByteStr(Rc::new(encrypted_key))),
            ],
        )
    };

    MacEager::expr(rv)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("lc", expand_litcrypt);
}
