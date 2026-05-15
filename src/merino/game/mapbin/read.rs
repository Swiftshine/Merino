use std::io::{Cursor, Read};

use super::types::*;
use super::*;
use crate::merino::game::mapbin::Mapdata;
use anyhow::{Result, anyhow};
use byteorder::{BigEndian, ReadBytesExt};

impl Mapdata {
    pub fn read(bytes: &[u8]) -> Result<Self> {
        MapdataReader::new(bytes).read_level()
    }
}

/* Reader */

trait Readable {
    fn read(reader: &mut MapdataReader) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Default)]
struct MapdataReader<'a> {
    cursor: Cursor<&'a [u8]>,
    pub version: f32,
    pub object_types: Vec<String32>,
    pub item_types: Vec<String32>,
    pub collision_types: Vec<String32>,
    pub rect_types: Vec<String32>,
    pub enemy_types: Vec<String32>,
    pub unk_types_1: Vec<String32>,
}

impl<'a> MapdataReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            version: 0.0,
            ..Default::default()
        }
    }

    // pub fn position(&self) -> u64 {
    //     self.cursor.position()
    // }

    // pub fn align(&mut self, alignment: u64) {
    //     self.cursor
    //         .set_position(self.position().next_multiple_of(alignment));
    // }

    /* raw data */

    fn read_bytes(&mut self, num_bytes: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0u8; num_bytes];
        self.cursor.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    // pub fn read_u8(&mut self) -> Result<u8> {
    //     Ok(self.cursor.read_u8()?)
    // }

    fn read_u32(&mut self) -> Result<u32> {
        Ok(self.cursor.read_u32::<BigEndian>()?)
    }

    fn read_i32(&mut self) -> Result<i32> {
        Ok(self.cursor.read_i32::<BigEndian>()?)
    }

    fn read_f32(&mut self) -> Result<f32> {
        Ok(self.cursor.read_f32::<BigEndian>()?)
    }

    fn read_u64(&mut self) -> Result<u64> {
        Ok(self.cursor.read_u64::<BigEndian>()?)
    }

    fn read_string(&mut self, string_length: usize) -> Result<String> {
        let bytes = self.read_bytes(string_length)?;

        // find pos of null byte
        let string_len = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());

        Ok(String::from_utf8_lossy(&bytes[..string_len]).to_string())
    }

    fn read_object<T>(&mut self) -> Result<T>
    where
        T: Readable,
    {
        T::read(self)
    }

    fn read_at_version<T, F>(&mut self, min_version: f32, f: F) -> Result<Option<T>>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        if self.version >= min_version {
            f(self).map(Some)
        } else {
            Ok(None)
        }
    }

    fn read_below_version<T, F>(&mut self, max_version: f32, f: F) -> Result<Option<T>>
    where
        F: FnOnce(&mut Self) -> Result<T>,
    {
        if self.version < max_version {
            f(self).map(Some)
        } else {
            Ok(None)
        }
    }
    // custom

    /// Reads a u32 count and performs F that many times, returning a Result<Vec<T>>
    fn read_array<T, F>(&mut self, mut f: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let count = self.read_u32()?;

        let mut results = Vec::with_capacity(count as usize);

        for _ in 0..count {
            results.push(f(self)?);
        }

        Ok(results)
    }

    fn read_level(mut self) -> Result<Mapdata> {
        // read header

        let _filesize = self.read_u64()? as usize; // excluding the header
        let version = self.read_f32()?;
        self.version = version;

        // read strings

        self.object_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.item_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.collision_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.rect_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.enemy_types = self.read_array(|reader| reader.read_object::<String32>())?;
        self.unk_types_1 = self.read_array(|reader| reader.read_object::<String32>())?;
        let root = MapDataNode::read(&mut self)?;

        Ok(Mapdata {
            version,
            object_types: self.object_types,
            item_types: self.item_types,
            collision_types: self.collision_types,
            rect_types: self.rect_types,
            enemy_types: self.enemy_types,
            unk_types_1: self.unk_types_1,
            root,
        })
    }

    // accessors
    fn get_string_by_index(&self, list: &[String32], index: usize) -> String32 {
        // just give it a blank one
        list.get(index).cloned().unwrap_or_default()
    }

    fn read_object_type(&mut self) -> Result<String32> {
        let index = self.read_u32()? as usize;
        Ok(self.get_string_by_index(&self.object_types, index))
    }

    fn read_item_type(&mut self) -> Result<String32> {
        let index = self.read_u32()? as usize;
        Ok(self.get_string_by_index(&self.item_types, index))
    }

    fn read_collision_type(&mut self) -> Result<String32> {
        let index = self.read_u32()? as usize;
        Ok(self.get_string_by_index(&self.collision_types, index))
    }

    // pub fn read_unk_type_1_type(&mut self) -> Result<String32> {
    //     let index = self.read_u32()? as usize;
    //     self.get_string_by_index(&self.unk_types_1, index, "Terrain")
    // }
}

/* Readable impls */

impl Readable for Vec2f {
    fn read(reader: &mut MapdataReader) -> Result<Self>
    where
        Self: Sized,
    {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;

        Ok(Self { x, y })
    }
}

impl Readable for Vec3f {
    fn read(reader: &mut MapdataReader) -> Result<Self>
    where
        Self: Sized,
    {
        let x = reader.read_f32()?;
        let y = reader.read_f32()?;
        let z = reader.read_f32()?;

        Ok(Self { x, y, z })
    }
}

impl Readable for CollisionLine {
    fn read(reader: &mut MapdataReader) -> Result<Self>
    where
        Self: Sized,
    {
        let start = reader.read_object::<Vec2f>()?;
        let end = reader.read_object::<Vec2f>()?;
        let collision_normal = reader.read_object::<Vec2f>()?;

        Ok(Self {
            start,
            end,
            collision_normal,
        })
    }
}

impl<const N: usize> Readable for LimitedString<N> {
    fn read(reader: &mut MapdataReader) -> Result<Self> {
        Ok(Self(reader.read_string(N)?))
    }
}

impl<const N: usize> Readable for Params<N> {
    fn read(reader: &mut MapdataReader) -> Result<Self> {
        let mut int_values = [0i32; N];
        for i in 0..N {
            int_values[i] = reader.read_i32()?;
        }

        let mut float_values = [0.0f32; N];
        for i in 0..N {
            float_values[i] = reader.read_f32()?;
        }

        let mut string_values: [LimitedString<64>; N] = std::array::from_fn(|_| Default::default());

        for i in 0..N {
            string_values[i] = LimitedString::<64>::read(reader)?
        }

        Ok(Self {
            int_values,
            float_values,
            string_values,
        })
    }
}

impl Readable for MapDataNode {
    fn read(reader: &mut MapdataReader) -> Result<Self> {
        let node_type_raw = reader.read_u32()?;
        let node_type = MapNodeType::from_repr(node_type_raw)
            .ok_or_else(|| anyhow!("invalid node type, found {node_type_raw}"))?;

        let node_data = match node_type {
            MapNodeType::MapSet => NodeData::MapSet {
                unk1: reader.read_at_version(4.70, |r| r.read_i32())?,
                bounds_start: reader.read_object::<Vec2f>()?,
                bounds_end: reader.read_object::<Vec2f>()?,
            },

            MapNodeType::MapPolySet => NodeData::MapPolySet {
                line: reader.read_object::<CollisionLine>()?,
                collision_type: reader.read_collision_type()?,
                unk3: reader.read_u32()?,
            },

            MapNodeType::MapObjSet => NodeData::MapObjSet {
                name: reader.read_object_type()?,
                position: reader.read_object::<Vec3f>()?,
                unk3: reader.read_f32()?,
                unk4: reader.read_object::<Vec2f>()?,
                unk5: reader.read_object::<String32>()?,
                unk6: reader.read_at_version(4.43, |r| r.read_i32())?,
                unk7: reader.read_at_version(4.44, |r| r.read_object::<String32>())?,
                unk8: reader.read_object::<Vec2f>()?,
                unk9: reader.read_object::<Vec2f>()?,
                unk10: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk11: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk12: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk13: reader.read_at_version(4.71, |r| r.read_i32())?,
                params: reader.read_object::<Params<5>>()?,
                unk14: reader.read_at_version(4.50, |r| {
                    let mut outer =
                        std::array::from_fn(|_| [Default::default(), Default::default()]);
                    for i in 0..5 {
                        outer[i] = [
                            r.read_object::<LimitedString<32>>()?,
                            r.read_object::<LimitedString<32>>()?,
                        ];
                    }
                    Ok(outer)
                })?,
            },

            MapNodeType::MapItemSet => NodeData::MapItemSet {
                name: reader.read_item_type()?,
                position: reader.read_object::<Vec3f>()?,
                unk3: reader.read_f32()?,
                unk4: reader.read_object::<Vec2f>()?,
                unk5: reader.read_object::<String32>()?,
                unk6: reader.read_at_version(4.43, |r| r.read_i32())?,
                unk7: reader.read_at_version(4.44, |r| r.read_object::<String32>())?,
                unk8: reader.read_object::<Vec2f>()?,
                unk9: reader.read_object::<Vec2f>()?,
                unk10: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk11: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk12: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk13: reader.read_at_version(4.71, |r| r.read_i32())?,
                params: reader.read_object::<Params<5>>()?,
            },

            MapNodeType::MapEnemySet => NodeData::MapEnemySet {
                name: reader.read_object::<String32>()?,
                direction: reader.read_object::<String16>()?,
                orientation: reader.read_object::<String16>()?,
                position: reader.read_object::<Vec3f>()?,
                unk7: reader.read_at_version(4.45, |r| r.read_object::<String32>())?,
                unk8: reader
                    .read_below_version(4.43, |r| r.read_string(16))?
                    .map(Into::into),
                unk9: reader
                    .read_below_version(4.43, |r| r.read_string(16))?
                    .map(Into::into),
                unk10: reader.read_below_version(4.43, |r| r.read_object::<String32>())?,
                unk11: reader.read_below_version(4.43, |r| r.read_i32())?,
                unk12: reader.read_below_version(4.43, |r| r.read_i32())?,
                unk13: reader.read_i32()?,
                unk14: reader.read_at_version(4.42, |r| r.read_i32())?,
                unk15: reader.read_at_version(4.44, |r| r.read_object::<String32>())?,
                unk16: reader.read_f32()?,
                unk17: reader.read_f32()?,
                unk18: reader.read_f32()?,
                unk19: reader.read_f32()?,
                unk20: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk21: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk22: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk23: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk24: reader.read_at_version(4.72, |r| r.read_i32())?,
                params: reader.read_object::<Params<5>>()?,
            },

            MapNodeType::MapLocator => NodeData::MapLocator {
                name: reader.read_object::<String64>()?,
                position: reader.read_object::<Vec3f>()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapPath => NodeData::MapPath {
                name: reader.read_object::<String64>()?,
                points: reader.read_array(|r| r.read_object::<Vec2f>())?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapRect => NodeData::MapRect {
                name: reader.read_object::<String64>()?,
                bounds_start: reader.read_object::<Vec2f>()?,
                bounds_end: reader.read_object::<Vec2f>()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapCircle => NodeData::MapCircle {
                name: reader.read_object::<String64>()?,
                position: reader.read_object::<Vec2f>()?,
                radius: reader.read_f32()?,
                params: reader.read_object::<Params<3>>()?,
            },

            MapNodeType::MapTerrain => NodeData::MapTerrain {
                collision_type: reader.read_collision_type()?,
                position: reader.read_object::<Vec3f>()?,
                unk3: reader.read_at_version(4.43, |r| r.read_i32())?,
                unk4: reader.read_at_version(4.44, |r| r.read_object::<String32>())?,
                unk5: reader.read_f32()?,
                unk6: reader.read_f32()?,
                unk7: reader.read_f32()?,
                unk8: reader.read_f32()?,
                unk9: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk10: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk11: reader.read_at_version(4.71, |r| r.read_i32())?,
                unk12: reader.read_at_version(4.71, |r| r.read_i32())?,
                lines: reader.read_array(|r| r.read_object::<CollisionLine>())?,
                params: reader.read_object::<Params<3>>()?,
                unk15: reader.read_at_version(4.6, |r| {
                    let mut outer = std::array::from_fn(|_| Default::default());
                    for i in 0..3 {
                        outer[i] = [r.read_object::<String32>()?, r.read_object::<String32>()?];
                    }
                    Ok(outer)
                })?,
            },
        };

        let flags = reader.read_u32()?;

        // helper to read a list of sub-nodes if flag present
        let mut read_child_node = |flag: MapNodeFlag| -> Result<Option<Vec<MapDataNode>>> {
            if (flags & flag as u32) != 0 {
                let nodes = reader.read_array(|r| Self::read(r))?;
                Ok(Some(nodes))
            } else {
                Ok(None)
            }
        };

        Ok(MapDataNode {
            node_type,
            node_data,
            children_mappolyset: read_child_node(MapNodeFlag::MapPolySet)?,
            children_mapobjset: read_child_node(MapNodeFlag::MapObjSet)?,
            children_mapitemset: read_child_node(MapNodeFlag::MapItemSet)?,
            children_mapenemyset: read_child_node(MapNodeFlag::MapEnemySet)?,
            children_maplocator: read_child_node(MapNodeFlag::MapLocator)?,
            children_mappath: read_child_node(MapNodeFlag::MapPath)?,
            children_maprect: read_child_node(MapNodeFlag::MapRect)?,
            children_mapcircle: read_child_node(MapNodeFlag::MapCircle)?,
            children_mapterrain: read_child_node(MapNodeFlag::MapTerrain)?,
        })
    }
}
