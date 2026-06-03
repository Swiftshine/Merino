mod read;
pub(crate) mod types;
mod write;

use enum_map::Enum;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::{AsRefStr, Display, EnumString, FromRepr};
use types::*;

#[derive(
    FromRepr,
    Debug,
    Default,
    Display,
    AsRefStr,
    Copy,
    Clone,
    EnumString,
    PartialEq,
    Enum,
    Hash,
    Eq,
    Serialize,
    Deserialize,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum NodeChildType {
    MapPolySet,
    MapObjSet,
    MapItemSet,
    MapEnemySet,
    MapLocator,
    MapPath,
    MapRect,
    MapCircle,
    MapTerrain,
}

impl From<NodeChildType> for MapNodeType {
    fn from(value: NodeChildType) -> Self {
        match value {
            NodeChildType::MapPolySet => Self::MapPolySet,
            NodeChildType::MapObjSet => Self::MapObjSet,
            NodeChildType::MapItemSet => Self::MapItemSet,
            NodeChildType::MapEnemySet => Self::MapEnemySet,
            NodeChildType::MapLocator => Self::MapLocator,
            NodeChildType::MapPath => Self::MapPath,
            NodeChildType::MapRect => Self::MapRect,
            NodeChildType::MapCircle => Self::MapCircle,
            NodeChildType::MapTerrain => Self::MapTerrain,
        }
    }
}

impl TryFrom<MapNodeType> for NodeChildType {
    type Error = &'static str;

    fn try_from(value: MapNodeType) -> Result<Self, Self::Error> {
        match value {
            MapNodeType::MapPolySet => Ok(Self::MapPolySet),
            MapNodeType::MapObjSet => Ok(Self::MapObjSet),
            MapNodeType::MapItemSet => Ok(Self::MapItemSet),
            MapNodeType::MapEnemySet => Ok(Self::MapEnemySet),
            MapNodeType::MapLocator => Ok(Self::MapLocator),
            MapNodeType::MapPath => Ok(Self::MapPath),
            MapNodeType::MapRect => Ok(Self::MapRect),
            MapNodeType::MapCircle => Ok(Self::MapCircle),
            MapNodeType::MapTerrain => Ok(Self::MapTerrain),
            MapNodeType::MapSet => Err("Cannot convert MapSet"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Clone, Copy)]
pub struct NodeStep {
    node_type: NodeChildType,
    index: usize,
}

impl NodeStep {
    pub fn new(node_type: NodeChildType, index: usize) -> Self {
        Self { node_type, index }
    }

    pub fn node_type(&self) -> NodeChildType {
        self.node_type
    }

    // pub fn node_index(&self) -> usize {
    //     self.index
    // }
}

/// A path to any given node in the tree.
/// This is indicated in sequential traversal order.
/// e.g. ```[[MapPolySet, 0], [MapObjSet, 0], [MapItemSet, 1]]``` would be:
/// ```MapPolySet[0].MapObjSet[0].MapItemSet[1]```.
/// A path to root is an empty vec.
#[derive(Debug, Hash, PartialEq, Clone)]
pub struct NodePath(Vec<NodeStep>);

impl NodePath {
    pub fn push(&mut self, step: NodeStep) {
        self.0.push(step);
    }

    pub fn pop(&mut self) -> Option<NodeStep> {
        self.0.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &NodeStep> {
        self.0.iter()
    }

    pub const fn root() -> Self {
        Self(Vec::new())
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn parent(&self) -> Self {
        let mut prev = self.clone();
        prev.pop();

        prev
    }

    pub fn get_step(&self) -> Option<NodeStep> {
        self.0.last().copied()
    }

    pub fn with_step(&self, step: NodeStep) -> Self {
        let mut path = self.clone();
        path.push(step);

        path
    }

    pub fn is_descendant_of(&self, ancestor: &Self) -> bool {
        self.0.starts_with(&ancestor.0)
    }
}

#[derive(Default, Debug)]
pub struct CollisionLine {
    pub start: Vec2f,
    pub end: Vec2f,
    pub collision_normal: Vec2f,
}

impl CollisionLine {
    pub fn calculate_collision_normal(&mut self) {
        let direction = (self.end.x - self.start.x, self.end.y - self.start.y);

        let magnitude = f32::sqrt(direction.0.powf(2.0) + direction.1.powf(2.0));

        let normalized = (direction.0 / magnitude, direction.1 / magnitude);

        self.collision_normal.x = -normalized.1;
        self.collision_normal.y = normalized.0;
    }
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

impl Mapdata {
    pub fn get_node_at_path_mut(&mut self, path: &NodePath) -> Option<&mut MapDataNode> {
        let mut current = &mut self.root;

        for step in path.iter() {
            let vec = match step.node_type {
                NodeChildType::MapPolySet => &mut current.children_mappolyset,
                NodeChildType::MapObjSet => &mut current.children_mapobjset,
                NodeChildType::MapItemSet => &mut current.children_mapitemset,
                NodeChildType::MapEnemySet => &mut current.children_mapenemyset,
                NodeChildType::MapLocator => &mut current.children_maplocator,
                NodeChildType::MapPath => &mut current.children_mappath,
                NodeChildType::MapRect => &mut current.children_maprect,
                NodeChildType::MapCircle => &mut current.children_mapcircle,
                NodeChildType::MapTerrain => &mut current.children_mapterrain,
            };

            current = vec.as_mut()?.get_mut(step.index)?;
        }

        Some(current)
    }

    pub fn get_node_at_path(&self, path: &NodePath) -> Option<&MapDataNode> {
        let mut current = &self.root;

        for step in path.iter() {
            let vec = match step.node_type {
                NodeChildType::MapPolySet => &current.children_mappolyset,
                NodeChildType::MapObjSet => &current.children_mapobjset,
                NodeChildType::MapItemSet => &current.children_mapitemset,
                NodeChildType::MapEnemySet => &current.children_mapenemyset,
                NodeChildType::MapLocator => &current.children_maplocator,
                NodeChildType::MapPath => &current.children_mappath,
                NodeChildType::MapRect => &current.children_maprect,
                NodeChildType::MapCircle => &current.children_mapcircle,
                NodeChildType::MapTerrain => &current.children_mapterrain,
            };

            current = vec.as_ref()?.get(step.index)?;
        }

        Some(current)
    }

    /// The path given should not point to root.
    pub fn remove_node_at_path(&mut self, path: NodePath) -> Option<MapDataNode> {
        assert!(!path.is_root());

        let parent_path = path.parent();
        let current_step = path.get_step()?;

        let parent_node = if parent_path.is_root() {
            &mut self.root
        } else {
            self.get_node_at_path_mut(&parent_path)?
        };

        let vec = parent_node.children_vec_mut(current_step.node_type)?;

        if current_step.index < vec.len() {
            Some(vec.remove(current_step.index))
        } else {
            None
        }
    }

    pub fn move_node(&mut self, child: NodePath, parent: NodePath) {
        if child.is_root() {
            // can't move root node
            return;
        }

        // prevent a node from moving into itself or its descendants
        if parent.is_descendant_of(&child) {
            return;
        }

        // get the list this node belongs to
        let child_type = child.get_step().unwrap().node_type();

        // remove node
        let child_node = self.remove_node_at_path(child).unwrap();

        // get parent node
        let parent_node = self.get_node_at_path_mut(&parent).unwrap();

        // push node
        parent_node
            .children_vec_option_mut(child_type)
            .get_or_insert_with(Vec::new)
            .push(child_node);
    }

    pub fn rebuild_string_tables(&mut self) {
        self.object_types.clear();
        self.item_types.clear();
        self.collision_types.clear();
        self.rect_types.clear();
        self.enemy_types.clear();
        self.unk_types_1.clear();

        fn insert_unique(table: &mut Vec<String32>, value: &String32) {
            if !table.contains(value) {
                table.push(value.clone());
            }
        }

        fn visit(node: &MapDataNode, map: &mut Mapdata) {
            match &node.node_data {
                NodeData::MapPolySet {
                    collision_type,
                    ..
                } => {
                    insert_unique(&mut map.collision_types, collision_type);
                }

                NodeData::MapObjSet {
                    name,
                    ..
                } => {
                    insert_unique(&mut map.object_types, name);
                }

                NodeData::MapItemSet {
                    name,
                    ..
                } => {
                    insert_unique(&mut map.item_types, name);
                }

                NodeData::MapEnemySet { name, ..} => {
                    insert_unique(&mut map.enemy_types, name);
                }

                NodeData::MapTerrain { collision_type, ..} => {
                    insert_unique(&mut map.collision_types, collision_type);
                }
                _ => {}
            }

            macro_rules! recurse {
                ($field:ident) => {
                    if let Some(children) = &node.$field {
                        for child in children {
                            visit(child, map);
                        }
                    }
                };
            }

            recurse!(children_mappolyset);
            recurse!(children_mapobjset);
            recurse!(children_mapitemset);
            recurse!(children_mapenemyset);
            recurse!(children_maplocator);
            recurse!(children_mappath);
            recurse!(children_maprect);
            recurse!(children_mapcircle);
            recurse!(children_mapterrain);
        }

        // avoid borrow issues
        let root = std::mem::take(&mut self.root);
        visit(&root, self);
        self.root = root;
    }
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

impl MapDataNode {
    /// On children.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (NodeStep, &mut MapDataNode)> {
        let mut items = Vec::new();

        // helper macro to reduce boilerplate
        macro_rules! collect_children {
            ($child_type:ident, $field:ident) => {
                if let Some(vec) = &mut self.$field {
                    for (i, node) in vec.iter_mut().enumerate() {
                        let step = NodeStep::new(NodeChildType::$child_type, i);
                        items.push((step, node));
                    }
                }
            };
        }

        collect_children!(MapPolySet, children_mappolyset);
        collect_children!(MapObjSet, children_mapobjset);
        collect_children!(MapItemSet, children_mapitemset);
        collect_children!(MapEnemySet, children_mapenemyset);
        collect_children!(MapLocator, children_maplocator);
        collect_children!(MapPath, children_mappath);
        collect_children!(MapRect, children_maprect);
        collect_children!(MapCircle, children_mapcircle);
        collect_children!(MapTerrain, children_mapterrain);

        items.into_iter()
    }

    pub fn children_vec(&self, child_type: NodeChildType) -> Option<&Vec<MapDataNode>> {
        match child_type {
            NodeChildType::MapPolySet => self.children_mappolyset.as_ref(),
            NodeChildType::MapObjSet => self.children_mapobjset.as_ref(),
            NodeChildType::MapItemSet => self.children_mapitemset.as_ref(),
            NodeChildType::MapEnemySet => self.children_mapenemyset.as_ref(),
            NodeChildType::MapLocator => self.children_maplocator.as_ref(),
            NodeChildType::MapPath => self.children_mappath.as_ref(),
            NodeChildType::MapRect => self.children_maprect.as_ref(),
            NodeChildType::MapCircle => self.children_mapcircle.as_ref(),
            NodeChildType::MapTerrain => self.children_mapterrain.as_ref(),
        }
    }

    pub fn children_vec_mut(&mut self, child_type: NodeChildType) -> Option<&mut Vec<MapDataNode>> {
        match child_type {
            NodeChildType::MapPolySet => self.children_mappolyset.as_mut(),
            NodeChildType::MapObjSet => self.children_mapobjset.as_mut(),
            NodeChildType::MapItemSet => self.children_mapitemset.as_mut(),
            NodeChildType::MapEnemySet => self.children_mapenemyset.as_mut(),
            NodeChildType::MapLocator => self.children_maplocator.as_mut(),
            NodeChildType::MapPath => self.children_mappath.as_mut(),
            NodeChildType::MapRect => self.children_maprect.as_mut(),
            NodeChildType::MapCircle => self.children_mapcircle.as_mut(),
            NodeChildType::MapTerrain => self.children_mapterrain.as_mut(),
        }
    }

    pub fn children_vec_option_mut(
        &mut self,
        child_type: NodeChildType,
    ) -> &mut Option<Vec<MapDataNode>> {
        match child_type {
            NodeChildType::MapPolySet => &mut self.children_mappolyset,
            NodeChildType::MapObjSet => &mut self.children_mapobjset,
            NodeChildType::MapItemSet => &mut self.children_mapitemset,
            NodeChildType::MapEnemySet => &mut self.children_mapenemyset,
            NodeChildType::MapLocator => &mut self.children_maplocator,
            NodeChildType::MapPath => &mut self.children_mappath,
            NodeChildType::MapRect => &mut self.children_maprect,
            NodeChildType::MapCircle => &mut self.children_mapcircle,
            NodeChildType::MapTerrain => &mut self.children_mapterrain,
        }
    }
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
        line: CollisionLine,
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
        lines: Vec<CollisionLine>,
        params: Params<3>,
        unk15: Option<[[String32; 2]; 3]>, // version >= 4.6
    },
}

impl NodeData {
    pub fn position(&self) -> Vec2f {
        match self {
            NodeData::MapSet {
                bounds_start,
                bounds_end,
                ..
            }
            | NodeData::MapRect {
                bounds_start,
                bounds_end,
                ..
            } => {
                // center of rect
                Vec2f {
                    x: (bounds_start.x + bounds_end.x) * 0.5,
                    y: (bounds_start.y + bounds_end.y) * 0.5,
                }
            }

            NodeData::MapPolySet { line, .. } => {
                // midpoint of line
                Vec2f {
                    x: (line.start.x + line.end.x) * 0.5,
                    y: (line.start.y + line.end.y) * 0.5,
                }
            }

            NodeData::MapObjSet { position, .. }
            | NodeData::MapItemSet { position, .. }
            | NodeData::MapEnemySet { position, .. }
            | NodeData::MapTerrain { position, .. }
            | NodeData::MapLocator { position, .. } => (*position).into(),

            NodeData::MapCircle { position, .. } => *position,

            NodeData::MapPath { points, .. } => points.first().copied().unwrap(),

            _ => unreachable!("Node data cannot be None"),
        }
    }
}
