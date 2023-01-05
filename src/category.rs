use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Sponsor,
    SelfPromo,
    Interaction,
    Poi,
    Intro,
    Outro,
    Preview,
    MusicOfftopic,
    Filler,
    ExclusiveAccess,
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        match self {
            Category::Sponsor => "sponsor",
            Category::SelfPromo => "selfpromo",
            Category::Interaction => "interaction",
            Category::Poi => "poi_highlight",
            Category::Intro => "intro",
            Category::Outro => "outro",
            Category::Preview => "preview",
            Category::MusicOfftopic => "music_offtopic",
            Category::Filler => "filler",
            Category::ExclusiveAccess => "exclusive_access",
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<'de> serde::Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "sponsor" => Ok(Category::Sponsor),
            "selfpromo" => Ok(Category::SelfPromo),
            "interaction" => Ok(Category::Interaction),
            "poi_highlight" => Ok(Category::Poi),
            "intro" => Ok(Category::Intro),
            "outro" => Ok(Category::Outro),
            "preview" => Ok(Category::Preview),
            "music_offtopic" => Ok(Category::MusicOfftopic),
            "filler" => Ok(Category::Filler),
            "exclusive_access" => Ok(Category::ExclusiveAccess),
            _ => Err(serde::de::Error::custom("invalid category")),
        }
    }
}
