
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message<T: crate::traits::MessageTrait> {
    pub kind: String,
    pub topic: Option<String>,
    pub message: Option<T>,
}