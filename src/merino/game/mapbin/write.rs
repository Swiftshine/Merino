use crate::merino::game::mapbin::CollisionLine;
use crate::merino::game::mapbin::MapDataNode;
use crate::merino::game::mapbin::MapNodeFlag;
use crate::merino::game::mapbin::Mapdata;
use crate::merino::game::mapbin::NodeData;
use crate::merino::game::mapbin::types::*;
use anyhow::Result;
use anyhow::anyhow;
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

const PLACEHOLDER_VALUE: u32 = 0xDEADCAFE;

pub trait Writable {
    fn write(&self, writer: &mut MapdataWriter, version: f32) -> Result<()>;
}

pub struct MapdataWriter {
    pub buffer: Vec<u8>,
}

impl MapdataWriter {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    // primitives

    pub fn write_i32(&mut self, val: i32) -> Result<()> {
        self.buffer.write_i32::<BigEndian>(val)?;
        Ok(())
    }

    fn write_u32(&mut self, val: u32) -> Result<()> {
        self.buffer.write_u32::<BigEndian>(val)?;
        Ok(())
    }

    pub fn write_f32(&mut self, val: f32) -> Result<()> {
        self.buffer.write_f32::<BigEndian>(val)?;
        Ok(())
    }

    pub fn write_string(&mut self, string: &String, size: usize) -> Result<()> {
        let bytes = string.as_bytes();

        let len = bytes.len().min(size);

        self.buffer.write_all(&bytes[..len])?;

        for _ in 0..(size - len) {
            self.buffer.write_u8(0)?;
        }

        Ok(())
    }

    // util
    fn get_index_of(&self, list: &[String32], value: &String32, label: &str) -> Result<u32> {
        list.iter()
            .position(|s| s == value)
            .map(|i| i as u32)
            .ok_or_else(|| anyhow!("{} '{}' not found in string table", label, value))
    }

    fn write_at_version<T, F>(
        &mut self,
        version: f32,
        min: f32,
        val: &Option<T>,
        f: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self, &T) -> Result<()>,
    {
        if version >= min {
            if let Some(v) = val {
                f(self, v)?;
            } else {
                return Err(anyhow!("Missing required versioned field (>= {})", min));
            }
        }
        Ok(())
    }

    fn write_below_version<T, F>(
        &mut self,
        version: f32,
        max: f32,
        val: &Option<T>,
        f: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Self, &T) -> Result<()>,
    {
        if version < max {
            if let Some(v) = val {
                f(self, v)?;
            } else {
                return Err(anyhow!("Missing required versioned field (< {})", max));
            }
        }
        Ok(())
    }
    // custom

    fn write_level(mut self, mapdata: &Mapdata) -> Result<Vec<u8>> {
        // filesize
        self.write_u32(PLACEHOLDER_VALUE)?;
        self.write_u32(PLACEHOLDER_VALUE)?;

        // version
        self.write_f32(mapdata.version)?;

        // strings

        let mut write_string32_array = |array: &Vec<String32>| -> Result<()> {
            let count = array.len();
            self.write_u32(count as u32)?;

            for string in array.iter() {
                string.write(&mut self, mapdata.version)?;
            }

            Ok(())
        };

        let string_tables = [
            &mapdata.object_types,
            &mapdata.item_types,
            &mapdata.collision_types,
            &mapdata.rect_types,
            &mapdata.enemy_types,
            &mapdata.unk_types_1,
        ];

        for table in string_tables {
            write_string32_array(table)?;
        }

        // nodes
        mapdata.root.write(&mut self, &mapdata, mapdata.version)?;

        // pad to 0x20 bytes
        let len = self.buffer.len();
        for _ in 0..(len.next_multiple_of(0x20) - len) {
            self.buffer.push(0);
        }

        // write size
        let total_size = self.buffer.len() as u64;
        let mut real_size_slice = &mut self.buffer[0..8];
        real_size_slice.write_u64::<BigEndian>(total_size - 0xC)?; // exclude header

        Ok(self.buffer)
    }
}

impl Mapdata {
    pub fn write(&self) -> Result<Vec<u8>> {
        MapdataWriter::new().write_level(self)
    }
}

/* Writable impls */

impl Writable for Vec2f {
    fn write(&self, writer: &mut MapdataWriter, _: f32) -> Result<()> {
        writer.write_f32(self.x)?;
        writer.write_f32(self.y)?;
        Ok(())
    }
}

impl Writable for Vec3f {
    fn write(&self, writer: &mut MapdataWriter, _: f32) -> Result<()> {
        writer.write_f32(self.x)?;
        writer.write_f32(self.y)?;
        writer.write_f32(self.z)?;
        Ok(())
    }
}

impl<const N: usize> Writable for LimitedString<N> {
    fn write(&self, writer: &mut MapdataWriter, _: f32) -> Result<()> {
        writer.write_string(&self.0, N)
    }
}

impl<const N: usize> Writable for Params<N> {
    fn write(&self, writer: &mut MapdataWriter, version: f32) -> Result<()> {
        for v in self.int_values {
            writer.write_i32(v)?;
        }
        for v in self.float_values {
            writer.write_f32(v)?;
        }
        for v in &self.string_values {
            v.write(writer, version)?;
        }
        Ok(())
    }
}

impl Writable for CollisionLine {
    fn write(&self, writer: &mut MapdataWriter, version: f32) -> Result<()> {
        self.start.write(writer, version)?;
        self.end.write(writer, version)?;
        self.collision_normal.write(writer, version)?;
        Ok(())
    }
}

impl MapDataNode {
    fn write(&self, writer: &mut MapdataWriter, mapdata: &Mapdata, version: f32) -> Result<()> {
        // type
        writer.write_u32(self.node_type as u32)?;

        // write data
        match &self.node_data {
            NodeData::MapSet {
                unk1,
                bounds_start,
                bounds_end,
            } => {
                writer.write_at_version(version, 4.70, unk1, |w, v| w.write_i32(*v))?;
                bounds_start.write(writer, version)?;
                bounds_end.write(writer, version)?;
            }

            NodeData::MapPolySet {
                line,
                collision_type,
                unk3,
            } => {
                line.write(writer, version)?;
                let index =
                    writer.get_index_of(&mapdata.collision_types, collision_type, "Collision")?;
                writer.write_u32(index)?;
                writer.write_u32(*unk3)?;
            }

            NodeData::MapObjSet {
                name,
                position,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                params,
                unk14,
            } => {
                let index = writer.get_index_of(&mapdata.object_types, name, "Object")?;
                writer.write_u32(index)?;
                position.write(writer, version)?;
                writer.write_f32(*unk3)?;
                unk4.write(writer, version)?;
                unk5.write(writer, version)?;
                writer.write_at_version(version, 4.43, unk6, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.44, unk7, |w, v| v.write(w, version))?;
                unk8.write(writer, version)?;
                unk9.write(writer, version)?;
                writer.write_at_version(version, 4.71, unk10, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk11, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk12, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk13, |w, v| w.write_i32(*v))?;
                params.write(writer, version)?;
                writer.write_at_version(version, 4.50, unk14, |w, v| {
                    for pair in v {
                        pair[0].write(w, version)?;
                        pair[1].write(w, version)?;
                    }
                    Ok(())
                })?;
            }

            NodeData::MapItemSet {
                name,
                position,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                params,
            } => {
                let index = writer.get_index_of(&mapdata.item_types, name, "Item")?;
                writer.write_u32(index)?;
                position.write(writer, version)?;
                writer.write_f32(*unk3)?;
                unk4.write(writer, version)?;
                unk5.write(writer, version)?;
                writer.write_at_version(version, 4.43, unk6, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.44, unk7, |w, v| v.write(w, version))?;
                unk8.write(writer, version)?;
                unk9.write(writer, version)?;
                writer.write_at_version(version, 4.71, unk10, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk11, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk12, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk13, |w, v| w.write_i32(*v))?;
                params.write(writer, version)?;
            }

            NodeData::MapEnemySet {
                name,
                direction,
                orientation,
                position,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                unk13,
                unk14,
                unk15,
                unk16,
                unk17,
                unk18,
                unk19,
                unk20,
                unk21,
                unk22,
                unk23,
                unk24,
                params,
            } => {
                name.write(writer, version)?;
                direction.write(writer, version)?;
                orientation.write(writer, version)?;
                position.write(writer, version)?;
                writer.write_at_version(version, 4.45, unk7, |w, v| v.write(w, version))?;
                writer.write_below_version(version, 4.43, unk8, |w, v| v.write(w, version))?;
                writer.write_below_version(version, 4.43, unk9, |w, v| v.write(w, version))?;
                writer.write_below_version(version, 4.43, unk10, |w, v| v.write(w, version))?;
                writer.write_below_version(version, 4.43, unk11, |w, v| w.write_i32(*v))?;
                writer.write_below_version(version, 4.43, unk12, |w, v| w.write_i32(*v))?;
                writer.write_i32(*unk13)?;
                writer.write_at_version(version, 4.42, unk14, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.44, unk15, |w, v| v.write(w, version))?;
                writer.write_f32(*unk16)?;
                writer.write_f32(*unk17)?;
                writer.write_f32(*unk18)?;
                writer.write_f32(*unk19)?;
                writer.write_at_version(version, 4.71, unk20, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk21, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk22, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk23, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.72, unk24, |w, v| w.write_i32(*v))?;
                params.write(writer, version)?;
            }

            NodeData::MapLocator {
                name,
                position,
                params,
            } => {
                name.write(writer, version)?;
                position.write(writer, version)?;
                params.write(writer, version)?;
            }

            NodeData::MapPath {
                name,
                points,
                params,
            } => {
                name.write(writer, version)?;
                writer.write_u32(points.len() as u32)?;
                for p in points {
                    p.write(writer, version)?;
                }
                params.write(writer, version)?;
            }

            NodeData::MapRect {
                name,
                bounds_start,
                bounds_end,
                params,
            } => {
                name.write(writer, version)?;
                bounds_start.write(writer, version)?;
                bounds_end.write(writer, version)?;
                params.write(writer, version)?;
            }

            NodeData::MapCircle {
                name,
                position,
                radius,
                params,
            } => {
                name.write(writer, version)?;
                position.write(writer, version)?;
                writer.write_f32(*radius)?;
                params.write(writer, version)?;
            }

            NodeData::MapTerrain {
                collision_type,
                position,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
                unk8,
                unk9,
                unk10,
                unk11,
                unk12,
                lines,
                params,
                unk15,
            } => {
                let index =
                    writer.get_index_of(&mapdata.collision_types, collision_type, "Collision")?;
                writer.write_u32(index)?;
                position.write(writer, version)?;
                writer.write_at_version(version, 4.43, unk3, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.44, unk4, |w, v| v.write(w, version))?;
                writer.write_f32(*unk5)?;
                writer.write_f32(*unk6)?;
                writer.write_f32(*unk7)?;
                writer.write_f32(*unk8)?;
                writer.write_at_version(version, 4.71, unk9, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk10, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk11, |w, v| w.write_i32(*v))?;
                writer.write_at_version(version, 4.71, unk12, |w, v| w.write_i32(*v))?;
                writer.write_u32(lines.len() as u32)?;
                for line in lines.iter() {
                    line.write(writer, version)?;
                }
                params.write(writer, version)?;
                writer.write_at_version(version, 4.6, unk15, |w, v| {
                    for pair in v {
                        pair[0].write(w, version)?;
                        pair[1].write(w, version)?;
                    }
                    Ok(())
                })?;
            }

            NodeData::None => unreachable!(),
        }

        let mut flags = 0u32;
        let child_refs = [
            (&self.children_mappolyset, MapNodeFlag::MapPolySet),
            (&self.children_mapobjset, MapNodeFlag::MapObjSet),
            (&self.children_mapitemset, MapNodeFlag::MapItemSet),
            (&self.children_mapenemyset, MapNodeFlag::MapEnemySet),
            (&self.children_maplocator, MapNodeFlag::MapLocator),
            (&self.children_mappath, MapNodeFlag::MapPath),
            (&self.children_maprect, MapNodeFlag::MapRect),
            (&self.children_mapcircle, MapNodeFlag::MapCircle),
            (&self.children_mapterrain, MapNodeFlag::MapTerrain),
        ];

        for (sub, flag) in child_refs {
            if sub.is_some() {
                flags |= flag as u32;
            }
        }

        writer.write_u32(flags)?;

        for (sub, _) in child_refs.iter() {
            if let Some(nodes) = sub {
                writer.write_u32(nodes.len() as u32)?;
                for n in nodes {
                    n.write(writer, mapdata, version)?;
                }
            }
        }

        Ok(())
    }
}
