use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Object {
    pub name: String,
    pub id: Uuid,
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

impl Object {
    pub fn new() -> Self {
        Object {
            name: str!("New Object"),
            id: Uuid::new_v4(),
        }
    }

    /*
    /// Add a tilemap
    pub fn add_tilemap(&mut self, tilemap: Tilemap) {
        self.tilemaps.push(tilemap)
    }

    /// Get the tilemap of the given uuid.
    pub fn get_tilemap(&mut self, uuid: Uuid) -> Option<&mut Tilemap> {
        for t in &mut self.tilemaps {
            if t.id == uuid {
                return Some(t);
            }
        }
        None
        }*/
}
