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

#![feature(core, io)]
#[macro_use] extern crate from_error_scope;

use from_error_scope::FromErrorScope;
use std::str::Utf8Error;
use std::io::CharsError;

struct ErrorScope;

impl FromErrorScope<Utf8Error, CharsError> for ErrorScope {
    fn from_error(&self, _: Utf8Error) -> CharsError {
       CharsError::NotUtf8
    }
}

fn from_utf8(v: &[u8]) -> Result<&str, CharsError> {
    Ok(trys!(ErrorScope, std::str::from_utf8(v)))
}

fn main() {
    let abc=vec![65,66,67];
    println!("{:?}", from_utf8(&abc).unwrap());
}
