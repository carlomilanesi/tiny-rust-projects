use persy::{Config, Persy, PersyId};

/*
pub fn open(filename: &str) -> Result<Persy, Box<dyn std::error::Error>> {
    // open or create the file
    println!("About to open or create",);
    let persy = Persy::open_or_create_with(filename, Config::new(), |persy| {
        println!("Created and about to initialize",);
        //Start a transaction all the operations in persy are done inside a transaction.
        let mut tx = persy.begin()?;

        //Create a segment called "seg" using the started tx.
        tx.create_segment("seg")?;

        //Commit the tx.
        tx.prepare()?.commit()?;
        println!("Initialized",);
        Ok(())
    })?;
    Ok(persy)
}

pub fn insert(persy: &Persy) -> Result<PersyId, Box<dyn std::error::Error>> {
    println!("About to insert",);
    //Start a transaction all the operations in persy are done inside a transaction.
    let mut tx = persy.begin()?;

    //Prepare some raw data
    let data = vec![1; 20];

    //Insert the data inside the segment with the current tx.
    let id = tx.insert("seg", &data)?;

    //Commit the tx.
    tx.prepare()?.commit()?;

    Ok(id)
}

pub fn scan(persy: &Persy) -> Result<(), Box<dyn std::error::Error>> {
    println!("About to scan");
    let mut id = None;
    let to_find = vec![1; 20];
    for (read_id, content) in persy.scan("seg")? {
        //.... do something with the record.id and record.content
        if content == to_find {
            id = Some(read_id);
            println!("Found {read_id}");
            break;
        }
    }
    Ok(())
}

pub fn update(persy: &Persy, update_id: PersyId) -> Result<(), Box<dyn std::error::Error>> {
    println!("About to update {update_id}");
    //Begin a transaction
    let mut tx = persy.begin()?;
    let new_data = vec![2; 20];
    // Update the record with new data
    tx.update("seg", &update_id, &new_data)?;
    //Commit the tx.
    tx.prepare()?.commit()?;
    Ok(())
}

pub fn delete(persy: &Persy, delete_id: PersyId) -> Result<(), Box<dyn std::error::Error>> {
    println!("About to delete {delete_id}");
    //Begin a transaction
    let mut tx = persy.begin()?;
    // delete the record
    tx.delete("seg", &delete_id)?;
    //Commit the tx.
    tx.prepare()?.commit()?;
    Ok(())
}
*/

//-------------------------------------------------------------------------------------
/*
There is the trait `DbConnection`, containing several function signatures, including `insert_nation`.
This trait must be implemented by several types: `MockDbConnection`, `SqliteConnection`, `PostgresConnection`, `PersyConnection`.
So, each of them must implement the function `insert_nation`.
Without changing the trait `DbConnection` nor its implementation for type different from `PersyConnection`, we want specify that
an implementation of `DbConnection` for `PersyConnection` must use a specified type.
an implementation of  the By changing

 for PersyConnection<'_, Serder> {

*/

use crate::data_access::{
    DbConnection, Latitude, Longitude, Nation, NationId, NationName, OptionalTownId, Town, TownId,
    TownName,
};
use std::{error::Error, str::FromStr};

use serde::{Deserialize, Serialize};

pub trait Serder {
    fn serialize<T: serde::Serialize>(obj: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    fn deserialize<'de, T: serde::Deserialize<'de>>(
        buffer: &'de [u8],
    ) -> Result<T, Box<dyn std::error::Error>>;
}

pub struct BincodeSerder;

impl Serder for BincodeSerder {
    fn serialize<T: serde::Serialize>(obj: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(obj)?)
    }

    fn deserialize<'de, T: serde::Deserialize<'de>>(
        buffer: &'de [u8],
    ) -> Result<T, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(buffer)?)
    }
}

pub struct JsonSerder;

impl Serder for JsonSerder {
    fn serialize<T: serde::Serialize>(obj: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(serde_json::to_vec(obj)?)
    }

    fn deserialize<'de, T: serde::Deserialize<'de>>(
        buffer: &'de [u8],
    ) -> Result<T, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(buffer)?)
    }
}

pub struct PersyConnection<S>
where
    S: Serder,
{
    conn: Persy,
    phantom: std::marker::PhantomData<S>,
}

impl<S> PersyConnection<S>
where
    S: Serder,
{
    fn new(db: Persy) -> Self {
        Self {
            conn: db,
            phantom: std::marker::PhantomData::<S>,
        }
    }
}

impl<S> DbConnection for PersyConnection<S>
where
    S: Serder,
{
    fn open_existing(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let db = Persy::open(options, Config::new())?;
        if !segments_exist(&db)? {
            return Err("Persy open_existing: segments missing.".into());
        }
        Ok(PersyConnection::new(db))
    }

    fn open_existing_truncated(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut db = Persy::open(options, Config::new())?;
        if !segments_exist(&db)? {
            return Err("Persy open_existing_truncated: segments missing.".into());
        }
        truncate_segments(&mut db)?;
        Ok(PersyConnection::new(db))
    }

    fn create(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        if std::path::Path::new(options).exists() {
            return Err("Persy create: file already exists.".into());
        }
        let db = Persy::open_or_create_with(options, Config::new(), |persy| {
            create_segments(&persy)?;
            Ok(())
        })?;
        Ok(PersyConnection::new(db))
    }

    fn open_or_create(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let db = Persy::open_or_create_with(options, Config::new(), |_| Ok(()))?;
        if !segments_exist(&db)? {
            create_segments(&db)?;
        }
        Ok(PersyConnection::new(db))
    }

    fn open_truncated_or_create(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let db = Persy::open_or_create_with(options, Config::new(), |_| Ok(()))?;
        if segments_exist(&db)? {
            truncate_segments(&db)?;
        } else {
            create_segments(&db)?;
        }
        Ok(PersyConnection::new(db))
    }

    fn insert_nation(&mut self, nation: &Nation) -> Result<NationId, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        let data = S::serialize(&nation)?;
        let id = tx.insert("Nations", &data)?;
        tx.prepare()?.commit()?;
        Ok(NationId::PersyKey(id.to_string()))
    }

    fn insert_town(&mut self, town: &Town) -> Result<TownId, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        let data = S::serialize(&town)?;
        let id = tx.insert("Towns", &data)?;
        tx.prepare()?.commit()?;
        Ok(TownId::PersyKey(id.to_string()))
    }

    fn delete_nation(&mut self, id: &NationId) -> Result<bool, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        let mut success = false;
        if let NationId::PersyKey(key) = id {
            success = tx.delete("Nations", &PersyId::from_str(key)?).is_ok();
            tx.prepare()?.commit()?;
        }
        Ok(success)
    }

    fn delete_town(&mut self, id: &TownId) -> Result<bool, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        let mut success = false;
        if let TownId::PersyKey(key) = id {
            success = tx.delete("Towns", &PersyId::from_str(key)?).is_ok();
            tx.prepare()?.commit()?;
        }
        Ok(success)
    }

    fn update_nation(&mut self, id: &NationId, nation: &Nation) -> Result<bool, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        let new_data = S::serialize(nation)?;
        let mut success = false;
        if let NationId::PersyKey(key) = id {
            success = tx
                .update("Nations", &PersyId::from_str(key)?, &new_data)
                .is_ok();
            tx.prepare()?.commit()?;
        }
        Ok(success)
    }

    fn update_town(&mut self, id: &TownId, town: &Town) -> Result<bool, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        let new_data = S::serialize(town)?;
        let mut success = false;
        if let TownId::PersyKey(key) = id {
            success = tx
                .update("Towns", &PersyId::from_str(key)?, &new_data)
                .is_ok();
            tx.prepare()?.commit()?;
        }
        Ok(success)
    }

    fn get_nation(&mut self, id: &NationId) -> Result<Option<Nation>, Box<dyn Error>> {
        if let NationId::PersyKey(id) = id {
            Ok(self
                .conn
                .scan("Nations")?
                .into_iter()
                .filter_map(move |(k, v)| {
                    let nation = S::deserialize::<Nation>(v.as_slice()).unwrap();
                    if *id == k.to_string() {
                        Some(nation)
                    } else {
                        None
                    }
                })
                .next())
        } else {
            Err("Key is not a PersyId".into())
        }
    }

    fn get_town(&mut self, id: &TownId) -> Result<Option<Town>, Box<dyn Error>> {
        if let TownId::PersyKey(id) = id {
            Ok(self
                .conn
                .scan("Towns")?
                .into_iter()
                .filter_map(move |(k, v)| {
                    let town: Town = S::deserialize(v.as_slice()).unwrap();
                    if *id == k.to_string() {
                        Some(town)
                    } else {
                        None
                    }
                })
                .next())
        } else {
            Err("Key is not a PersyId".into())
        }
    }

    fn filter_nations_by_name(
        &mut self,
        name: &NationName,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(NationId, Nation), Box<dyn Error>>> + '_>,
        Box<dyn Error>,
    > {
        let name = name.clone();
        Ok(Box::new(self.conn.scan("Nations")?.into_iter().filter_map(
            move |(k, v)| {
                let nation: Nation = S::deserialize(v.as_slice()).unwrap();
                if name == nation.name {
                    Some(Ok((NationId::PersyKey(k.to_string()), nation)))
                } else {
                    None
                }
            },
        )))
    }

    fn filter_towns_by_name(
        &mut self,
        name: &TownName,
    ) -> Result<Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>, Box<dyn Error>>
    {
        let name = name.clone();
        Ok(Box::new(self.conn.scan("Towns")?.into_iter().filter_map(
            move |(k, v)| {
                let town: Town = S::deserialize(v.as_slice()).unwrap();
                if name == town.name {
                    Some(Ok((TownId::PersyKey(k.to_string()), town)))
                } else {
                    None
                }
            },
        )))
    }

    fn filter_towns_by_lat_long(
        &mut self,
        min_lat: &Latitude,
        max_lat: &Latitude,
        min_long: &Longitude,
        max_long: &Longitude,
    ) -> Result<Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>, Box<dyn Error>>
    {
        let min_lat = min_lat.clone();
        let max_lat = max_lat.clone();
        let min_long = min_long.clone();
        let max_long = max_long.clone();
        Ok(Box::new(self.conn.scan("Towns")?.into_iter().filter_map(
            move |(k, v)| {
                let town: Town = S::deserialize(v.as_slice()).unwrap();

                if min_lat <= town.lat
                    && town.lat <= max_lat
                    && min_long <= town.long
                    && town.long <= max_long
                {
                    Some(Ok((TownId::PersyKey(k.to_string()), town)))
                } else {
                    None
                }
            },
        )))
    }
}

fn segments_exist(db: &Persy) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(db.exists_segment("Nations")? && db.exists_segment("Towns")?)
}

fn create_segments(db: &Persy) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin()?;
    tx.create_segment("Nations")?;
    tx.create_segment("Towns")?;
    tx.prepare()?.commit()?;
    Ok(())
}

fn truncate_segments(db: &Persy) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin()?;
    tx.drop_segment("Nations")?;
    tx.drop_segment("Towns")?;
    tx.create_segment("Nations")?;
    tx.create_segment("Towns")?;
    tx.prepare()?.commit()?;
    Ok(())
}
