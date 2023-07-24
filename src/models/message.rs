use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message<T: crate::traits::MessageTrait> {
    pub kind: String,
    pub topic: Option<String>,
    pub message: Option<T>,
    pub message_type: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoneType {

}

impl Display for NoneType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "None")
    }
}

impl Default for NoneType {
    fn default() -> Self {
        NoneType {

        }
    }
}

impl crate::traits::MessageTrait for NoneType {

}

pub fn get_message_type_as_str<T>() -> Option<String> {
    let content_type = std::any::type_name::<T>();
    let message_type;
    if content_type == std::any::type_name::<NoneType>() {
        message_type = None;
    }
    else {
        let proper_type_name = content_type.rsplit_once("::").unwrap().1.to_string();
        message_type = Some(proper_type_name);
    }

    return message_type;
}