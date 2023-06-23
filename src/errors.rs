// Copyright 2023 Danmarks Tekniske Universitet
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! ASDB taxa error definitions

use std::error;
use std::fmt;
use std::io;
use std::num;

use regex;
use serde_json;

#[derive(Debug)]
pub enum ASDBTaxonError {
    Io(io::Error),
    InvalidTaxId(String),
    NotFound(i64),
    JSONParserError(serde_json::Error),
    IntParserError(num::ParseIntError),
    RegexError(regex::Error),
}

macro_rules! implement_custom_error_from {
    ($f: ty, $e: expr) => {
        impl From<$f> for ASDBTaxonError {
            fn from(f: $f) -> ASDBTaxonError {
                $e(f)
            }
        }
    };
}

implement_custom_error_from!(io::Error, ASDBTaxonError::Io);
implement_custom_error_from!(serde_json::Error, ASDBTaxonError::JSONParserError);
implement_custom_error_from!(num::ParseIntError, ASDBTaxonError::IntParserError);
implement_custom_error_from!(regex::Error, ASDBTaxonError::RegexError);

impl fmt::Display for ASDBTaxonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ASDBTaxonError::Io(ref err) => write!(f, "IO error: {}", err),
            ASDBTaxonError::InvalidTaxId(ref err) => write!(f, "Invalid TaxID: {}", err),
            ASDBTaxonError::NotFound(ref err) => write!(f, "TaxID not found: {}", err),
            ASDBTaxonError::JSONParserError(ref err) => write!(f, "Failed to parse JSON: {}", err),
            ASDBTaxonError::IntParserError(ref err) => write!(f, "Failed to parse int: {}", err),
            ASDBTaxonError::RegexError(ref err) => write!(f, "Failed to generate regex: {}", err),
        }
    }
}

impl error::Error for ASDBTaxonError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ASDBTaxonError::Io(ref err) => Some(err),
            ASDBTaxonError::JSONParserError(ref err) => Some(err),
            ASDBTaxonError::IntParserError(ref err) => Some(err),
            ASDBTaxonError::RegexError(ref err) => Some(err),
            ASDBTaxonError::NotFound(_) | ASDBTaxonError::InvalidTaxId(_) => None,
        }
    }
}