use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Point {
    pub name: String,
    pub id: Uuid,
    pub position: Vec3f,
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

impl Point {
    pub fn new() -> Self {
        Point {
            name: String::new(),
            id: Uuid::new_v4(),
            position: Vec3f::zero(),
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
