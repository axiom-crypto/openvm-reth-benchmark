use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// A wrapper that serializes `Vec<u8>` using `serialize_bytes`/`deserialize_bytes`.
/// This leverages OpenVM's optimized byte array handling which uses `read_padded_bytes`
/// and `hint_buffer_u32!` for efficient word-aligned I/O.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizedBytes(pub Vec<u8>);

impl Serialize for OptimizedBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Use serialize_bytes which maps to OpenVM's optimized deserialize_bytes
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for OptimizedBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BytesVisitor;

        impl<'de> de::Visitor<'de> for BytesVisitor {
            type Value = OptimizedBytes;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OptimizedBytes(v.to_vec()))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OptimizedBytes(v))
            }

            // Fallback for deserializers that don't support bytes
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut bytes = Vec::new();
                while let Some(byte) = seq.next_element()? {
                    bytes.push(byte);
                }
                Ok(OptimizedBytes(bytes))
            }
        }

        // This will call OpenVM's optimized deserialize_bytes method
        deserializer.deserialize_bytes(BytesVisitor)
    }
}
