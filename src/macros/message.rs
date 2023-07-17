#[macro_export]
macro_rules! register_message {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        use std::fmt::{Debug, Display, Formatter};

        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        pub struct $name {
            $(pub $field: $t),*
        }

        impl generic_robot_framework::traits::MessageTrait for $name {

        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                $(
                    write!(f, "{}: {}\n",
                        stringify!($field).to_string(),
                        format!("{:#?}", self.$field)
                    )?;
                )*
                Ok(())
            }
        }
    }
}