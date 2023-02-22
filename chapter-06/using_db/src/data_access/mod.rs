pub mod mock_db;
pub mod persy_db;
pub mod postgres_db;
pub mod sqlite_db;

use persy::PersyId;
use std::error::Error;
extern crate rustc_serialize;

#[derive(PartialEq, Eq, Hash, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum NationId {
    Serial(i32),
    BigSerial(i64),
    PersyKey(String),
}
impl NationId {
    fn increment(&mut self) {
        match self {
            Self::Serial(n) => *self = Self::Serial(*n + 1),
            Self::BigSerial(n) => *self = Self::BigSerial(*n + 1),
            _ => {}
        }
    }
    fn to_i32(&self) -> i32 {
        match *self {
            NationId::BigSerial(n) => n as i32,
            NationId::Serial(n) => n,
            _ => 0,
        }
    }
    fn to_i64(&self) -> i64 {
        match *self {
            NationId::BigSerial(n) => n,
            NationId::Serial(n) => n as i64,
            _ => 0,
        }
    }
    /*
    pub fn to_string(&self) -> String {
        match self {
            NationId::BigSerial(n) => n.to_string(),
            NationId::Serial(n) => n.to_string(),
            NationId::PersyKey(key) => key.to_string(),
        }
    }
    fn to_persy_id(&self) -> PersyId {
        match *self {
            NationId::PersyKey(id) => id,
            _ => PersyId::from_str("0"),
        }
    }
    */
}

impl std::fmt::Display for NationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NationId::BigSerial(n) => write!(f, "{}", n),
            NationId::Serial(n) => write!(f, "{}", n),
            NationId::PersyKey(key) => write!(f, "{}", key),
        }
    }
}

impl std::fmt::Display for TownId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TownId::BigSerial(n) => write!(f, "{}", n),
            TownId::Serial(n) => write!(f, "{}", n),
            TownId::PersyKey(key) => write!(f, "{}", key),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NationName(pub String);

#[derive(PartialEq, Eq, Hash, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum TownId {
    Serial(i32),
    BigSerial(i64),
    PersyKey(String),
}
impl TownId {
    fn increment(&mut self) {
        match self {
            Self::Serial(n) => *self = Self::Serial(*n + 1),
            Self::BigSerial(n) => *self = Self::BigSerial(*n + 1),
            _ => {}
        }
    }
    fn to_i32(&self) -> i32 {
        match *self {
            TownId::BigSerial(n) => n as i32,
            TownId::Serial(n) => n,
            _ => 0,
        }
    }
    fn to_i64(&self) -> i64 {
        match *self {
            TownId::BigSerial(n) => n,
            TownId::Serial(n) => n as i64,
            _ => 0,
        }
    }
    /*
    fn to_persy_id(&self) -> PersyId {
        match *self {
            TownId::PersyKey(id) => id,
            _ => PersyId::from_str("0"),
        }
    }
    */
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OptionalTownId(pub Option<TownId>);

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TownName(pub String);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Latitude(pub f64);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct Longitude(pub f64);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Nation {
    pub name: NationName,
    pub capital_id: OptionalTownId,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Town {
    pub name: TownName,
    pub lat: Latitude,
    pub long: Longitude,
    pub nation_id: NationId,
}

pub trait DbConnection {
    /// If the specified database already exists, it opens it.
    /// Otherwise, it fails.
    fn open_existing(options: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    /// If the specified database already exists, it opens it and removes all its data.
    /// Otherwise, it fails.
    fn open_existing_truncated(options: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    /// If the specified database already exists, it fails.
    /// Otherwise, it creates and initializes it.
    fn create(options: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    /// If the specified database already exists, it opens it.
    /// Otherwise, it creates and initializes it.
    fn open_or_create(options: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    /// If the specified database already exists, it opens it and removes all its data.
    /// Otherwise, it creates and initializes it.
    fn open_truncated_or_create(options: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn insert_nation(&mut self, nation: &Nation) -> Result<NationId, Box<dyn Error>>;

    fn insert_town(&mut self, town: &Town) -> Result<TownId, Box<dyn Error>>;

    fn delete_nation(&mut self, id: &NationId) -> Result<bool, Box<dyn Error>>;

    fn delete_town(&mut self, id: &TownId) -> Result<bool, Box<dyn Error>>;

    fn update_nation(&mut self, id: &NationId, nation: &Nation) -> Result<bool, Box<dyn Error>>;

    fn update_town(&mut self, id: &TownId, town: &Town) -> Result<bool, Box<dyn Error>>;

    fn get_nation(&mut self, nation_id: &NationId) -> Result<Option<Nation>, Box<dyn Error>>;

    fn get_town(&mut self, town_id: &TownId) -> Result<Option<Town>, Box<dyn Error>>;

    fn filter_nations_by_name(
        &mut self,
        name: &NationName,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(NationId, Nation), Box<dyn Error>>> + '_>,
        Box<dyn Error>,
    >;

    fn filter_towns_by_name(
        &mut self,
        name: &TownName,
    ) -> Result<Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>, Box<dyn Error>>;

    fn filter_towns_by_lat_long(
        &mut self,
        min_lat: &Latitude,
        max_lat: &Latitude,
        min_long: &Longitude,
        max_long: &Longitude,
    ) -> Result<Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>, Box<dyn Error>>;
}
