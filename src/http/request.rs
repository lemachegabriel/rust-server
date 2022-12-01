use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult, write};

pub struct Request {
  path: String,
  query_string: Option<String>,
  method: Method,
}

impl TryFrom<&[u8]> for Request {
  type Error = ParseErrror;

  fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
    unimplemented!()
  }
}

pub enum ParseErrror {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseErrror {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}

impl Display for ParseErrror {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseErrror {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.message())
  }
}


impl Error for ParseErrror {

}

