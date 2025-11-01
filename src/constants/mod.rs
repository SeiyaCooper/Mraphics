use crate::render::GadgetIndex;

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
    group_index: 0,
    binding_index: 2,
};

pub const POSITION_ATTR_LABEL: &'static str = "mraphics-position-attribute";
pub const POSITION_ATTR_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 0,
};

pub const COLOR_ATTR_LABEL: &'static str = "mraphics-color-attribute";
pub const COLOR_ATTR_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 1,
};
