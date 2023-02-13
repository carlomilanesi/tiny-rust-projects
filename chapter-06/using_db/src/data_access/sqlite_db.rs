use crate::data_access::{
    DbConnection, Latitude, Longitude, Nation, NationId, NationName, OptionalTownId, Town, TownId,
    TownName,
};
use sqlite::{Connection, State, Statement, Value};
use std::error::Error;

pub struct SqliteConnection {
    conn: Connection,
}

impl SqliteConnection {
    pub fn create(connection_string: &str) -> Self {
        Self {
            conn: sqlite::open(connection_string).unwrap(),
        }
    }
}

trait SqliteParam {
    fn param(self, name: &str, value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

impl SqliteParam for Statement<'_> {
    fn param(mut self, name: &str, value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        self.bind((name, value))?;
        Ok(self)
    }
}

trait IsNewType {
    fn inner(&self) -> Value;
}

macro_rules! impl_IsNewType {
    (for $($t:ty),+) => {
        $(impl IsNewType for $t {
            fn inner(&self) -> Value {
                self.0.clone().into()
            }
        })*
    }
}

macro_rules! impl_IsOptionalNewType {
    (for $($t:ty),+) => {
        $(impl IsNewType for $t {
            fn inner(&self) -> Value {
                match self.0.clone() {
                    Some(some) => some.clone().into(),
                    None => Value::Null,
                }
            }
        })*
    }
}

impl_IsNewType!(for NationId, NationName, TownId, TownName, Latitude, Longitude);
impl_IsOptionalNewType!(for OptionalTownId);

trait ToValue {
    fn to_value(&self) -> Value;
}

impl<T> ToValue for T
where
    T: IsNewType,
{
    fn to_value(&self) -> Value {
        self.inner()
    }
}

impl DbConnection for SqliteConnection {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        self.conn.execute(
            "DROP TABLE IF EXISTS Nations;
            DROP TABLE IF EXISTS Towns;
            CREATE TABLE Nations (
                name TEXT NOT NULL,
                capital_id INTEGER NULL
            );
            CREATE TABLE Towns (
                name TEXT NOT NULL,
                lat FLOAT NOT NULL,
                long FLOAT NOT NULL,
                nation_id INTEGER NOT NULL
            );",
        )?;
        Ok(())
    }

    fn insert_nation(&mut self, nation: Nation) -> Result<NationId, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare(
                "INSERT INTO Nations (
                    name, capital_id
                ) VALUES (
                    :name, :capital_id
                ) RETURNING ROWID",
            )?
            .param(":name", nation.name.to_value())? // rende Value::String(nation.name.0)
            .param(":capital_id", nation.capital_id.to_value())?; // rende Value::Null o Value::Integer(nation.capital_id.0.unwrap())
        command.next()?;
        Ok(NationId(command.read(0)?))
    }

    fn insert_town(&mut self, town: Town) -> Result<TownId, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare(
                "INSERT INTO Towns (
                    name, lat, long, nation_id
                ) VALUES (
                    :name, :lat, :long, :nation_id
                ) RETURNING ROWID",
            )?
            .param(":name", town.name.to_value())?
            .param(":lat", town.lat.to_value())?
            .param(":long", town.long.to_value())?
            .param(":nation_id", town.nation_id.to_value())?;
        command.next()?;
        Ok(TownId(command.read(0)?))
    }

    fn delete_nation(&mut self, id: NationId) -> Result<bool, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare("DELETE FROM Nations WHERE ROWID = :id RETURNING ROWID")?
            .param(":id", id.to_value())?;
        command.next()?;
        Ok(command.read::<i64, _>(0)? == id.0)
    }

    fn delete_town(&mut self, id: TownId) -> Result<bool, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare("DELETE FROM Towns WHERE ROWID = :id RETURNING ROWID")?
            .param(":id", id.to_value())?;
        command.next()?;
        Ok(command.read::<i64, _>(0)? == id.0)
    }

    fn update_nation(&mut self, id: NationId, nation: Nation) -> Result<bool, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare(
                "UPDATE Nations SET
                    name = :name,
                    capital_id = :capital_id
                WHERE ROWID = :nation_id RETURNING ROWID",
            )?
            .param(":name", nation.name.to_value())?
            .param(":capital_id", nation.capital_id.to_value())?
            .param(":nation_id", id.to_value())?;
        command.next()?;
        Ok(command.read::<i64, _>(0)? == id.0)
    }

    fn update_town(&mut self, id: TownId, town: Town) -> Result<bool, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare(
                "UPDATE Towns SET
                    name = :name,
                    lat = :lat,
                    long = :long,
                    nation_id = :nation_id
                WHERE ROWID = :id RETURNING ROWID",
            )?
            .param(":name", town.name.to_value())?
            .param(":lat", town.lat.to_value())?
            .param(":long", town.long.to_value())?
            .param(":nation_id", town.nation_id.to_value())?
            .param(":id", id.to_value())?;
        command.next()?;
        Ok(command.read::<i64, _>(0)? == id.0)
    }

    fn get_nation(&mut self, id: NationId) -> Result<Option<Nation>, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare(
                "SELECT * FROM Nations
                WHERE ROWID = :id",
            )?
            .param(":id", id.to_value())?;
        match command.next() {
            Ok(State::Row) => Ok(Some(Nation {
                name: NationName(command.read("name")?),
                //capital_id: Some(TownId(command.read::<i64, _>("capital_id")?)),
                capital_id: OptionalTownId(command.read::<Option<i64>, _>("capital_id")?),
            })),
            Ok(State::Done) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn get_town(&mut self, id: TownId) -> Result<Option<Town>, Box<dyn Error>> {
        let mut command = self
            .conn
            .prepare(
                "SELECT * FROM Towns
                WHERE ROWID = :id",
            )?
            .param(":id", id.to_value())?;
        match command.next() {
            Ok(State::Row) => Ok(Some(Town {
                name: TownName(command.read("name")?),
                lat: Latitude(command.read("lat")?),
                long: Longitude(command.read("long")?),
                nation_id: NationId(command.read("nation_id")?),
            })),
            Ok(State::Done) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn filter_nations_by_name(
        &mut self,
        name: NationName,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(NationId, Nation), Box<dyn Error>>> + '_>,
        Box<dyn Error>,
    > {
        Ok(Box::new(
            self.conn
                .prepare(
                    "SELECT rowid, name, capital_id FROM Nations
                    WHERE name = :name",
                )?
                .param(":name", name.to_value())?
                .into_iter()
                .map(move |row| {
                    let row = row.unwrap();
                    let capital_id = OptionalTownId(match row.try_read("capital_id") {
                        Ok(value) => Some(value),
                        Err(_) => None,
                    });
                    Ok((
                        NationId(row.read("rowid")),
                        Nation {
                            name: NationName(row.read::<&str, _>("name").to_string()),
                            capital_id: capital_id,
                        },
                    ))
                }),
        ))
    }

    fn filter_towns_by_name(
        &mut self,
        name: TownName,
    ) -> Result<Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>, Box<dyn Error>>
    {
        Ok(Box::new(
            self.conn
                .prepare(
                    "SELECT rowid, name, lat, long, nation_id FROM Towns
                    WHERE name = :name",
                )?
                .param(":name", name.to_value())?
                .into_iter()
                .map(move |row| {
                    let row = row.unwrap();
                    Ok((
                        TownId(row.read("rowid")),
                        Town {
                            name: TownName(row.read::<&str, _>("name").to_string()),
                            lat: Latitude(row.read("lat")),
                            long: Longitude(row.read("long")),
                            nation_id: NationId(row.read("nation_id")),
                        },
                    ))
                }),
        ))
    }

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
    > {
        Ok(Box::new(
            self.conn
                .prepare(
                    "SELECT rowid, name, lat, long, nation_id FROM Towns
                    WHERE :min_lat <= lat AND lat <= :max_lat
                    AND :min_long <= long AND long <= :max_long",
                )?
                .param(":min_lat", min_lat.to_value())?
                .param(":max_lat", max_lat.to_value())?
                .param(":min_long", min_long.to_value())?
                .param(":max_long", max_long.to_value())?
                .into_iter()
                .map(move |row| {
                    let row = row.unwrap();
                    Ok((
                        TownId(row.read("rowid")),
                        Town {
                            name: TownName(row.read::<&str, _>("name").to_string()),
                            lat: Latitude(row.read("lat")),
                            long: Longitude(row.read("long")),
                            nation_id: NationId(row.read("nation_id")),
                        },
                    ))
                }),
        ))
    }
    */
}
