use crate::sfnt::types::Fixed;
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct Maxp {
    pub version: Fixed,
    pub num_glyphs: u16,
    pub max_points: u16,
    pub max_contours: u16,
    pub max_component_points: u16,
    pub max_component_contours: u16,
    pub max_zones: u16,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}
