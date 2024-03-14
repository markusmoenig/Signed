use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub id: Uuid,

    pub objects: Vec<Object>,
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

impl Project {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            id: Uuid::new_v4(),

            objects: Vec::new(),
        }
    }

    /// Add an object
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    /// Get the tilemap of the given uuid.
    pub fn get_object_mut(&mut self, uuid: Uuid) -> Option<&mut Object> {
        self.objects.iter_mut().find(|o| o.id == uuid)
    }
}
