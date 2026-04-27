use serde::{Serialize, Serializer};

struct Duration {
    seconds: u64,
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We convert our internal data to a formatted string
        let s = format!("{}s", self.seconds);
        // Then we pass that string to the serializer
        serializer.serialize_str(&s)
    }
}

use serde::{de, Deserialize, Deserializer, Visitor};
use std::fmt;

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Duration;

            // Define what a "Duration" looks like in human-readable error messages
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string ending with 's'")
            }

            // Define how to handle a string input
            fn visit_str<E>(self, value: &str) -> Result<Duration, E>
            where
                E: de::Error,
            {
                if value.ends_with('s') {
                    let num_part = &value[..value.len() - 1];
                    let seconds = num_part.parse::<u64>().map_err(de::Error::custom)?;
                    Ok(Duration { seconds })
                } else {
                    Err(de::Error::custom("missing 's' suffix"))
                }
            }
        }

        // Initiate the handshake between the deserializer and our visitor
        deserializer.deserialize_str(DurationVisitor)
    }
}
