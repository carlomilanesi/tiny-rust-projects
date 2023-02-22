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
        let prepared = tx.prepare()?;
        prepared.commit()?;
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
    let prepared = tx.prepare()?;
    prepared.commit()?;

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
    let prepared = tx.prepare()?;
    prepared.commit()?;
    Ok(())
}

pub fn delete(persy: &Persy, delete_id: PersyId) -> Result<(), Box<dyn std::error::Error>> {
    println!("About to delete {delete_id}");
    //Begin a transaction
    let mut tx = persy.begin()?;
    // delete the record
    tx.delete("seg", &delete_id)?;
    //Commit the tx.
    let prepared = tx.prepare()?;
    prepared.commit()?;
    Ok(())
}
*/

//-------------------------------------------------------------------------------------
use crate::data_access::{
    DbConnection, Latitude, Longitude, Nation, NationId, NationName, OptionalTownId, Town, TownId,
    TownName,
};
use std::error::Error;

pub struct PersyConnection {
    conn: Persy,
}

impl PersyConnection {}

/*
trait PersyParam {
    fn param(self, name: &str, value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

impl PersyParam for Statement<'_> {
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
*/

fn segments_exist(db: &Persy) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(db.exists_segment("Nations")? && db.exists_segment("Towns")?)
}

fn create_segments(db: &Persy) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin()?;
    tx.create_segment("Nations")?;
    tx.create_segment("Towns")?;
    let prepared = tx.prepare()?;
    prepared.commit()?;
    Ok(())
}

fn truncate_segments(db: &Persy) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = db.begin()?;
    tx.drop_segment("Nations")?;
    tx.drop_segment("Towns")?;
    tx.create_segment("Nations")?;
    tx.create_segment("Towns")?;
    let prepared = tx.prepare()?;
    prepared.commit()?;
    Ok(())
}

impl DbConnection for PersyConnection {
    fn open_existing(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let db = Persy::open(options, Config::new())?;
        if !segments_exist(&db)? {
            return Err("Persy open_existing: segments missing.".into());
        }
        Ok(PersyConnection { conn: db })
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
        Ok(PersyConnection { conn: db })
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
        Ok(PersyConnection { conn: db })
    }

    fn open_or_create(options: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let db = Persy::open_or_create_with(options, Config::new(), |_| Ok(()))?;
        if !segments_exist(&db)? {
            create_segments(&db)?;
        }
        Ok(PersyConnection { conn: db })
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
        Ok(PersyConnection { conn: db })
    }

    fn insert_nation(&mut self, nation: &Nation) -> Result<NationId, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        //let data = serde_json::to_vec(&nation)?;
        let data = bincode::serialize(&nation)?;
        let id = tx.insert("Nations", &data)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
        Ok(NationId::PersyKey(id.to_string()))
    }

    fn insert_town(&mut self, town: &Town) -> Result<TownId, Box<dyn Error>> {
        let mut tx = self.conn.begin()?;
        //let data = serde_json::to_vec(&town)?;
        let data = bincode::serialize(&town)?;
        let id = tx.insert("Towns", &data)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
        Ok(TownId::PersyKey(id.to_string()))
    }

    fn delete_nation(&mut self, id: &NationId) -> Result<bool, Box<dyn Error>> {
        /*
        let mut command = self
            .conn
            .prepare("DELETE FROM Nations WHERE ROWID = :id RETURNING ROWID")?
            .param(":id", id.to_value())?;
        command.next()?;
        Ok(command.read::<i64, _>(0)? == id.0)
        */
        Ok(false)
    }

    fn delete_town(&mut self, id: &TownId) -> Result<bool, Box<dyn Error>> {
        /*
        let mut command = self
            .conn
            .prepare("DELETE FROM Towns WHERE ROWID = :id RETURNING ROWID")?
            .param(":id", id.to_value())?;
        command.next()?;
        Ok(command.read::<i64, _>(0)? == id.0)
        */
        Ok(false)
    }

    fn update_nation(&mut self, id: &NationId, nation: &Nation) -> Result<bool, Box<dyn Error>> {
        /*
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
        */
        Ok(false)
    }

    fn update_town(&mut self, id: &TownId, town: &Town) -> Result<bool, Box<dyn Error>> {
        /*
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
        */
        Ok(false)
    }

    fn get_nation(&mut self, id: &NationId) -> Result<Option<Nation>, Box<dyn Error>> {
        if let NationId::PersyKey(id) = id {
            Ok(self
                .conn
                .scan("Nations")?
                .into_iter()
                .filter_map(move |(k, v)| {
                    //let nation: Nation = serde_json::from_slice(&v).unwrap();
                    let nation: Nation = bincode::deserialize(&v).unwrap();
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
                    //let town: Town = serde_json::from_slice(&v).unwrap();
                    let town: Town = bincode::deserialize(&v).unwrap();
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
                //let nation: Nation = serde_json::from_slice(&v).unwrap();
                let nation: Nation = bincode::deserialize(&v).unwrap();
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
                //let town: Town = serde_json::from_slice(&v).unwrap();
                let town: Town = bincode::deserialize(&v).unwrap();
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
                //let town: Town = serde_json::from_slice(&v).unwrap();
                let town: Town = bincode::deserialize(&v).unwrap();
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
