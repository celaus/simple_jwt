use std::marker::Sized;
use serde::{Serialize, Deserialize};
use serde_json;
use rustc_serialize::base64::{FromBase64, ToBase64, URL_SAFE};

use super::errors::{JWTError, Result};

/// trait that can be convert to/from base64 string
/// impl for serde::Serialize+serde::Deserialize are already defined
pub trait JWTStringConvertable : Sized{
    fn from_base64_str(string: &str) -> Result<Self>; 
    fn to_base64_str(&self) -> Result<String>;
}

impl<T> JWTStringConvertable for T
    where T: Serialize + Deserialize {
    fn from_base64_str(string: &str) -> Result<T> {
        let slice = try!(string.from_base64());
        serde_json::from_slice(&slice).map_err(JWTError::JsonError)
    }

    fn to_base64_str(&self) -> Result<String> {
        let b_string = try!(serde_json::to_vec(&self));
        Ok(b_string.to_base64(URL_SAFE))
    }
}

