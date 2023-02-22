/*
// Look at installed docker images by typing:
sudo docker images

// Look at existing docker containers by typing:
sudo docker ps -a

// Download and run a Postgres docker image by typing:
sudo docker run --name pg-world -e POSTGRES_PASSWORD=myp -p 5432:5432 -d postgres:latest

// To stop a running container
sudo docker stop {container-id}

// To destroy a stopped docker container
sudo docker rm {container-id}

// Running a docker shell by typing:
psql -U postgres -h localhost
// The password will be asked interactively.
*/

use crate::data_access::{
    DbConnection, Latitude, Longitude, Nation, NationId, NationName, OptionalTownId, Town, TownId,
    TownName,
};
use postgres::{fallible_iterator::FallibleIterator, types::ToSql, Client, NoTls, Row, RowIter};

/*
fn value_or_null<'a, T: ToSql + Sync + 'a>(n: Option<T>) -> Box<dyn ToSql + Sync + 'a> {
    if let Some(id) = n {
        Box::new(id) as Box<dyn ToSql + Sync + '_>
    } else {
        Box::new(None::<&T>) as Box<dyn ToSql + Sync + '_>
    }
}
*/
fn value_or_null(optional_town_id: &OptionalTownId) -> Box<dyn ToSql + Sync> {
    if let Some(town_id) = optional_town_id.0.clone() {
        match town_id {
            TownId::Serial(id) => Box::new(id) as Box<dyn ToSql + Sync + '_>,
            TownId::BigSerial(id) => Box::new(id) as Box<dyn ToSql + Sync + '_>,
            TownId::PersyKey(id) => Box::new(0) as Box<dyn ToSql + Sync + '_>,
        }
    } else {
        Box::new(None::<&i64>) as Box<dyn ToSql + Sync + '_>
    }
}

pub struct PostgresConnection {
    conn: Client,
}

/*
fn aa() {
    use std::io::ErrorKind::Other;
    use std::{error, io};
    type AbstractError = Box<dyn error::Error>;

    // // // Errors
    // // // ------------------------------

    // // I/O errors
    // // --------------------

    // An I/O error: allowed.
    let _: io::Error = io::Error::new(Other, "ouch!");

    // A reference to an I/O error: allowed.
    let _: &io::Error = &io::Error::new(Other, "ouch!");

    // A boxed I/O error: allowed.
    let _: Box<io::Error> = Box::new(io::Error::new(Other, "ouch!"));

    // // Trait errors
    // // --------------------

    // A trait error: allowed through casting of reference.
    let _: dyn error::Error = *(&io::Error::new(Other, "ouch!") as &dyn error::Error);

    // A reference to a trait error: allowed.
    let _: &dyn error::Error = &io::Error::new(Other, "ouch!");

    // Boxed trait error: allowed.
    let _: Box<dyn error::Error> = Box::new(io::Error::new(Other, "ouch!"));

    // // // Slice iterators over errors
    // // // ------------------------------
    use std::slice::Iter;

    // // Slice iterators over I/O errors
    // // --------------------

    // A slice iterator over I/O errors: allowed.
    let _: Iter<io::Error> = [io::Error::new(Other, "ouch!")].iter();

    // A slice iterator over references to I/O errors: allowed.
    let _: Iter<&io::Error> = [&io::Error::new(Other, "ouch!")].iter();

    // A slice iterator over boxed I/O errors: allowed.
    let _: Iter<Box<io::Error>> = [Box::new(io::Error::new(Other, "ouch!"))].iter();

    // // Slice iterators over trait errors
    // // --------------------

    // A slice iterator over trait errors: NOT allowed.
    let _: Iter<dyn error::Error>;

    // A slice iterator over references to trait errors: allowed through casting of reference.
    let _: Iter<&dyn error::Error> = [&io::Error::new(Other, "ouch!") as &dyn error::Error].iter();

    // A slice iterator over boxed trait errors: allowed though casting of Box.
    let _: Iter<Box<dyn error::Error>> =
        [Box::new(io::Error::new(Other, "ouch!")) as Box<dyn error::Error>].iter();

    // // References to slice iterators over I/O errors
    // // --------------------

    // A reference to a slice iterator over I/O errors: allowed.
    let _: &Iter<io::Error> = &[io::Error::new(Other, "ouch!")].iter();

    // A reference to a slice iterator over references to I/O errors: allowed.
    let _: &Iter<&io::Error> = &[&io::Error::new(Other, "ouch!")].iter();

    // A reference to a slice iterator over boxed I/O errors: allowed.
    let _: &Iter<Box<io::Error>> = &[Box::new(io::Error::new(Other, "ouch!"))].iter();

    // // References to slice iterators over trait errors
    // // --------------------

    // A reference to a slice iterator over trait errors: NOT allowed.
    let _: &Iter<dyn error::Error>;

    // A reference to a slice iterator over references to trait errors: allowed through casting of reference.
    let _: &Iter<&dyn error::Error> =
        &[&io::Error::new(Other, "ouch!") as &dyn error::Error].iter();

    // A reference to a slice iterator over boxed trait errors: allowed through casting of box.
    let _: &Iter<Box<dyn error::Error>> =
        &[Box::new(io::Error::new(Other, "ouch!")) as Box<dyn error::Error>].iter();

    // // Boxed slice iterators over I/O errors
    // // --------------------

    // A boxed slice iterator over I/O errors: allowed.
    let _: Box<Iter<io::Error>> = Box::new([io::Error::new(Other, "ouch!")].iter());

    // A boxed slice iterator over references to I/O errors: allowed.
    let _: Box<Iter<&io::Error>> = Box::new([&io::Error::new(Other, "ouch!")].iter());

    // A boxed slice iterator over boxed I/O errors: allowed.
    let _: Box<Iter<Box<io::Error>>> = Box::new([Box::new(io::Error::new(Other, "ouch!"))].iter());

    // // Boxed slice iterators over trait errors
    // // --------------------

    // A boxed slice iterator over trait errors: NOT allowed.
    let _: Box<Iter<dyn error::Error>>;

    // A boxed slice iterator over references to trait errors: allowed through casting of reference.
    let _: Box<Iter<&dyn error::Error>> =
        Box::new([&io::Error::new(Other, "ouch!") as &dyn error::Error].iter());

    // A boxed slice iterator over boxed trait errors: allowed through casting of box.
    let _: Box<Iter<Box<dyn error::Error>>> =
        Box::new([Box::new(io::Error::new(Other, "ouch!")) as Box<dyn error::Error>].iter());

    // // // Trait iterators over errors
    // // // ------------------------------

    // // Trait iterators over I/O errors
    // // --------------------

    // A trait iterator over I/O errors: NOT allowed.
    let _: Iterator<Item = io::Error>;

    // A trait iterator over references to I/O errors: NOT allowed.
    let _: Iterator<Item = &io::Error>;

    // A trait iterator over boxed I/O errors: NOT allowed.
    let _: Iterator<Item = Box<io::Error>>;

    // // Trait iterators over trait errors
    // // --------------------

    // A trait iterator over trait errors: NOT allowed.
    let _: Iterator<Item = dyn error::Error>;

    // A trait iterator over references to trait errors: NOT allowed.
    let _: Iterator<Item = &dyn error::Error>;

    // A trait iterator over boxed trait errors: NOT allowed.
    let _: Iterator<Item = Box<dyn error::Error>>;

    // // References to trait iterator over I/O errors
    // // --------------------

    // A reference to a trait iterator over I/O errors: allowed.
    let _: &dyn Iterator<Item = io::Error> = &[io::Error::new(Other, "ouch!")].into_iter();

    // A reference to a trait iterator over references to I/O errors: allowed.
    let _: &dyn Iterator<Item = &io::Error> = &[&io::Error::new(Other, "ouch!")].into_iter();

    // A reference to a trait iterator over boxed I/O errors: allowed.
    let _: &dyn Iterator<Item = Box<io::Error>> =
        &[Box::new(io::Error::new(Other, "ouch!"))].into_iter();

    // // References to trait iterators over trait errors
    // // --------------------

    // A reference to a trait iterator over trait errors: ?????????
    let _: &dyn Iterator<Item = dyn error::Error>; //=&[*(&io::Error::new(Other, "ouch!") as &dyn error::Error)].into_iter();
    let _: &dyn Iterator<Item = dyn error::Error>; //=&[*(&io::Error::new(Other, "ouch!") as &dyn error::Error)].iter();

    // A reference to a trait iterator over references to trait errors: allowed through casting of reference.
    let _: &dyn Iterator<Item = &dyn error::Error> =
        &[&io::Error::new(Other, "ouch!") as &dyn error::Error].into_iter();

    // A reference to a trait iterator over boxed trait errors: allowed through casting of box.
    let _: &dyn Iterator<Item = Box<dyn error::Error>> =
        &[Box::new(io::Error::new(Other, "ouch!")) as Box<dyn error::Error>].into_iter();

    // // Boxed trait iterators over I/O errors
    // // --------------------

    // A boxed trait iterator over I/O errors: allowed.
    let _: Box<dyn Iterator<Item = io::Error>> =
        Box::new([io::Error::new(Other, "ouch!")].into_iter());

    // A boxed trait iterator over references to I/O errors: allowed.
    let _: Box<dyn Iterator<Item = &io::Error>> =
        Box::new([&io::Error::new(Other, "ouch!")].into_iter());

    // A boxed trait iterator over boxed I/O errors: allowed.
    let _: Box<dyn Iterator<Item = Box<io::Error>>> =
        Box::new([Box::new(io::Error::new(Other, "ouch!"))].into_iter());

    // // Boxed trait iterators over trait errors
    // // --------------------

    // A boxed trait iterator over trait errors: NOT allowed value.
    let _: Box<dyn Iterator<Item = dyn error::Error>> =
        Box::new([(&io::Error::new(Other, "ouch!") as &dyn error::Error)].iter());

    // A boxed trait iterator over references to trait errors: allowed through casting of reference.
    let _: Box<dyn Iterator<Item = &dyn error::Error>> =
        Box::new([&io::Error::new(Other, "ouch!") as &dyn error::Error].into_iter());

    // A boxed trait iterator over boxed trait errors: allowed through casting of box.
    let _: Box<dyn Iterator<Item = Box<dyn error::Error>>> =
        Box::new([Box::new(io::Error::new(Other, "ouch!")) as Box<dyn error::Error>].into_iter());
}
*/

/*
impl PostgresConnection {
    pub fn open_with_string(connection_string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            conn: Client::connect(connection_string, NoTls)?,
        })
    }

    pub fn open(options: &PostgresCreationArgs) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            conn: Client::connect(
                &format!(
                    "postgres://{}{}{}@{}{}{}{}{}",
                    options.username,
                    if options.password.is_empty() { "" } else { ":" },
                    options.password,
                    options.host,
                    if options.port.is_empty() { "" } else { ":" },
                    options.port,
                    if options.database.is_empty() { "" } else { "/" },
                    options.database,
                ),
                NoTls,
            )?,
        })
    }
}
*/

/*
trait PostgresParam {
    fn param(self, name: &str, value: Value) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

impl PostgresParam for Statement<'_> {
    fn param(mut self, name: &str, value: Value) -> Result<Self, Box<dyn std::error::Error>>
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
impl_IsNewType!(for NationId, NationName, TownId, TownName, Latitude, Longitude);

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

impl<T> ToValue for Option<T>
where
    T: IsNewType,
{
    fn to_value(&self) -> Value {
        match self {
            Some(val) => val.inner().into(),
            None => Value::Null,
        }
    }
}
*/

impl PostgresConnection {
    fn create_connection(options: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            conn: Client::connect(options, NoTls)?,
        })
    }

    fn create_tables(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.conn.batch_execute(
            "CREATE TABLE Nations (
                rowid BIGSERIAL PRIMARY KEY,
                name VARCHAR(40) NOT NULL,
                capital_id BIGINT NULL
            );
            CREATE TABLE Towns (
                rowid BIGSERIAL PRIMARY KEY,
                name VARCHAR(40) NOT NULL,
                lat DOUBLE PRECISION NOT NULL,
                long DOUBLE PRECISION NOT NULL,
                nation_id BIGINT NOT NULL
            );",
        )?;
        Ok(())
    }

    fn truncate_tables(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self
            .conn
            .batch_execute("TRUNCATE Nations, Towns RESTART IDENTITY;")?;
        Ok(())
    }

    fn tables_exist(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self
            .conn
            .query_opt(
                "SELECT COUNT(*) FROM pg_tables
                WHERE schemaname = 'public'
                AND tablename IN ('nations', 'towns')",
                &[],
            )?
            .ok_or(String::from("Postgresql tables_exists: internal error."))?
            .get::<_, i64>(0)
            == 2)
    }
}

impl DbConnection for PostgresConnection {
    fn open_existing(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut result = Self::create_connection(options)?;
        if !result.tables_exist()? {
            return Err("Postgresql open_existing: tables missing.".into());
        }
        Ok(result)
    }

    fn open_existing_truncated(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut result = Self::create_connection(options)?;
        if !result.tables_exist()? {
            return Err("Postgresql open_existing_truncated: tables missing.".into());
        }
        result.truncate_tables();
        Ok(result)
    }

    fn create(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut result = Self::create_connection(options)?;
        if result.tables_exist()? {
            return Err("Postgresql create: tables already exist.".into());
        }
        result.create_tables();
        Ok(result)
    }

    fn open_or_create(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut result = Self::create_connection(options)?;
        if !result.tables_exist()? {
            result.create_tables();
        }
        Ok(result)
    }

    fn open_truncated_or_create(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut result = Self::create_connection(options)?;
        if result.tables_exist()? {
            result.truncate_tables();
        } else {
            result.create_tables();
        }
        Ok(result)
    }

    fn insert_nation(&mut self, nation: &Nation) -> Result<NationId, Box<dyn std::error::Error>> {
        let result = self.conn.query_one(
            "INSERT INTO Nations (
                name, capital_id
            ) VALUES (
                $1, $2
            ) RETURNING rowid",
            &[&nation.name.0, &*value_or_null(&nation.capital_id)],
        )?;
        Ok(NationId::BigSerial(result.get(0)))
    }

    fn insert_town(&mut self, town: &Town) -> Result<TownId, Box<dyn std::error::Error>> {
        let result = self.conn.query_one(
            "INSERT INTO Towns (
                name, lat, long, nation_id
            ) VALUES (
                $1, $2, $3, $4
            ) RETURNING rowid",
            &[
                &town.name.0,
                &town.lat.0,
                &town.long.0,
                &town.nation_id.to_i64(),
            ],
        )?;
        Ok(TownId::BigSerial(result.get(0)))
    }

    fn delete_nation(&mut self, id: &NationId) -> Result<bool, Box<dyn std::error::Error>> {
        let deleted_lines = self.conn.execute(
            "DELETE FROM Nations WHERE rowid = $1 RETURNING rowid",
            &[&id.to_i64()],
        )?;
        Ok(deleted_lines == 1)
    }

    fn delete_town(&mut self, id: &TownId) -> Result<bool, Box<dyn std::error::Error>> {
        let deleted_lines = self.conn.execute(
            "DELETE FROM Towns WHERE rowid = $1 RETURNING rowid",
            &[&id.to_i64()],
        )?;
        Ok(deleted_lines == 1)
    }

    fn update_nation(
        &mut self,
        id: &NationId,
        nation: &Nation,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let updated_lines = self.conn.execute(
            "UPDATE Nations SET
                name = $2,
                capital_id = $3
            WHERE rowid = $1",
            &[
                &id.to_i64(),
                &nation.name.0,
                &*value_or_null(&nation.capital_id),
            ],
        )?;
        Ok(updated_lines == 1)
    }

    fn update_town(
        &mut self,
        id: &TownId,
        town: &Town,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let updated_lines = self.conn.execute(
            "UPDATE Towns SET
                name = $2,
                lat = $3,
                long = $4,
                nation_id = $5
                WHERE rowid = $1",
            &[
                &id.to_i64(),
                &town.name.0,
                &town.lat.0,
                &town.long.0,
                &town.nation_id.to_i64(),
            ],
        )?;
        Ok(updated_lines == 1)
    }

    fn get_nation(&mut self, id: &NationId) -> Result<Option<Nation>, Box<dyn std::error::Error>> {
        Ok(
            match self.conn.query_opt(
                "SELECT * FROM Nations
                WHERE rowid = $1",
                &[&id.to_i64()],
            )? {
                Some(result) => Some(Nation {
                    name: NationName(result.get("name")),
                    capital_id: OptionalTownId(match result.get("capital_id") {
                        Some(id) => Some(TownId::BigSerial(id)),
                        None => None,
                    }),
                }),
                None => None,
            },
        )
    }

    fn get_town(&mut self, id: &TownId) -> Result<Option<Town>, Box<dyn std::error::Error>> {
        Ok(
            match self.conn.query_opt(
                "SELECT * FROM Towns
                WHERE rowid = $1",
                &[&id.to_i64()],
            )? {
                Some(result) => Some(Town {
                    name: TownName(result.get("name")),
                    lat: Latitude(result.get("lat")),
                    long: Longitude(result.get("long")),
                    nation_id: NationId::BigSerial(result.get("nation_id")),
                }),
                None => None,
            },
        )
    }

    /*
    struct RowIterator<T> {
        inner: RowIter<'a>;
    }

    impl Iterator for RowIterator<T> {
        // we will be counting with usize
        type Item = T;

        // next() is the only required method
        fn next(&mut self) -> Option<Self::Item> {
            inner.next()
            */

    fn filter_nations_by_name(
        &mut self,
        name: &NationName,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(NationId, Nation), Box<dyn std::error::Error>>> + '_>,
        Box<dyn std::error::Error>,
    > {
        let it = self.conn.query_raw(
            "SELECT rowid, name, capital_id FROM Nations
            WHERE name = $1",
            [name.0.clone()].iter(),
        );
        match it {
            Ok(row_iter) => Ok(Box::new(row_iter_to_row_iterator(row_iter).map(
                |row_error: Result<Row, postgres::Error>| match row_error {
                    Ok(row) => Ok((
                        NationId::BigSerial(row.get("rowid")),
                        Nation {
                            name: NationName(row.get("name")),
                            capital_id: OptionalTownId(match row.get("capital_id") {
                                Some(id) => Some(TownId::BigSerial(id)),
                                None => None,
                            }),
                        },
                    )),
                    Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>),
                },
            ))),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn filter_towns_by_name(
        &mut self,
        name: &TownName,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn std::error::Error>>> + '_>,
        Box<dyn std::error::Error>,
    > {
        let it = self.conn.query_raw(
            "SELECT rowid, name, lat, long, nation_id FROM Towns
            WHERE name = $1",
            [name.0.clone()].iter(),
        );
        match it {
            Ok(row_iter) => Ok(Box::new(row_iter_to_row_iterator(row_iter).map(
                |row_error: Result<Row, postgres::Error>| match row_error {
                    Ok(row) => Ok((
                        TownId::BigSerial(row.get("rowid")),
                        Town {
                            name: TownName(row.get("name")),
                            lat: Latitude(row.get("lat")),
                            long: Longitude(row.get("long")),
                            nation_id: NationId::BigSerial(row.get("nation_id")),
                        },
                    )),
                    Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>),
                },
            ))),
            Err(error) => Err(Box::new(error)),
        }
    }

    fn filter_towns_by_lat_long(
        &mut self,
        min_lat: &Latitude,
        max_lat: &Latitude,
        min_long: &Longitude,
        max_long: &Longitude,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn std::error::Error>>> + '_>,
        Box<dyn std::error::Error>,
    > {
        let it = self.conn.query_raw(
            "SELECT rowid, name, lat, long, nation_id FROM Towns
            WHERE $1 <= lat AND lat <= $2
            AND $3 <= long AND long <= $4",
            [min_lat.0, max_lat.0, min_long.0, max_long.0].iter(),
        );
        match it {
            Ok(row_iter) => Ok(Box::new(row_iter_to_row_iterator(row_iter).map(
                |row_error: Result<Row, postgres::Error>| match row_error {
                    Ok(row) => Ok((
                        TownId::BigSerial(row.get("rowid")),
                        Town {
                            name: TownName(row.get("name")),
                            lat: Latitude(row.get("lat")),
                            long: Longitude(row.get("long")),
                            nation_id: NationId::BigSerial(row.get("nation_id")),
                        },
                    )),
                    Err(error) => Err(Box::new(error) as Box<dyn std::error::Error>),
                },
            ))),
            Err(error) => Err(Box::new(error)),
        }
    }
}

struct RowIterator<'a> {
    row_iter: RowIter<'a>,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Result<Row, postgres::Error>;
    fn next(&mut self) -> Option<<Self as std::iter::Iterator>::Item> {
        match self.row_iter.next() {
            Ok(Some(item)) => Some(Ok(item)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn row_iter_to_row_iterator(row_iter: RowIter) -> RowIterator {
    RowIterator { row_iter }
}
