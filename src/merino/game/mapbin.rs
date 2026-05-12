#[allow(unused_variables)] // todo! get rid of this when it's no longer needed.

mod read;
mod types;
mod write;

use types::*;

use strum::{AsRefStr, Display, EnumString, FromRepr};
use enum_map::Enum;

#[derive(
    FromRepr, Debug, Default, Display, AsRefStr, Copy, Clone, EnumString, PartialEq, Enum, Hash, Eq,
)]
#[repr(u32)]
pub enum MapNodeType {
    #[default]
    MapSet = 0,
    MapPolySet = 1,
    MapObjSet = 2,
    MapItemSet = 3,
    MapEnemySet = 4,
    MapLocator = 5,
    MapPath = 6,
    MapRect = 7,
    MapCircle = 8,
    MapTerrain = 9,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum MapNodeFlag {
    MapPolySet = 0x1,
    MapObjSet = 0x2,
    MapItemSet = 0x4,
    MapEnemySet = 0x8,
    MapLocator = 0x10,
    MapPath = 0x20,
    MapRect = 0x40,
    MapCircle = 0x80,
    MapTerrain = 0x100,
}

#[derive(Default)]
pub struct Mapdata {
    pub version: f32,
    pub object_types: Vec<String32>,
    pub item_types: Vec<String32>,
    pub collision_types: Vec<String32>,
    pub rect_types: Vec<String32>,
    pub enemy_types: Vec<String32>,
    pub unk_types_1: Vec<String32>,
    pub root: MapDataNode,
}

#[derive(Debug, Default)]
pub struct MapDataNode {
    pub node_type: MapNodeType,
    pub node_data: NodeData,
    pub children_mappolyset: Option<Vec<MapDataNode>>, // MapPolySet
    pub children_mapobjset: Option<Vec<MapDataNode>>,  // MapObjSet
    pub children_mapitemset: Option<Vec<MapDataNode>>, // MapItemSet
    pub children_mapenemyset: Option<Vec<MapDataNode>>, // MapEnemySet
    pub children_maplocator: Option<Vec<MapDataNode>>, // MapLocator
    pub children_mappath: Option<Vec<MapDataNode>>,    // MapPath
    pub children_maprect: Option<Vec<MapDataNode>>,    // MapRect
    pub children_mapcircle: Option<Vec<MapDataNode>>,  // MapCircle
    pub children_mapterrain: Option<Vec<MapDataNode>>, // MapTerrain
}


#[derive(Debug, Default)]
pub enum NodeData {
    #[default]
    None,
    MapSet {
        unk1: Option<i32>, // >= 4.70
        bounds_start: Vec2f,
        bounds_end: Vec2f,
    },

    MapPolySet {
        start: Vec2f,
        end: Vec2f,
        collision_normal: Vec2f,
        collision_type: String32,
        unk3: u32,
    },

    MapObjSet {
        name: String32,
        position: Vec3f,
        unk3: f32,
        unk4: Vec2f,
        unk5: String32,
        unk6: Option<i32>,      // >= 4.43
        unk7: Option<String32>, // >= 4.44
        unk8: Vec2f,
        unk9: Vec2f,
        unk10: Option<i32>, // >= 4.71
        unk11: Option<i32>, // >= 4.71
        unk12: Option<i32>, // >= 4.71
        unk13: Option<i32>, // >= 4.71
        params: Params<5>,
        unk14: Option<[[String32; 2]; 5]>, // >= 4.50
    },

    MapItemSet {
        name: String32,
        position: Vec3f,
        unk3: f32,
        unk4: Vec2f,
        unk5: String32,
        unk6: Option<i32>,      // version >= 4.43
        unk7: Option<String32>, // version >= 4.44
        unk8: Vec2f,
        unk9: Vec2f,
        unk10: Option<i32>, // version >= 4.71
        unk11: Option<i32>, // version >= 4.71
        unk12: Option<i32>, // version >= 4.71
        unk13: Option<i32>, // version >= 4.71
        params: Params<5>,
    },

    MapEnemySet {
        name: String32,
        direction: String16,
        orientation: String16,
        position: Vec3f,
        unk7: Option<String32>,  // version >= 4.45
        unk8: Option<String16>,  // version < 4.43
        unk9: Option<String16>,  // version < 4.43
        unk10: Option<String32>, // version < 4.43
        unk11: Option<i32>,      // version < 4.43
        unk12: Option<i32>,      // version < 4.43
        unk13: i32,
        unk14: Option<i32>,      // version >= 4.42
        unk15: Option<String32>, // version >= 4.44
        unk16: f32,
        unk17: f32,
        unk18: f32,
        unk19: f32,
        unk20: Option<i32>, // version >= 4.71
        unk21: Option<i32>, // version >= 4.71
        unk22: Option<i32>, // version >= 4.71
        unk23: Option<i32>, // version >= 4.71
        unk24: Option<i32>, // version >= 4.72
        params: Params<5>,
    },

    MapLocator {
        name: String64,
        position: Vec3f,
        params: Params<3>,
    },

    MapPath {
        name: String64,
        points: Vec<Vec2f>,
        params: Params<3>,
    },

    MapRect {
        name: String64,
        bounds_start: Vec2f,
        bounds_end: Vec2f,
        params: Params<3>,
    },

    MapCircle {
        name: String64,
        position: Vec2f,
        radius: f32,
        params: Params<3>,
    },

    MapTerrain {
        collision_type: String32,
        position: Vec3f,
        unk3: Option<i32>,      // version >= 4.43
        unk4: Option<String32>, // version >= 4.44
        unk5: f32,
        unk6: f32,
        unk7: f32,
        unk8: f32,
        unk9: Option<i32>,  // version >= 4.71
        unk10: Option<i32>, // version >= 4.71
        unk11: Option<i32>, // version >= 4.71
        unk12: Option<i32>, // version >= 4.71
        unk13: Vec<[Vec2f; 3]>,
        params: Params<3>,
        unk15: Option<[[String32; 2]; 3]>, // version >= 4.6
    },
}
