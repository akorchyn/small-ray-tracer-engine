use std::collections::HashMap;

use tobj::Material;

pub(crate) struct TextureLoader {
    textures: HashMap<usize, u8>,
}

impl TextureLoader {
    pub(crate) fn new() -> Self {
        TextureLoader {
            textures: HashMap::new(),
        }
    }

    pub(crate) fn load_texture(&mut self, id: usize, mat_info: &Material) {
        let entry = self
            .textures
            .entry(id)
            .or_insert_with(|| load_data(mat_info));
    }
}

fn load_data(mat_info: &Material) -> u8 {
    0
}
