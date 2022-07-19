use serde::de::IgnoredAny;
use serde::{de, forward_to_deserialize_any};

pub fn deserialize_map_values<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::Deserialize<'de>,
{
    T::deserialize(MapValues { deserializer })
}

struct MapValues<D> {
    deserializer: D,
}

impl<'de, D> de::Deserializer<'de> for MapValues<D>
where
    D: de::Deserializer<'de>,
{
    type Error = D::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserializer.deserialize_map(MapValuesVisitor { visitor })
    }

    fn is_human_readable(&self) -> bool {
        self.deserializer.is_human_readable()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct MapValuesVisitor<V> {
    visitor: V,
}

impl<'de, V> de::Visitor<'de> for MapValuesVisitor<V>
where
    V: de::Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a map value")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        self.visitor.visit_seq(MapValuesSeq { map })
    }
}

struct MapValuesSeq<M> {
    map: M,
}

impl<'de, M> de::SeqAccess<'de> for MapValuesSeq<M>
where
    M: de::MapAccess<'de>,
{
    type Error = M::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.map.next_key::<IgnoredAny>()?.is_none() {
            return Ok(None);
        }

        let element = self.map.next_value_seed(seed)?;

        Ok(Some(element))
    }
}
