use crate::data_access::{DbConnection, Nation, NationId, NationName, Town, TownId, TownName};
use std::collections::{hash_map, HashMap};
use std::error::Error;

pub struct MockDbConnection {
    top_town_id: TownId,
    towns: HashMap<TownId, Town>,
    top_nation_id: NationId,
    nations: HashMap<NationId, Nation>,
}

impl MockDbConnection {
    pub fn create() -> Self {
        Self {
            top_town_id: TownId(0),
            towns: HashMap::<TownId, Town>::new(),
            top_nation_id: NationId(0),
            nations: HashMap::<NationId, Nation>::new(),
        }
    }
}

impl DbConnection for MockDbConnection {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn insert_nation(&mut self, nation: Nation) -> Result<NationId, Box<dyn Error>> {
        self.top_nation_id.0 += 1;
        self.nations.insert(self.top_nation_id, nation);
        Ok(self.top_nation_id)
    }

    fn insert_town(&mut self, town: Town) -> Result<TownId, Box<dyn Error>> {
        self.top_town_id.0 += 1;
        self.towns.insert(self.top_town_id, town);
        Ok(self.top_town_id)
    }

    fn delete_nation(&mut self, id: NationId) -> Result<bool, Box<dyn Error>> {
        Ok(self.nations.remove(&id).is_some())
    }

    fn delete_town(&mut self, id: TownId) -> Result<bool, Box<dyn Error>> {
        Ok(self.towns.remove(&id).is_some())
    }

    fn update_nation(
        &mut self,
        nation_id: NationId,
        nation: Nation,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(
            match self
                .nations
                .entry(nation_id)
                .and_modify(|item| *item = nation)
            {
                hash_map::Entry::Occupied(_) => true,
                hash_map::Entry::Vacant(_) => false,
            },
        )
    }

    fn update_town(&mut self, town_id: TownId, town: Town) -> Result<bool, Box<dyn Error>> {
        Ok(
            match self.towns.entry(town_id).and_modify(|item| *item = town) {
                hash_map::Entry::Occupied(_) => true,
                hash_map::Entry::Vacant(_) => false,
            },
        )
    }

    fn get_nation(&mut self, nation_id: NationId) -> Result<Option<Nation>, Box<dyn Error>> {
        Ok(match self.nations.get(&nation_id) {
            Some(nation) => Some(nation.clone()),
            None => None,
        })
    }

    fn get_town(&mut self, town_id: TownId) -> Result<Option<Town>, Box<dyn Error>> {
        Ok(match self.towns.get(&town_id) {
            Some(town) => Some(town.clone()),
            None => None,
        })
    }

    fn filter_nations_by_name(
        &mut self,
        name: NationName,
    ) -> Result<
        Box<dyn Iterator<Item = Result<(NationId, Nation), Box<dyn Error>>> + '_>,
        Box<dyn Error>,
    > {
        Ok(Box::new(self.nations.iter().filter_map(move |(k, v)| {
            if v.name == name {
                Some(Ok((*k, v.clone())))
            } else {
                None
            }
        })))
    }

    fn filter_towns_by_name(
        &mut self,
        name: TownName,
    ) -> Result<Box<dyn Iterator<Item = Result<(TownId, Town), Box<dyn Error>>> + '_>, Box<dyn Error>>
    {
        Ok(Box::new(self.towns.iter().filter_map(move |(k, v)| {
            if v.name == name {
                Some(Ok((*k, v.clone())))
            } else {
                None
            }
        })))
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
        Ok(Box::new(self.towns.iter().filter_map(move |(k, v)| {
            if min_lat <= v.lat && v.lat <= max_lat && min_long <= v.long && v.long <= max_long {
                Some(Ok((*k, v.clone())))
            } else {
                None
            }
        })))
    }
    */
}
