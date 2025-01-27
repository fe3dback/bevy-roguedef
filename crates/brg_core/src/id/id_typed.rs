use std::marker::PhantomData;
use std::ops::Deref;

use serde::{Deserialize, Deserializer};

use crate::prelude::{Id, IdCategory};

// todo: replace all this code to derive macros
pub trait IdCategoryType {
    fn id_of(id: Id) -> IdOf<Self>
    where
        Self: Sized;
    fn category() -> IdCategory;
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct IdOf<C: IdCategoryType> {
    id:       Id,
    _phantom: PhantomData<C>,
}

impl<C: IdCategoryType> IdOf<C> {
    fn new(id: Id) -> Self {
        debug_assert_eq!(
            C::category(),
            id.category(),
            "try to cast Id '{}' into IdOf<{}>, but Id have another category: {}",
            id,
            C::category(),
            id.category(),
        );

        Self {
            id,
            _phantom: Default::default(),
        }
    }
}

impl<'de, T: IdCategoryType> Deserialize<'de> for IdOf<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Id::deserialize(deserializer).map(Self::new)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug, Deserialize)]
pub struct ICUnits;
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug, Deserialize)]
pub struct ICDoodads;
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug, Deserialize)]
pub struct ICDoodadsCategory;

impl IdCategoryType for ICUnits {
    fn id_of(id: Id) -> IdOf<Self>
    where
        Self: Sized,
    {
        IdOf::<ICUnits>::new(id)
    }

    fn category() -> IdCategory {
        IdCategory::Units
    }
}

impl IdCategoryType for ICDoodads {
    fn id_of(id: Id) -> IdOf<Self>
    where
        Self: Sized,
    {
        IdOf::<ICDoodads>::new(id)
    }

    fn category() -> IdCategory {
        IdCategory::Doodads
    }
}

impl IdCategoryType for ICDoodadsCategory {
    fn id_of(id: Id) -> IdOf<Self>
    where
        Self: Sized,
    {
        IdOf::<ICDoodadsCategory>::new(id)
    }

    fn category() -> IdCategory {
        IdCategory::DoodadsCategory
    }
}

impl<T: IdCategoryType> From<Id> for IdOf<T> {
    fn from(id: Id) -> Self {
        IdOf::<T>::new(id)
    }
}

impl<T: IdCategoryType> From<IdOf<T>> for Id {
    fn from(value: IdOf<T>) -> Self {
        value.id
    }
}

impl<T: IdCategoryType> Deref for IdOf<T> {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T: IdCategoryType + Sized> AsRef<Id> for IdOf<T> {
    #[inline]
    fn as_ref(&self) -> &Id {
        &self.id
    }
}

impl AsRef<Id> for Id {
    #[inline]
    fn as_ref(&self) -> &Id {
        &self
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::id::id_typed::{ICDoodads, IdOf};
    use crate::prelude::Id;

    #[test]
    fn test_id_cast() {
        let untyped_id = Id::new("dABCD").unwrap();
        let typed_id: IdOf<ICDoodads> = untyped_id.into();

        assert_eq!(untyped_id, typed_id.id);
        assert_eq!(typed_id._phantom, PhantomData::<ICDoodads>);

        let another_untyped_id: Id = typed_id.into();
        assert_eq!(untyped_id, another_untyped_id);
    }
}
