
use std::cell::RefCell;
use std::rc::Rc;
// use std::fmt::{Display, Formatter, Debug};

use self::TokenType::*;
// use std::fmt;
use std::borrow::Borrow;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;
use std::fmt::Write;
use std::collections::HashSet;
use std::intrinsics::write_bytes;

include!("NFA.rs");
include!("State.rs");
include!("StateNext.rs");
include!("TokenType.rs");
include!("Token.rs");
include!("ChiReader.rs");
include!("ChiReaderState.rs");
include!("StateSet.rs");