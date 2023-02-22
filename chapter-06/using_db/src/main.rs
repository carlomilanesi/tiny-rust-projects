mod data_access;

use std::time::Instant;

//use data_access::persy_db::{open, delete, insert, scan, update};
use data_access::{
    mock_db::MockDbConnection, DbConnection, Latitude, Longitude, Nation, NationId, NationName,
    Town, TownId, TownName,
};
use std::error::Error;

use crate::data_access::persy_db::PersyConnection;
use crate::data_access::postgres_db::PostgresConnection;
use crate::data_access::sqlite_db::SqliteConnection;
use crate::data_access::OptionalTownId;

fn main() -> Result<(), Box<dyn Error>> {
    /*
    let bergamo = Town {
        name: TownName("Bergamo".to_string()),
        lat: Latitude(12.34),
        long: Longitude(-4.567),
        nation_id: NationId::BigSerial(189),
    };
    serde_json(&bergamo);
    serde_postcard(&bergamo);
    serde_messagepack(&bergamo);
    serde_bson(&bergamo);
    //serde_dbus(&bergamo);
    serde_bincode(&bergamo);
    */
    /*
    let db = open("world.persy")?;
    let id = insert(&db)?;
    scan(&db)?;
    update(&db, id)?;
    delete(&db, id)?;
    */

    // /*
    println!("****** Using a mock DB ******");
    let mock_start = Instant::now();
    let mut db1 = MockDbConnection::open_truncated_or_create("")?;
    let mock_creation = mock_start.elapsed();
    process_world(&mut db1)?;
    let mock_query = mock_start.elapsed() - mock_creation;

    println!();
    println!("****** Using a memory SQLite DB ******");
    let memory_sqlite_start = Instant::now();
    let mut db2 = SqliteConnection::open_truncated_or_create(":memory:")?;
    let memory_sqlite_creation = memory_sqlite_start.elapsed();
    process_world(&mut db2)?;
    let memory_sqlite_query = memory_sqlite_start.elapsed() - memory_sqlite_creation;

    println!();
    println!("****** Using a persistent SQLite DB ******");
    let storage_sqlite_start = Instant::now();
    let mut db3 = SqliteConnection::open_truncated_or_create("world.db")?;
    let storage_sqlite_creation = storage_sqlite_start.elapsed();
    process_world(&mut db3)?;
    let storage_sqlite_query = storage_sqlite_start.elapsed() - storage_sqlite_creation;

    println!();
    println!("****** Using a persistent Postgresql DB ******");
    let postgresql_start = Instant::now();
    let mut db4 =
        PostgresConnection::open_truncated_or_create("postgres://postgres:myp@localhost")?;
    let postgresql_creation = postgresql_start.elapsed();
    process_world(&mut db4)?;
    let postgresql_query = postgresql_start.elapsed() - postgresql_creation;

    println!();
    println!("****** Using a persistent Persy DB ******");
    let persy_start = Instant::now();
    let mut db5 = PersyConnection::open_truncated_or_create("world.persy")?;
    let persy_creation = persy_start.elapsed();
    process_world(&mut db5)?;
    let persy_query = persy_start.elapsed() - persy_creation;

    eprintln!(
        "Mock: {}, {}",
        mock_creation.as_micros(),
        mock_query.as_micros()
    );
    eprintln!(
        "Memory SQLite: {}, {}",
        memory_sqlite_creation.as_micros(),
        memory_sqlite_query.as_micros()
    );
    eprintln!(
        "Storage SQLite: {}, {}",
        storage_sqlite_creation.as_micros(),
        storage_sqlite_query.as_micros()
    );
    eprintln!(
        "PostgreSQL: {}, {}",
        postgresql_creation.as_micros(),
        postgresql_query.as_micros()
    );
    eprintln!(
        "Persy: {}, {}",
        persy_creation.as_micros(),
        persy_query.as_micros()
    );
    // */
    Ok(())
}

fn process_world(db: &mut dyn DbConnection) -> Result<(), Box<dyn Error>> {
    // Defining nations
    let france = Nation {
        name: NationName("France".to_string()),
        capital_id: OptionalTownId(None),
    };
    let uk = Nation {
        name: NationName("United Kingdom".to_string()),
        capital_id: OptionalTownId(None),
    };
    let germany = Nation {
        name: NationName("Germany".to_string()),
        capital_id: OptionalTownId(None),
    };

    // Inserting nations
    let france_id = db.insert_nation(&france)?;
    println!("Inserted {} {}", france_id, france.name.0);
    let uk_id = db.insert_nation(&uk)?;
    println!("Inserted {} {}", uk_id, uk.name.0);
    let germany_id = db.insert_nation(&germany)?;
    println!("Inserted {} {}", germany_id, germany.name.0);

    // Removing nations
    println!(
        "Removing nation with id 100 {}",
        db.delete_nation(&NationId::BigSerial(100))?
    );
    println!(
        "Removing nation with id {} (UK) {}",
        uk_id,
        db.delete_nation(&uk_id)?
    );
    println!(
        "Removing nation with id {} (UK) {}",
        uk_id,
        db.delete_nation(&uk_id)?
    );

    // Defining towns
    let paris = Town {
        name: TownName("Paris".to_string()),
        lat: Latitude(1.1),
        long: Longitude(2.2),
        nation_id: france_id.clone(),
    };
    let london = Town {
        name: TownName("London".to_string()),
        lat: Latitude(3.3),
        long: Longitude(4.4),
        nation_id: uk_id,
    };
    let berlin = Town {
        name: TownName("Berlin".to_string()),
        lat: Latitude(5.5),
        long: Longitude(6.6),
        nation_id: germany_id.clone(),
    };

    // Inserting towns
    let paris_id = db.insert_town(&paris)?;
    println!(
        "Inserted {} {} {} {} {}",
        paris_id, paris.name.0, paris.lat.0, paris.long.0, paris.nation_id
    );
    let berlin_id = db.insert_town(&berlin)?;
    println!(
        "Inserted {} {} {} {} {}",
        berlin_id, berlin.name.0, berlin.lat.0, berlin.long.0, berlin.nation_id
    );
    let london_id = db.insert_town(&london)?;
    println!(
        "Inserted {} {} {} {} {}",
        london_id, london.name.0, london.lat.0, london.long.0, london.nation_id
    );

    // Filtering towns by position
    println!("Towns with position in range lat 0 to 7 long 0 to 7");
    for row in
        db.filter_towns_by_lat_long(&Latitude(0.), &Latitude(7.), &Longitude(0.), &Longitude(7.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id, town.name.0, town.nation_id
            );
        }
    }

    println!("Towns with position in range lat 3 to 4 long 0 to 7");
    for row in
        db.filter_towns_by_lat_long(&Latitude(3.), &Latitude(4.), &Longitude(0.), &Longitude(7.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id, town.name.0, town.nation_id
            );
        }
    }

    println!("Towns with position in range lat 0 to 7 long 2 to 3");
    for row in
        db.filter_towns_by_lat_long(&Latitude(0.), &Latitude(7.), &Longitude(2.), &Longitude(3.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id, town.name.0, town.nation_id
            );
        }
    }

    println!("Towns with position in range lat 0 to 1 long 0 to 7");
    for row in
        db.filter_towns_by_lat_long(&Latitude(0.), &Latitude(1.), &Longitude(0.), &Longitude(7.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id, town.name.0, town.nation_id
            );
        }
    }

    // Removing towns
    println!(
        "Removing town with id 100 {}",
        db.delete_town(&TownId::BigSerial(100))?
    );
    println!(
        "Removing town with id {} (London) {}",
        london_id,
        db.delete_town(&london_id)?
    );
    println!(
        "Removing town with id {} (London) {}",
        london_id,
        db.delete_town(&london_id)?
    );

    // Getting nations
    println!(
        "Getting nation {} {}",
        france_id,
        db.get_nation(&france_id).unwrap().unwrap().name.0
    );
    println!(
        "Getting nation {} {}",
        germany_id,
        db.get_nation(&germany_id).unwrap().unwrap().name.0
    );

    // Getting towns
    println!(
        "Getting town {} {}",
        paris_id,
        db.get_town(&paris_id).unwrap().unwrap().name.0
    );
    println!(
        "Getting town {} {}",
        berlin_id,
        db.get_town(&berlin_id).unwrap().unwrap().name.0
    );

    // Filtering nations
    println!("Nations with name 'France':");
    for row in db.filter_nations_by_name(&NationName("France".to_string()))? {
        if let Ok((id, nation)) = row {
            println!(
                "- id: {}, name: {}, capital_id: {:?}",
                id, nation.name.0, nation.capital_id.0
            );
        }
    }
    println!("Nations with name 'United Kingdom':");
    for row in db.filter_nations_by_name(&NationName("United Kingdom".to_string()))? {
        if let Ok((id, nation)) = row {
            println!(
                "- id: {}, name: {}, capital_id: {:?}",
                id, nation.name.0, nation.capital_id.0
            );
        }
    }
    println!("Nations with name 'Germany':");
    for row in db.filter_nations_by_name(&NationName("Germany".to_string()))? {
        if let Ok((id, nation)) = row {
            println!(
                "- id: {}, name: {}, capital_id: {:?}",
                id, nation.name.0, nation.capital_id.0
            );
        }
    }

    // Filtering towns by name
    println!("Towns with name 'Paris':");
    for row in db.filter_towns_by_name(&TownName("Paris".to_string()))? {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id, town.name.0, town.nation_id
            );
        }
    }
    println!("Towns with name 'London':");
    for row in db.filter_towns_by_name(&TownName("London".to_string()))? {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id, town.name.0, town.nation_id
            );
        }
    }
    println!("Towns with name 'Berlin':");
    for row in db.filter_towns_by_name(&TownName("Berlin".to_string()))? {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id, town.name.0, town.nation_id
            );
        }
    }
    /*
     */

    Ok(())
}

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn serde_json(town: &Town) {
    let t = std::time::Instant::now();
    let json_serialized = serde_json::to_string(&town).unwrap();
    println!(
        "json serialized = {} in {:?}",
        json_serialized.len(),
        t.elapsed()
    );
    let t = std::time::Instant::now();
    let deserialized: Town = serde_json::from_str(&json_serialized).unwrap();
    assert_eq!(deserialized, *town);
    println!(
        "json deserialized = {} in {:?}",
        std::mem::size_of_val(&deserialized),
        t.elapsed()
    );
}

extern crate alloc;

fn serde_postcard(town: &Town) {
    let t = std::time::Instant::now();
    //let postcard_serialized = postcard::to_vec::<_, 256>(&town).unwrap();
    let postcard_serialized: alloc::vec::Vec<u8> = postcard::to_allocvec(&town).unwrap();
    println!(
        "postcard serialized = {} in {:?}",
        postcard_serialized.len(),
        t.elapsed()
    );
    let t = std::time::Instant::now();
    let deserialized: Town = postcard::from_bytes(&postcard_serialized).unwrap();
    assert_eq!(deserialized, *town);
    println!(
        "postcard deserialized = {} in {:?}",
        std::mem::size_of_val(&deserialized),
        t.elapsed()
    );
}

fn serde_messagepack(town: &Town) {
    let t = std::time::Instant::now();
    let messagepack_serialized = rmp_serde::to_vec(&town).unwrap();
    println!(
        "messagepack serialized = {} in {:?}",
        messagepack_serialized.len(),
        t.elapsed()
    );
    let t = std::time::Instant::now();
    let deserialized: Town = rmp_serde::from_slice(&messagepack_serialized).unwrap();
    assert_eq!(deserialized, *town);
    println!(
        "messagepack deserialized = {} in {:?}",
        std::mem::size_of_val(&deserialized),
        t.elapsed()
    );
}

fn serde_bson(town: &Town) {
    let t = std::time::Instant::now();
    let bson_serialized = bson::to_vec(&town).unwrap();
    println!(
        "bson serialized = {} in {:?}",
        bson_serialized.len(),
        t.elapsed()
    );
    let t = std::time::Instant::now();
    let deserialized: Town = bson::from_slice(&bson_serialized).unwrap();
    assert_eq!(deserialized, *town);
    println!(
        "bson deserialized = {} in {:?}",
        std::mem::size_of_val(&deserialized),
        t.elapsed()
    );
}

fn serde_bincode(town: &Town) {
    let t = std::time::Instant::now();
    let bincode_serialized = bincode::serialize(&town).unwrap();
    println!(
        "bincode serialized = {} in {:?}",
        bincode_serialized.len(),
        t.elapsed()
    );
    let t = std::time::Instant::now();
    let deserialized: Town = bincode::deserialize(&bincode_serialized).unwrap();
    assert_eq!(deserialized, *town);
    println!(
        "bincode deserialized = {} in {:?}",
        std::mem::size_of_val(&deserialized),
        t.elapsed()
    );
}
