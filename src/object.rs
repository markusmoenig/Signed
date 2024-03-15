use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Object {
    pub name: String,
    pub id: Uuid,

    pub points: Vec<Point>,
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

impl Object {
    pub fn new() -> Self {
        let points = vec![
            Point {
                name: str!("Point A"),
                id: Uuid::new_v4(),
                position: Vec3f::zero(),
            },
            Point {
                name: str!("Point B"),
                id: Uuid::new_v4(),
                position: vec3f(0.0, 1.0, 0.0),
            },
        ];

        Object {
            name: str!("New Object"),
            id: Uuid::new_v4(),

            points,
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
