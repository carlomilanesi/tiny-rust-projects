pub mod mock_db;
pub mod postgres_db;
pub mod sqlite_db;
use std::error::Error;

/*
trait Nullable<T> {
    fn value(&self) -> T;
}
*/

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct NationId(pub i64);

#[derive(Clone, Debug, PartialEq)]
pub struct NationName(pub String);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct TownId(pub i64);
/*
impl Nullable<i64> for TownId {
    fn value(&self) -> i64 {
        self.0
    }
}
*/

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct OptionalTownId(pub Option<i64>);
/*
impl Nullable<i64> for TownId {
    fn value(&self) -> i64 {
        self.0
    }
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TownName(pub String);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Latitude(pub f64);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Longitude(pub f64);

#[derive(Clone, Debug)]
pub struct Nation {
    pub name: NationName,
    pub capital_id: OptionalTownId,
}

#[derive(Clone, Debug)]
pub struct Town {
    pub name: TownName,
    pub lat: Latitude,
    pub long: Longitude,
    pub nation_id: NationId,
}

pub trait DbConnection {
    fn init(&mut self) -> Result<(), Box<dyn Error>>;

    fn insert_nation(&mut self, nation: Nation) -> Result<NationId, Box<dyn Error>>;

    fn insert_town(&mut self, town: Town) -> Result<TownId, Box<dyn Error>>;

    fn delete_nation(&mut self, id: NationId) -> Result<bool, Box<dyn Error>>;

    fn delete_town(&mut self, id: TownId) -> Result<bool, Box<dyn Error>>;

    fn update_nation(&mut self, id: NationId, nation: Nation) -> Result<bool, Box<dyn Error>>;

    fn update_town(&mut self, id: TownId, town: Town) -> Result<bool, Box<dyn Error>>;

    fn get_nation(&mut self, nation_id: NationId) -> Result<Option<Nation>, Box<dyn Error>>;

    fn get_town(&mut self, town_id: TownId) -> Result<Option<Town>, Box<dyn Error>>;

    fn filter_nations_by_name(
        &mut self,
        name: NationName,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(NationId, Nation), Box<dyn Error>>> + '_>,
        Box<dyn Error>,
    >;

    fn filter_towns_by_name(
        &mut self,
        name: TownName,
    ) -> Result<Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>, Box<dyn Error>>;

    /*
    fn filter_towns_by_lat_long(
        &mut self,
        min_lat: Latitude,
        max_lat: Latitude,
        min_long: Longitude,
        max_long: Longitude,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>,
        Box<dyn Error>,
    >;
    */
}
