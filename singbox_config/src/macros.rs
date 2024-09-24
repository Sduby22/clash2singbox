#[macro_export]
macro_rules! gen_enum {
    ($name:ident, $($arm:ident),*) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(rename_all="lowercase")]
        #[serde(tag="type")]
        pub enum $name {
            $($arm($arm),)*
            #[serde(skip)]
            None,
        }

        impl $name {
            pub fn get_tag(&self) -> Option<&String> {
                match self {
                    $($name::$arm(e) => Some(&e.tag),)*
                    $name::None => None,
                }
            }
        }
    }
}
