use crate::render::GadgetIndex;

pub(crate) const VIEW_MAT_LABEL: &'static str = "mraphics-view-mat";
pub(crate) const VIEW_MAT_INDEX: GadgetIndex = GadgetIndex {
    group_index: 0,
    binding_index: 0,
};

pub(crate) const PROJECTION_MAT_LABEL: &'static str = "mraphics-projection-mat";
pub(crate) const PROJECTION_MAT_INDEX: GadgetIndex = GadgetIndex {
    group_index: 0,
    binding_index: 1,
};

pub(crate) const MODEL_MAT_LABEL: &'static str = "mraphics-model-mat";
pub(crate) const MODEL_MAT_INDEX: GadgetIndex = GadgetIndex {
    group_index: 0,
    binding_index: 2,
};

pub(crate) const POSITION_ATTR_LABEL: &'static str = "mraphics-position-attribute";
pub(crate) const POSITION_ATTR_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 0,
};

pub(crate) const COLOR_ATTR_LABEL: &'static str = "mraphics-color-attribute";
pub(crate) const COLOR_ATTR_INDEX: GadgetIndex = GadgetIndex {
    group_index: 1,
    binding_index: 1,
};
