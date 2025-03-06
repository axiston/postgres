//! Additional [`serde::Serializer`]s and [`serde::Deserializer`]s.

pub mod iso8601 {
    //! [`time::serde::iso8601`] for [`PrimitiveDateTime`].

    use serde::{Deserializer, Serializer};
    use time::PrimitiveDateTime;

    pub fn serialize<S>(datetime: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let datetime = datetime.assume_utc();
        time::serde::iso8601::serialize(&datetime, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let odt = time::serde::iso8601::deserialize(deserializer)?;
        Ok(PrimitiveDateTime::new(odt.date(), odt.time()))
    }

    pub mod option {
        //! [`time::serde::iso8601::option`] for [`PrimitiveDateTime`].

        use serde::{Deserializer, Serializer};
        use time::PrimitiveDateTime;

        pub fn serialize<S>(
            datetime: &Option<PrimitiveDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let datetime = datetime.map(|odt| odt.assume_utc());
            time::serde::iso8601::option::serialize(&datetime, serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let odt = time::serde::iso8601::option::deserialize(deserializer)?;
            Ok(odt.map(|odt| PrimitiveDateTime::new(odt.date(), odt.time())))
        }
    }
}
