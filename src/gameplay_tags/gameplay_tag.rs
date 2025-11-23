use super::gameplay_tag_manager::GameplayTagManager;
use crate::unique_name::UniqueNamePool;
use bevy::ecs::system::SystemParam;
use bevy::prelude::ResMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameplayTag {
    pub tag_bit_index: u16,
}
#[derive(SystemParam)]
pub struct GameplayTagRegister<'w> {
    pub unique_name_pool: ResMut<'w, UniqueNamePool>,
    pub gameplay_tag_manager: ResMut<'w, GameplayTagManager>,
}

impl<'w> GameplayTagRegister<'w> {
    pub fn request_or_register_tag(&mut self, full_tag_name: &str) -> GameplayTag {
        let unique_name = self.unique_name_pool.new_name(full_tag_name);

        if let Some(tag) = self.gameplay_tag_manager.get_tag(unique_name) {
            return tag;
        }

        let parent_tag_index = full_tag_name
            .rsplit_once('.')
            // Found a parent string (e.g., "Ability.Fireball" -> "Ability")
            .map(|(parent_name, _)| {
                // *** RECURSIVE CALL ***
                // Ensure the parent is registered before proceeding
                let parent_tag = self.request_or_register_tag(parent_name);
                parent_tag.tag_bit_index
            });

        self.gameplay_tag_manager
            .register_tag_internal(unique_name, parent_tag_index)
    }
}
