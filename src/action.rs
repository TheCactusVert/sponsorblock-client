use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    Skip,
    Mute,
    Full,
    Poi,
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Action::Skip => "skip",
            Action::Mute => "mute",
            Action::Full => "full",
            Action::Poi => "poi",
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "skip" => Ok(Action::Skip),
            "mute" => Ok(Action::Mute),
            "full" => Ok(Action::Full),
            "poi" => Ok(Action::Poi),
            _ => Err(serde::de::Error::custom("invalid action")),
        }
    }
}
