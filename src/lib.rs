extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate chrono;
extern crate url;
use url::Url;

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

#[macro_use]
pub mod macros;

mod errors;
use errors::*;

use std::slice::Iter;
use std::str::FromStr;
use std::time::Duration;

mod datetime;
use datetime::DateTime;
pub use datetime::*;

mod identifier;
pub use identifier::*;

mod callbackurl;
pub use callbackurl::*;

mod originator;
pub use originator::*;

mod recipient;
pub use recipient::*;

mod recipients;
pub use recipients::*;

mod payload;
pub use payload::*;

mod message;
pub use message::*;
