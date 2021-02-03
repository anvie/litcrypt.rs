#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Literal;
use quote::quote;
use std::env;

use std::sync::{Arc, Mutex};

mod xor;

lazy_static! {
    static ref MAGIC_SPELL: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

#[proc_macro]
pub fn use_litcrypt(tokens: TokenStream) -> TokenStream {
    let magic_spell = env::var("LITCRYPT_ENCRYPT_KEY").ok().or_else(|| {
        tokens
            .into_iter()
            .find(|a| match a {
                TokenTree::Literal(_) => true,
                _ => false,
            })
            .map(|a| match a {
                TokenTree::Literal(lit) => {
                    let s = lit.to_string();
                    String::from(&s[1..s.len() - 1])
                }
                _ => "default-secret-word".to_string(),
            })
    });

    {
        let mut m_spell = MAGIC_SPELL.lock().unwrap();
        *m_spell = magic_spell.clone();
    }
    // env::set_var("LITCRYPT_ENCRYPT_KEY", magic_spell.as_ref().map(|a| a.to_string()).unwrap());
    let encdec_func = quote! {
        pub mod litcrypt_internal {
            // This XOR code taken from https://github.com/zummenix/xor-rs
            /// Returns result of a XOR operation applied to a `source` byte sequence.
            ///
            /// `key` will be an infinitely repeating byte sequence.
            pub fn xor(source: &[u8], key: &[u8]) -> Vec<u8> {
                match key.len() {
                    0 => source.into(),
                    1 => xor_with_byte(source, key[0]),
                    _ => {
                        let key_iter = InfiniteByteIterator::new(key);
                        source.iter().zip(key_iter).map(|(&a, b)| a ^ b).collect()
                    }
                }
            }

            /// Returns result of a XOR operation applied to a `source` byte sequence.
            ///
            /// `byte` will be an infinitely repeating byte sequence.
            pub fn xor_with_byte(source: &[u8], byte: u8) -> Vec<u8> {
                source.iter().map(|&a| a ^ byte).collect()
            }

            struct InfiniteByteIterator<'a> {
                bytes: &'a [u8],
                index: usize,
            }

            impl<'a> InfiniteByteIterator<'a> {
                pub fn new(bytes: &'a [u8]) -> InfiniteByteIterator<'a> {
                    InfiniteByteIterator {
                        bytes: bytes,
                        index: 0,
                    }
                }
            }

            impl<'a> Iterator for InfiniteByteIterator<'a> {
                type Item = u8;
                fn next(&mut self) -> Option<u8> {
                    let byte = self.bytes[self.index];
                    self.index = next_index(self.index, self.bytes.len());
                    Some(byte)
                }
            }

            fn next_index(index: usize, count: usize) -> usize {
                if index + 1 < count {
                    index + 1
                } else {
                    0
                }
            }

            pub fn decrypt_bytes(encrypted: &[u8], encrypt_key: &[u8]) -> String {
                let decrypted = xor(&encrypted[..], &encrypt_key);
                String::from_utf8(decrypted).unwrap()
            }
        }
    };
    let result = if let Some(ekey) = magic_spell {
        let ekey = xor::xor(ekey.as_bytes(), b"l33t");
        let ekey = Literal::byte_string(&ekey);
        quote! {
            static LITCRYPT_ENCRYPT_KEY: &'static [u8] = #ekey;
            #encdec_func
        }
    } else {
        let ekey = xor::xor(b"default-secret-word", b"l33t");
        let ekey = Literal::byte_string(&ekey);
        quote! {
            static LITCRYPT_ENCRYPT_KEY: &'static [u8] = #ekey;
            #encdec_func
        }
    };
    result.into()
}

#[proc_macro]
pub fn lc(_item: TokenStream) -> TokenStream {
    let mut something = String::from("");
    for tok in _item {
        something = match tok {
            TokenTree::Literal(lit) => lit.to_string(),
            _ => "<unknown>".to_owned(),
        }
    }
    something = String::from(&something[1..something.len() - 1]);
    let ekey = {
        let m_spell = MAGIC_SPELL.lock().unwrap();
        (*m_spell).clone()
    };
    let encrypt_key = match ekey {
        Some(ref a) => a.as_bytes(),
        None => b"default-secret-word",
    };
    let encrypt_key = xor::xor(encrypt_key, b"l33t");
    let encrypted = xor::xor(&something.as_bytes(), &encrypt_key);
    let encrypted = Literal::byte_string(&encrypted);

    let result = quote! {
        crate::litcrypt_internal::decrypt_bytes(#encrypted, crate::LITCRYPT_ENCRYPT_KEY)
    };

    result.into()
}
