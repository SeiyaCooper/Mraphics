use crate::render::GadgetIndex;

// Built-in colors
mod colors;
pub use colors::*;

// Common resolutions
pub const RESOLUTION_4K: (u32, u32) = (3840, 2160);
pub const RESOLUTION_1080P: (u32, u32) = (1920, 1080);
pub const RESOLUTION_720P: (u32, u32) = (1280, 720);
pub const RESOLUTION_480P: (u32, u32) = (854, 480);
pub const RESOLUTION_360P: (u32, u32) = (640, 360);

// Primitive topologies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl PrimitiveTopology {
    pub fn to_wgpu(&self) -> wgpu::PrimitiveTopology {
        match self {
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
            PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            PrimitiveTopology::PointList => "point-list",
            PrimitiveTopology::LineList => "line-list",
            PrimitiveTopology::LineStrip => "line-strip",
            PrimitiveTopology::TriangleList => "triangle-list",
            PrimitiveTopology::TriangleStrip => "triangle-strip",
        }
    }
}

// Built-in gadgets
pub const VIEW_MAT_LABEL: &'static str = "mraphics-view-mat";
pub const VIEW_MAT_INDEX: GadgetIndex = GadgetIndex {
    group_index: 0,
    binding_index: 0,
};

pub const PROJECTION_MAT_LABEL: &'static str = "mraphics-projection-mat";
pub const PROJECTION_MAT_INDEX: GadgetIndex = GadgetIndex {
    group_index: 0,
    binding_index: 1,
};

pub const MODEL_MAT_LABEL: &'static str = "mraphics-model-mat";
pub const MODEL_MAT_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 0,
};

pub const POSITION_STORAGE_LABEL: &'static str = "mraphics-position-storage";
pub const POSITION_STORAGE_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 1,
};

pub const INDEX_BUFFER_LABEL: &'static str = "mraphics-index-buffer";

pub const COLOR_STORAGE_LABEL: &'static str = "mraphics-color-storage";
pub const COLOR_STORAGE_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 2,
};
