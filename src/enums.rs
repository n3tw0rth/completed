use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Default, Debug, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum ConfigType {
    #[default]
    #[strum(to_string = "email")]
    Email,
    #[strum(to_string = "gchat")]
    Gchat,
}
