use std::{
	io,
	io::BufReader,
	collections::HashMap,
};

use crate::ReadExt;

#[derive(Clone, Debug, Default)]
pub struct NpcDefinition {
    pub id: u16,
    pub name: String,
    pub model_data: NpcModelData,
    pub animation_data: NpcAnimationData,
    pub size: usize,
    pub actions: [String; 5],
    pub visible_on_minimap: bool,
    pub combat_level: u16,
    pub configs: Vec<u16>,
    pub varbit_id: Option<u16>,
    pub varp_index: Option<u16>,
    pub interactable: bool,
    pub pet: bool,
    pub params: HashMap<u32, String>,
}

#[derive(Clone, Debug, Default)]
#[doc(hidden)]
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
    pub head_icon: u16,
    pub rotate_speed: u16,
    pub rotate_flag: bool,
}

#[derive(Clone, Debug, Default)]
#[doc(hidden)]
pub struct NpcAnimationData {
    pub standing: u16,
    pub walking: u16,
    pub rotate_left: u16,
    pub rotate_right: u16,
    pub rotate_180: u16,
    pub rotate_90_left: u16,
    pub rotate_90_right: u16,
}

impl NpcDefinition {
	#[inline]
	#[doc(hidden)]
	pub fn new(id: u16, buffer: &[u8]) -> io::Result<Self> {
		let mut reader = BufReader::new(&buffer[..]);
		let npc_def = decode_buffer(id, &mut reader)?;

		Ok(npc_def)
	}
}

fn decode_buffer(id: u16, reader: &mut BufReader<&[u8]>) -> io::Result<NpcDefinition> {
	let mut npc_def = NpcDefinition::default();
    npc_def.id = id;

	loop {
        let opcode = reader.read_u8()?;

		match opcode {
			0 => break,
			1 => {
				let len = reader.read_u8()?;
				for _ in 0..len {
                    npc_def.model_data.models.push(reader.read_u16()?);
                }
			},
			2 => { npc_def.name = reader.read_string()?; },
            12 => { npc_def.size = reader.read_u8()? as usize; },
            13 => { npc_def.animation_data.standing = reader.read_u16()?; },
            14 => { npc_def.animation_data.walking = reader.read_u16()?; },
            15 => { npc_def.animation_data.rotate_left = reader.read_u16()?; },
            16 => { npc_def.animation_data.rotate_right = reader.read_u16()?; },
            17 => { 
                npc_def.animation_data.walking = reader.read_u16()?;
                npc_def.animation_data.rotate_180 = reader.read_u16()?;
                npc_def.animation_data.rotate_90_right = reader.read_u16()?;
                npc_def.animation_data.rotate_90_left = reader.read_u16()?;
             },
			30..=34 => { npc_def.actions[opcode as usize - 30] = reader.read_string()?; },
			40 => {
				let len = reader.read_u8()?;
				for _ in 0..len {
					npc_def.model_data.recolor_find.push(reader.read_u16()?);
					npc_def.model_data.recolor_replace.push(reader.read_u16()?);
				}
			},
			41 => {
				let len = reader.read_u8()?;
				for _ in 0..len {
					npc_def.model_data.retexture_find.push(reader.read_u16()?);
					npc_def.model_data.retexture_replace.push(reader.read_u16()?);
				}
			},
			60 => {
                let len = reader.read_u8()?;
				for _ in 0..len {
					npc_def.model_data.chat_head_models.push(reader.read_u16()?);
				}
            },
			93 => npc_def.visible_on_minimap = true,
			95 => { npc_def.combat_level = reader.read_u16()?; },
			97 => { npc_def.model_data.width_scale = reader.read_u16()?; },
            98 => { npc_def.model_data.height_scale = reader.read_u16()?; },
            99 => npc_def.model_data.render_priority = true,
            100 => { npc_def.model_data.ambient = reader.read_u8()?; },
            101 => { npc_def.model_data.contrast = reader.read_u8()?; },
            102 => { npc_def.model_data.head_icon = reader.read_u16()?; },
            103 => { npc_def.model_data.rotate_speed = reader.read_u16()?; },
            106 => { 
                let varbit_id = reader.read_u16()?;
                npc_def.varbit_id = if varbit_id == std::u16::MAX { None } else { Some(varbit_id) };

                let varp_index = reader.read_u16()?;
                npc_def.varp_index = if varp_index == std::u16::MAX { None } else { Some(varp_index) };

                npc_def.configs = Vec::new();
                let len = reader.read_u8()?;
				for _ in 0..=len {
					npc_def.configs.push(reader.read_u16()?);
				}
            },
            107 => npc_def.interactable = false,
			109 => npc_def.model_data.rotate_flag = false,
            111 => npc_def.pet = true,
            118 => {
                let varbit_id = reader.read_u16()?;
                npc_def.varbit_id = if varbit_id == std::u16::MAX { None } else { Some(varbit_id) };

                let varp_index = reader.read_u16()?;
                npc_def.varp_index = if varp_index == std::u16::MAX { None } else { Some(varp_index) };

                // should append var at end
                let _var = reader.read_u16()?;

                npc_def.configs = Vec::new();
                let len = reader.read_u8()?;
				for _ in 0..=len {
					npc_def.configs.push(reader.read_u16()?);
				}
            },
			249 => {
				let len = reader.read_u8()?;

				for _ in 0..len {
					let is_string = reader.read_u8()? == 1;
					let key = reader.read_u24()?;
					
					let value = if is_string {
						reader.read_string()?
					} else {
						reader.read_i32()?.to_string()
					};

					npc_def.params.insert(key, value);
				}
			}
			_ => { println!("opcode: {}", opcode); unreachable!() }
		}
	}

	Ok(npc_def)
}