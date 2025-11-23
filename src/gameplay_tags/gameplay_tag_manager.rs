use super::gameplay_tag::GameplayTag;
use super::gameplay_tag_container::{add_bit_with_tag, GameplayTagBits};
use crate::unique_name::UniqueName;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
pub const MAX_TAG_COUNTS: usize = 512;
#[derive(Resource)]
pub struct GameplayTagManager {
    pub tag_name_to_index: HashMap<UniqueName, u16>,
    pub tag_parent_index: Vec<Option<u16>>,
    pub tag_inherited_bits: Vec<GameplayTagBits>,
    next_tag_index: u16,
}

impl Default for GameplayTagManager {
    fn default() -> Self {
        Self {
            tag_name_to_index: HashMap::new(),
            tag_parent_index: Vec::new(),
            tag_inherited_bits: Vec::new(),
            next_tag_index: 0,
        }
    }
}

impl GameplayTagManager {
    pub fn get_tag(&self, unique_name: UniqueName) -> Option<GameplayTag> {
        self.tag_name_to_index
            .get(&unique_name)
            .map(|&index| GameplayTag {
                tag_bit_index: index,
            })
    }

    pub fn register_tag_internal(
        &mut self,
        unique_name: UniqueName,
        parent_tag_index: Option<u16>,
    ) -> GameplayTag {
        if let Some(&index) = self.tag_name_to_index.get(&unique_name) {
            return GameplayTag {
                tag_bit_index: index,
            };
        }

        let new_index = self.next_tag_index;
        if new_index as usize >= MAX_TAG_COUNTS {
            panic!("Exceeded MAX_TAG_COUNTS");
        }

        // Create inherited bits: Start with parent's bits or new empty bits
        let mut inherited_bits = parent_tag_index
            .and_then(|p_index| self.tag_inherited_bits.get(p_index as usize).cloned())
            .unwrap_or_else(GameplayTagBits::default);

        // Set the current tag's own bit in the inherited bits
        let self_tag = GameplayTag { tag_bit_index: new_index };
        add_bit_with_tag(&mut inherited_bits, &self_tag);

        // Update the Manager data structures
        self.tag_name_to_index.insert(unique_name, new_index);
        self.tag_parent_index.push(parent_tag_index);
        self.tag_inherited_bits.push(inherited_bits);
        self.next_tag_index += 1;

        self_tag
    }

    pub fn get_inherited_bits(&self, tag: &GameplayTag) -> Option<&GameplayTagBits> {
        self.tag_inherited_bits.get(tag.tag_bit_index as usize)
    }
}
