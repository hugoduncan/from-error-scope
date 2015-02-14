// Copyright 2015 Hugo Duncan
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Enable conversion between non-local `Error` types.
//!
//! When using libraries, your code often needs to convert between
//! [`Error`](http://doc.rust-lang.org/std/error/trait.Error.html)
//! types which aren't part of your local crate. In this case you
//! can't implement
//! [`std::error::FromError`](http://doc.rust-lang.org/std/error/trait.FromError.html).
//!
//! This is also useful if you need context dependent translation of
//! error types.
//!
//!
//! # Examples
//!
//! # Simple
//!
//! ```rust
//! #[macro_use] extern crate from_error_scope;
//!
//! use from_error_scope::FromErrorScope;
//! use std::str::Utf8Error;
//! use std::io::CharsError;
//!
//! struct ErrorScope;
//!
//! impl FromErrorScope<Utf8Error, CharsError> for ErrorScope {
//!     fn from_error(&self, err: Utf8Error) -> CharsError {
//!        CharsError::NotUtf8
//!     }
//! }
//!
//! fn from_utf8(v: &[u8]) -> Result<&str, CharsError> {
//!     Ok(trys!(ErrorScope, std::str::from_utf8(v)))
//! }
//!
//! fn main () {
//!     let abc = vec![65,66,67];
//!     println!("{}", from_utf8(&abc).unwrap())
//! }
//!
//! ```
//!


/// A
/// [`FromError`](http://doc.rust-lang.org/std/error/trait.FromError.html)
/// like trait, that can be implemented on an explicit Scope type,
/// enabling conversion between non-local
/// [`Error`](http://doc.rust-lang.org/std/error/trait.Error.html)
/// types.
pub trait FromErrorScope<E,T> {
    fn from_error(&self, err: E) -> T;
}

/// A [`try!`](http://doc.rust-lang.org/std/macro.try!.html) like
/// macro that takes an additional type argument for a type that
/// implements [`FromErrorScope`](./trait.FromErrorScope.html).  You
/// may wish to wrap this macro for a specific error scope, to shorten
/// invocations.
#[macro_export]
macro_rules! trys {
    ($tr:expr, $expr:expr) => (match $expr {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            return ::std::result::Result::Err($tr.from_error(err))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::FromErrorScope;

    #[derive(Debug,PartialEq)]
    pub enum E1{
        Error(String)
    }

    #[derive(Debug,PartialEq)]
    pub enum E2{
        Error(String)
    }

    #[derive(Debug,PartialEq)]
    pub enum E3{
        Error(String)
    }

    pub struct MyScope;

    impl FromErrorScope<E1,E2> for MyScope {
        fn from_error(&self, err: E1) ->  E2 {
            match err {
                E1::Error(s) => E2::Error(s)
            }
        }
    }

    impl FromErrorScope<E1,E3> for MyScope {
        fn from_error(&self, err: E1) ->  E3 {
            match err {
                E1::Error(s) => E3::Error(s)
            }
        }
    }


    fn one() -> Result<u32,E1> {
        Ok(1)
    }

    fn err1() -> Result<u32,E1> {
        Err(E1::Error("e1".to_string()))
    }

    fn foo() -> Result<u32, E2> {
        assert_eq!(trys!(MyScope, one()), 1);
        Ok(trys!(MyScope, err1()))
    }

    #[test]
    fn test_from_error_scope() {
        let res = foo();
        println!("res {:?}",res);
        assert_eq!(res, Err(E2::Error("e1".to_string())))
    }
}
