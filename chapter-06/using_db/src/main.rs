mod data_access;

use data_access::{
    mock_db::MockDbConnection, DbConnection, Latitude, Longitude, Nation, NationId, NationName,
    Town, TownId, TownName,
};
use std::error::Error;

use crate::data_access::postgres_db::PostgresConnection;
use crate::data_access::sqlite_db::SqliteConnection;
use crate::data_access::OptionalTownId;

fn main() -> Result<(), Box<dyn Error>> {
    println!("****** Using a mock DB ******");
    let t0 = std::time::Instant::now();
    process_world(&mut MockDbConnection::create())?;
    let t1 = std::time::Instant::now();

    println!();
    println!("****** Using a memory SQLite DB ******");
    let t2 = std::time::Instant::now();
    process_world(&mut SqliteConnection::create(":memory:"))?;
    let t3 = std::time::Instant::now();

    println!();
    println!("****** Using a persistent SQLite DB ******");
    let t4 = std::time::Instant::now();
    process_world(&mut SqliteConnection::create("world.db"))?;
    let t5 = std::time::Instant::now();

    println!();
    println!("****** Using a persistent Postgresql DB ******");
    let t6 = std::time::Instant::now();
    process_world(&mut PostgresConnection::create_with_string(
        "postgres://postgres:mypassword@localhost",
    )?)?;
    let t7 = std::time::Instant::now();

    eprintln!(
        "{} {} {} {}",
        (t1 - t0).as_micros(),
        (t3 - t2).as_micros(),
        (t5 - t4).as_micros(),
        (t7 - t6).as_micros(),
    );

    Ok(())
}

fn process_world(db: &mut dyn DbConnection) -> Result<(), Box<dyn Error>> {
    db.init()?;

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
    let france_id = db.insert_nation(france.clone())?;
    println!("Inserted {} {}", france_id.0, france.name.0);
    let uk_id = db.insert_nation(uk.clone())?;
    println!("Inserted {} {}", uk_id.0, uk.name.0);
    let germany_id = db.insert_nation(germany.clone())?;
    println!("Inserted {} {}", germany_id.0, germany.name.0);

    // Removing nations
    println!(
        "Removing nation with id 100 {}",
        db.delete_nation(NationId(100))?
    );
    println!(
        "Removing nation with id {} (UK) {}",
        uk_id.0,
        db.delete_nation(uk_id)?
    );
    println!(
        "Removing nation with id {} (UK) {}",
        uk_id.0,
        db.delete_nation(uk_id)?
    );

    // Defining towns
    let paris = Town {
        name: TownName("Paris".to_string()),
        lat: Latitude(1.1),
        long: Longitude(2.2),
        nation_id: france_id,
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
        nation_id: germany_id,
    };

    // Inserting towns
    let paris_id = db.insert_town(paris.clone())?;
    println!(
        "Inserted {} {} {} {} {}",
        paris_id.0, paris.name.0, paris.lat.0, paris.long.0, paris.nation_id.0
    );
    let berlin_id = db.insert_town(berlin.clone())?;
    println!(
        "Inserted {} {} {} {} {}",
        berlin_id.0, berlin.name.0, berlin.lat.0, berlin.long.0, berlin.nation_id.0
    );
    let london_id = db.insert_town(london.clone())?;
    println!(
        "Inserted {} {} {} {} {}",
        london_id.0, london.name.0, london.lat.0, london.long.0, london.nation_id.0
    );

    /*
    // Filtering towns by position
    println!("Towns with position in range lat 0 to 7 long 0 to 7");
    for row in
        db.filter_towns_by_lat_long(Latitude(0.), Latitude(7.), Longitude(0.), Longitude(7.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id.0, town.name.0, town.nation_id.0
            );
        }
    }

    println!("Towns with position in range lat 3 to 4 long 0 to 7");
    for row in
        db.filter_towns_by_lat_long(Latitude(3.), Latitude(4.), Longitude(0.), Longitude(7.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id.0, town.name.0, town.nation_id.0
            );
        }
    }

    println!("Towns with position in range lat 0 to 7 long 2 to 3");
    for row in
        db.filter_towns_by_lat_long(Latitude(0.), Latitude(7.), Longitude(2.), Longitude(3.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id.0, town.name.0, town.nation_id.0
            );
        }
    }

    println!("Towns with position in range lat 0 to 1 long 0 to 7");
    for row in
        db.filter_towns_by_lat_long(Latitude(0.), Latitude(1.), Longitude(0.), Longitude(7.))?
    {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id.0, town.name.0, town.nation_id.0
            );
        }
    }
    */

    // Removing towns
    println!("Removing town with id 100 {}", db.delete_town(TownId(100))?);
    println!(
        "Removing town with id {} (London) {}",
        london_id.0,
        db.delete_town(london_id)?
    );
    println!(
        "Removing town with id {} (London) {}",
        london_id.0,
        db.delete_town(london_id)?
    );

    // Getting nations
    println!(
        "Getting nation {} {}",
        france_id.0,
        db.get_nation(france_id).unwrap().unwrap().name.0
    );
    println!(
        "Getting nation {} {}",
        germany_id.0,
        db.get_nation(germany_id).unwrap().unwrap().name.0
    );

    // Getting towns
    println!(
        "Getting town {} {}",
        paris_id.0,
        db.get_town(paris_id).unwrap().unwrap().name.0
    );
    println!(
        "Getting town {} {}",
        berlin_id.0,
        db.get_town(berlin_id).unwrap().unwrap().name.0
    );

    // Filtering nations
    println!("Nations with name 'France':");
    for row in db.filter_nations_by_name(NationName("France".to_string()))? {
        if let Ok((id, nation)) = row {
            println!(
                "- id: {}, name: {}, capital_id: {:?}",
                id.0, nation.name.0, nation.capital_id.0
            );
        }
    }
    println!("Nations with name 'United Kingdom':");
    for row in db.filter_nations_by_name(NationName("United Kingdom".to_string()))? {
        if let Ok((id, nation)) = row {
            println!(
                "- id: {}, name: {}, capital_id: {:?}",
                id.0, nation.name.0, nation.capital_id.0
            );
        }
    }
    println!("Nations with name 'Germany':");
    for row in db.filter_nations_by_name(NationName("Germany".to_string()))? {
        if let Ok((id, nation)) = row {
            println!(
                "- id: {}, name: {}, capital_id: {:?}",
                id.0, nation.name.0, nation.capital_id.0
            );
        }
    }

    // Filtering towns by name
    println!("Towns with name 'Paris':");
    for row in db.filter_towns_by_name(TownName("Paris".to_string()))? {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id.0, town.name.0, town.nation_id.0
            );
        }
    }
    println!("Towns with name 'London':");
    for row in db.filter_towns_by_name(TownName("London".to_string()))? {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id.0, town.name.0, town.nation_id.0
            );
        }
    }
    println!("Towns with name 'Berlin':");
    for row in db.filter_towns_by_name(TownName("Berlin".to_string()))? {
        if let Ok((id, town)) = row {
            println!(
                "- id: {}, name: {}, nation_id: {}",
                id.0, town.name.0, town.nation_id.0
            );
        }
    }
    /*
     */

    Ok(())
}
