use std::{collections::HashMap, io, io::BufReader};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Definition;
use crate::{extension::ReadExt, util};

/// Contains all the information about a certain npc fetched from the cache through
/// the [NpcLoader](../../loader/osrs/struct.NpcLoader.html).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct NpcDefinition {
    pub id: u16,
    pub name: String,
    pub size: usize,
    pub actions: [String; 5],
    pub visible_on_minimap: bool,
    pub combat_level: Option<u16>,
    pub configs: Vec<u16>,
    pub varbit_id: Option<u16>,
    pub varp_index: Option<u16>,
    pub interactable: bool,
    // pub pet: bool,
    pub follower: bool,
    pub lowpriorityfollowerops: bool,
    pub params: HashMap<u32, String>,
    pub model_data: NpcModelData,
    pub animation_data: NpcAnimationData,
    pub category: u16,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NpcModelData {
    pub models: Vec<u16>,
    pub chat_head_models: Vec<u16>,
    pub recolor_find: Vec<u16>,
    pub recolor_replace: Vec<u16>,
    pub retexture_find: Vec<u16>,
    pub retexture_replace: Vec<u16>,
    pub width_scale: u16,
    pub height_scale: u16,
    pub render_priority: bool,
    pub ambient: u8,
    pub contrast: u8,
    pub head_icon: Option<u16>,
    pub rotate_speed: u16,
    pub rotate_flag: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NpcAnimationData {
    pub standing: Option<u16>,
    pub walking: Option<u16>,
    pub rotate_left: Option<u16>,
    pub rotate_right: Option<u16>,
    pub rotate_180: Option<u16>,
    pub rotate_90_left: Option<u16>,
    pub rotate_90_right: Option<u16>,
    pub crawling: Option<u16>,
    pub crawling_rotate_left: Option<u16>,
    pub crawling_rotate_right: Option<u16>,
    pub crawling_rotate_180: Option<u16>,
    pub running: Option<u16>,
    pub running_rotate_left: Option<u16>,
    pub running_rotate_right: Option<u16>,
    pub running_rotate_180: Option<u16>,
}

impl Definition for NpcDefinition {
    fn new(id: u16, buffer: &[u8]) -> crate::Result<Self> {
        let mut reader = BufReader::new(buffer);
        let npc_def = decode_buffer(id, &mut reader)?;

        Ok(npc_def)
    }
}

#[allow(clippy::too_many_lines)]
fn decode_buffer(id: u16, reader: &mut BufReader<&[u8]>) -> io::Result<NpcDefinition> {
    let mut npc_def = NpcDefinition {
        id,
        interactable: true,
        visible_on_minimap: true,
        model_data: NpcModelData {
            rotate_flag: true,
            width_scale: 128,
            height_scale: 128,
            rotate_speed: 32,
            ..NpcModelData::default()
        },
        ..NpcDefinition::default()
    };

    loop {
        let opcode = reader.read_u8()?;

        match opcode {
            0 => break,
            1 => {
                let len = reader.read_u8()?;
                for _ in 0..len {
                    npc_def.model_data.models.push(reader.read_u16()?);
                }
            }
            2 => {
                npc_def.name = reader.read_string()?;
            }
            12 => {
                npc_def.size = reader.read_u8()? as usize;
            }
            13 => {
                npc_def.animation_data.standing = Some(reader.read_u16()?);
            }
            14 => {
                npc_def.animation_data.walking = Some(reader.read_u16()?);
            }
            15 => {
                npc_def.animation_data.rotate_left = Some(reader.read_u16()?);
            }
            16 => {
                npc_def.animation_data.rotate_right = Some(reader.read_u16()?);
            }
            17 => {
                npc_def.animation_data.walking = Some(reader.read_u16()?);
                npc_def.animation_data.rotate_180 = Some(reader.read_u16()?);
                npc_def.animation_data.rotate_90_right = Some(reader.read_u16()?);
                npc_def.animation_data.rotate_90_left = Some(reader.read_u16()?);
            }
            18 => {
                npc_def.category = reader.read_u16()?;
            }
            30..=34 => {
                npc_def.actions[opcode as usize - 30] = reader.read_string()?;
            }
            40 => {
                let len = reader.read_u8()?;
                for _ in 0..len {
                    npc_def.model_data.recolor_find.push(reader.read_u16()?);
                    npc_def.model_data.recolor_replace.push(reader.read_u16()?);
                }
            }
            41 => {
                let len = reader.read_u8()?;
                for _ in 0..len {
                    npc_def.model_data.retexture_find.push(reader.read_u16()?);
                    npc_def
                        .model_data
                        .retexture_replace
                        .push(reader.read_u16()?);
                }
            }
            60 => {
                let len = reader.read_u8()?;
                for _ in 0..len {
                    npc_def.model_data.chat_head_models.push(reader.read_u16()?);
                }
            }
            93 => npc_def.visible_on_minimap = true,
            95 => {
                npc_def.combat_level = Some(reader.read_u16()?);
            }
            97 => {
                npc_def.model_data.width_scale = reader.read_u16()?;
            }
            98 => {
                npc_def.model_data.height_scale = reader.read_u16()?;
            }
            99 => npc_def.model_data.render_priority = true,
            100 => {
                npc_def.model_data.ambient = reader.read_u8()?;
            }
            101 => {
                npc_def.model_data.contrast = reader.read_u8()?;
            }
            102 => {
                // npc_def.model_data.head_icon = Some(reader.read_u16()?);
                let bitfield = reader.read_u8()? as i32;
                let mut len: i32 = 0;
                let mut bitfield_clone = bitfield.clone();
                
                while(bitfield_clone != 0) {
                    bitfield_clone >>= 1;
                    len += 1;
                }
                for i in 0..len { 
					if ((bitfield & 1 << i) == 0)
					{
					}
					else
					{
                        // Correct length reads, not doing anything. TODO
						let _ = reader.read_smart()?;
						let _ = reader.read_smart_u16()? - 1;
					}
                }
            }
            103 => {
                npc_def.model_data.rotate_speed = reader.read_u16()?;
            }
            106 => {
                let varbit_id = reader.read_u16()?;
                npc_def.varbit_id = if varbit_id == std::u16::MAX {
                    None
                } else {
                    Some(varbit_id)
                };

                let varp_index = reader.read_u16()?;
                npc_def.varp_index = if varp_index == std::u16::MAX {
                    None
                } else {
                    Some(varp_index)
                };

                npc_def.configs = Vec::new();
                let len = reader.read_u8()?;
                for _ in 0..=len {
                    npc_def.configs.push(reader.read_u16()?);
                }
            }
            107 => npc_def.interactable = false,
            109 => npc_def.model_data.rotate_flag = false,
            111 => {
                npc_def.follower = true;
                npc_def.lowpriorityfollowerops = true;
            }
            114 => {
                npc_def.animation_data.running = Some(reader.read_u16()?);
            }
            115 => {
                npc_def.animation_data.running = Some(reader.read_u16()?);
                npc_def.animation_data.running_rotate_180 = Some(reader.read_u16()?);
                npc_def.animation_data.running_rotate_left = Some(reader.read_u16()?);
                npc_def.animation_data.running_rotate_right = Some(reader.read_u16()?);
            }
            116 => {
                npc_def.animation_data.crawling = Some(reader.read_u16()?);
            }
            117 => {
                npc_def.animation_data.crawling = Some(reader.read_u16()?);
                npc_def.animation_data.crawling_rotate_180 = Some(reader.read_u16()?);
                npc_def.animation_data.crawling_rotate_left = Some(reader.read_u16()?);
                npc_def.animation_data.crawling_rotate_right = Some(reader.read_u16()?);
            }
            118 => {
                let varbit_id = reader.read_u16()?;
                npc_def.varbit_id = if varbit_id == std::u16::MAX {
                    None
                } else {
                    Some(varbit_id)
                };

                let varp_index = reader.read_u16()?;
                npc_def.varp_index = if varp_index == std::u16::MAX {
                    None
                } else {
                    Some(varp_index)
                };

                // should append var at end
                let _var = reader.read_u16()?;

                npc_def.configs = Vec::new();
                let len = reader.read_u8()?;
                for _ in 0..=len {
                    npc_def.configs.push(reader.read_u16()?);
                }
            }
            122 => npc_def.follower = true,
            123 => npc_def.lowpriorityfollowerops = true,
            249 => {
                npc_def.params = util::read_parameters(reader)?;
            }
            unknown => eprintln!("Unknown opcode {} when parsing npc definition", unknown),
        }
    }

    Ok(npc_def)
}
