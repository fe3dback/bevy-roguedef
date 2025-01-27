mod base64;
mod category;
mod id;
mod id_typed;

pub mod prelude {
    pub use super::category::IdCategory;
    pub use super::id::{Id, IdError};
    pub use super::id_typed::*;
}
