#[macro_export]
macro_rules! register_message {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        use std::fs;
        use std::path::{Path, PathBuf};
        use std::fmt::{Debug, Display, Formatter};
        use schemars::{schema_for, JsonSchema};

        #[derive(serde::Serialize, serde::Deserialize, JsonSchema, Debug, Default, Clone)]
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

        fn main() {
            let temp_path_string = std::env::args().nth(1).expect("no path given");
            let temp_path = Path::new(temp_path_string.as_str());


            let struct_name = stringify!($name).to_owned();

            let json_schema = schema_for!($name);
            let json_schema_string = serde_json::to_string_pretty(&json_schema).unwrap();
            let json_schema_path = temp_path.join("schemas").join(struct_name.clone() + ".json");
            fs::write(json_schema_path, json_schema_string).unwrap();

            let json_default = $name::default();
            let json_default_string = serde_json::to_string(&json_default).unwrap();
            let json_default_path = temp_path.join("defaults").join(struct_name.clone() + ".json");
            fs::write(json_default_path, json_default_string).unwrap();

            print!("{struct_name}");
        }
    }
}