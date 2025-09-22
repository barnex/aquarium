use crate::*;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

impl<T> Serialize for MemKeep<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.enumerate().collect::<Vec<_>>().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for MemKeep<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyWrapperVisitor<T> {
            marker: PhantomData<fn() -> MemKeep<T>>,
        }

        impl<'de, T> Visitor<'de> for MyWrapperVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = MemKeep<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a list of items for MyWrapper")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut m = MemKeep::new();

                while let Some((id, v)) = seq.next_element()? {
                    m._insert_at(id, v)
                }

                for (i, slot) in m.storage.iter_mut().enumerate().rev() {
                    if !slot.not_deleted.get() {
                        debug_assert!(slot.value.get_mut().is_none());
                        m.freelist.get_mut().push(i as u32);
                    }
                }

                Ok(m)
            }
        }

        deserializer.deserialize_seq(MyWrapperVisitor { marker: PhantomData })
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.index, self.generation).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (index, generation) = <(u32, u32)>::deserialize(deserializer)?;
        Ok(Id { index, generation })
    }
}
