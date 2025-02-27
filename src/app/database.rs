use std::collections::HashMap;

#[derive(Debug)]
pub struct Entity {
    id: u64,
    user_name: String,
    description: String,
}

const USER_NAME_MAX_LEN: usize = 32;
const DESCRIPTION_MAX_LEN: usize = 255;

// Serialized entity
// |header|[id][user_name][description]

impl Entity {
    pub fn new(id: u64, user_name: String, description: String) -> Self {
        // For now panic for invalid lenth args
        assert!(id != 0);
        assert!(user_name.len() < USER_NAME_MAX_LEN);
        assert!(description.len() < DESCRIPTION_MAX_LEN);

        Entity {
            id,
            user_name,
            description,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![0; 300];
        result[0] = self.user_name.len() as u8;
        result[1] = self.description.len() as u8;
        result[2..10].copy_from_slice(&self.id.to_le_bytes());
        result[10..10 + self.user_name.len()].copy_from_slice(self.user_name.as_bytes());
        result[42..42 + self.description.len()].copy_from_slice(self.description.as_bytes());
        return result;
    }
    pub fn deserialize(entity_bytes: &Vec<u8>) -> Self {
        assert_eq!(entity_bytes.len(), 300);
        let name_len = entity_bytes[0] as usize;
        let desc_len = entity_bytes[1] as usize;
        let id_offset = 2usize;
        let name_offset = 10usize;
        let desc_offset = 42usize;
        let id = u64::from_le_bytes(
            entity_bytes[id_offset..id_offset + 8]
                .try_into()
                .expect("How can a slice of size 8 be less than 8"),
        );
        let user_name = Self::get_str(entity_bytes, name_offset, name_len);
        let description = Self::get_str(entity_bytes, desc_offset, desc_len);
        return Entity::new(id, user_name, description);
    }
    fn get_str(entity_bytes: &Vec<u8>, offset: usize, len: usize) -> String {
        return String::from_utf8(entity_bytes[offset..offset + len].to_vec()).unwrap_or_else(
            |err| {
                eprintln!("[ERROR]: error parsing bytes to str {err:?}");
                return String::new();
            },
        );
    }
}

#[derive(Default, Debug)]
pub struct Database {
    page_table: HashMap<usize, Vec<u8>>,
    entities: usize,
    // TODO map of page_number to a 4096 size page
}

impl Database {
    pub fn new() -> Self {
        return Database::default();
    }
    pub fn select(&self) {
        // iterate page table
        // iterate entities per page
        let entities = self.entities;
        for entity in 0..entities {
            let page_num = entity / 13;
            let offset = entity * 300;
            if let Some(page) = self.page_table.get(&page_num) {
                let page_content = &page[offset..offset + 300];
                let entity = Entity::deserialize(&Vec::from(page_content));
                println!("page: {page_num}, offset: {offset}, content: {entity:?}");
            }
        }
    }
    pub fn insert(&mut self, entity: Entity) {
        let page = self.entities / 13;
        let offset = (self.entities % 13) * 300;
        if let Some(page_content) = self.page_table.get_mut(&page) {
            page_content[offset..offset + 300].copy_from_slice(&entity.serialize());
        } else {
            let mut arr = vec![0u8; 4096];
            arr[0..300].copy_from_slice(&entity.serialize());
            self.page_table.insert(page, arr);
        }
        self.entities += 1;
    }
}
