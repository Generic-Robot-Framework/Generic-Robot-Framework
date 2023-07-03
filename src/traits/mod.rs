use std::fmt::{Debug, Display};
use serde::{Serialize};

pub trait MessageTrait: Serialize + Debug + Display + Clone {

}