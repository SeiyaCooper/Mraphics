use mraphics_core::{
    Color, GadgetIndex, GeometryView, InstanceUpdater, Material, MeshLike, Mobject2DMaterial,
    MraphicsID, MultiColoredMaterial, RenderInstance,
};

const PREVIOUS_ATTR_LABEL: &'static str = "mobject-2d-previous-attribute";
const REVERSE_ATTR_LABEL: &'static str = "mobject-2d-reverse-attribute";
const THICKNESS_LABEL: &'static str = "mobject-2d-thickness-uniform";

pub struct Mobject2DPath {
    pub vertices: Vec<[f32; 3]>,

    pub stroked: bool,
    pub filled: bool,

    pub stroke_color: Color<f32>,
    pub fill_color: Color<f32>,
}

impl Mobject2DPath {
    pub fn new(vertices: Vec<[f32; 3]>) -> Self {
        Self {
            vertices,

            stroked: false,
            filled: false,

            // SAFETY: `BLACK` and `WHITE` are valid hex color strings defined in `crate::constants`.
            // `Color::from_hex_str` will succeed without panicking for these well-formed inputs.
            stroke_color: Color::from_hex_str(mraphics_core::constants::BLACK).unwrap(),
            fill_color: Color::from_hex_str(mraphics_core::constants::WHITE).unwrap(),
        }
    }
}

pub struct Mobject2DStroke {
    pub color: Color<f32>,
    pub thickness: f32,
    material: Mobject2DMaterial,
}

impl Mobject2DStroke {
    fn new() -> Self {
        Self {
            // SAFETY: `BLACK` is a valid hex color string defined in `crate::constants`.
            color: Color::from_hex_str(mraphics_core::constants::BLACK).unwrap(),
            thickness: 0.05,
            material: Mobject2DMaterial::new(),
        }
    }

    fn init_geometry_view(&self, view: &mut GeometryView) {
        view.add_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            mraphics_core::constants::POSITION_ATTR_INDEX,
            vec![],
        );

        view.add_attribute(
            PREVIOUS_ATTR_LABEL,
            GadgetIndex {
                group_index: 1,
                binding_index: 3,
            },
            vec![],
        );

        view.add_attribute(
            REVERSE_ATTR_LABEL,
            GadgetIndex {
                group_index: 1,
                binding_index: 4,
            },
            vec![],
        );

        view.add_attribute(
            mraphics_core::constants::COLOR_ATTR_LABEL,
            mraphics_core::constants::COLOR_ATTR_INDEX,
            vec![],
        );

        view.add_uniform(
            THICKNESS_LABEL,
            GadgetIndex {
                group_index: 1,
                binding_index: 5,
            },
            vec![],
        );
    }

    fn update_instance(&self, paths: &Vec<Mobject2DPath>, instance: &mut RenderInstance) {
        let view = &mut instance.geometry;

        let mut vertices = Vec::new();
        let mut previous = Vec::new();
        let mut color = Vec::new();
        let mut reverse = Vec::new();

        fn to_homogeneous(point: &[f32; 3]) -> [f32; 4] {
            [point[0], point[1], point[2], 1.0]
        }

        let mut build_path = |path: &Mobject2DPath| {
            let points = &path.vertices;

            // We need at least two points to build segments.
            if points.is_empty() || points.len() < 2 {
                return;
            }

            for i in 1..points.len() {
                let start = &to_homogeneous(&points[i]);
                let end = &to_homogeneous(&points[i - 1]);

                vertices.extend_from_slice(start);
                vertices.extend_from_slice(start);
                vertices.extend_from_slice(end);
                vertices.extend_from_slice(start);
                vertices.extend_from_slice(end);
                vertices.extend_from_slice(end);

                previous.extend_from_slice(end);
                previous.extend_from_slice(end);
                previous.extend_from_slice(start);
                previous.extend_from_slice(end);
                previous.extend_from_slice(start);
                previous.extend_from_slice(start);

                color.extend_from_slice(&path.stroke_color);
                color.extend_from_slice(&path.stroke_color);
                color.extend_from_slice(&path.stroke_color);
                color.extend_from_slice(&path.stroke_color);
                color.extend_from_slice(&path.stroke_color);
                color.extend_from_slice(&path.stroke_color);

                reverse.extend_from_slice(&[-1., 1., 1., 1., 1., -1.]);
            }
        };

        for path in paths {
            if path.stroked {
                build_path(&path);
            }
        }

        // SAFETY: These attributes exist because we initialized them in `Self::init_geometry_view`
        view.set_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
        )
        .unwrap();
        view.set_attribute(
            PREVIOUS_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&previous)),
        )
        .unwrap();
        view.set_attribute(
            REVERSE_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&reverse)),
        )
        .unwrap();
        view.set_attribute(
            mraphics_core::constants::COLOR_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&color)),
        )
        .unwrap();
        view.set_uniform(
            THICKNESS_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&[self.thickness])),
        )
        .unwrap();

        let vertex_count = (vertices.len() / 4) as u32;

        view.indices = mraphics_core::GeometryIndices::Sequential(vertex_count);

        if vertex_count == 0 {
            instance.visible = false;
        } else {
            instance.visible = true;
        }
    }
}

pub struct Mobject2DFill {
    pub color: Color<f32>,
    material: MultiColoredMaterial,
}

impl Mobject2DFill {
    fn new() -> Self {
        Self {
            // SAFETY: `WHITE` is a valid hex color string defined in `crate::constants`.
            color: Color::from_hex_str(mraphics_core::constants::WHITE).unwrap(),

            material: MultiColoredMaterial::new(),
        }
    }

    fn init_geometry_view(&self, view: &mut GeometryView) {
        view.add_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            mraphics_core::constants::POSITION_ATTR_INDEX,
            vec![],
        );
        view.add_attribute(
            mraphics_core::constants::COLOR_ATTR_LABEL,
            mraphics_core::constants::COLOR_ATTR_INDEX,
            vec![],
        );
    }

    fn update_instance(&self, paths: &Vec<Mobject2DPath>, instance: &mut RenderInstance) {
        let view = &mut instance.geometry;

        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        fn to_homogeneous(point: &[f32; 3]) -> [f32; 4] {
            [point[0], point[1], point[2], 1.0]
        }

        fn build_path(path: &Mobject2DPath, vertices: &mut Vec<f32>, colors: &mut Vec<f32>) {
            let points = &path.vertices;

            // We need at least three points to build polygons.
            if points.is_empty() || points.len() < 3 {
                return;
            }

            let first = &points[0];

            // SAFETY: Indices are within the valid range.
            // 1. Range `1..(points.len() - 1)` ensures `i ∈ [1, len - 2]`
            // 2. Thus `i < len` and `i + 1 < len` for all iterations
            for i in 1..(points.len() - 1) {
                vertices.extend_from_slice(&to_homogeneous(first));
                vertices.extend_from_slice(&to_homogeneous(&points[i]));
                vertices.extend_from_slice(&to_homogeneous(&points[i + 1]));

                colors.extend_from_slice(&path.fill_color);
                colors.extend_from_slice(&path.fill_color);
                colors.extend_from_slice(&path.fill_color);
            }
        }

        for path in paths {
            if path.filled {
                build_path(&path, &mut vertices, &mut colors);
            }
        }

        // SAFETY: These attributes exist because we initialized them in `Self::init_geometry_view`
        view.set_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
        )
        .unwrap();
        view.set_attribute(
            mraphics_core::constants::COLOR_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&colors)),
        )
        .unwrap();

        let vertex_count = (vertices.len() / 4) as u32;

        view.indices = mraphics_core::GeometryIndices::Sequential(vertex_count);

        if vertex_count == 0 {
            instance.visible = false;
        } else {
            instance.visible = true;
        }
    }
}

pub struct Mobject2D {
    identifier: MraphicsID,

    vertices: Vec<[f32; 3]>,
    pub paths: Vec<Mobject2DPath>,

    pub stroke: Mobject2DStroke,
    pub fill: Mobject2DFill,
}

impl Mobject2D {
    pub fn new() -> Self {
        Self {
            identifier: MraphicsID::acquire(),

            vertices: Vec::new(),
            paths: Vec::new(),

            stroke: Mobject2DStroke::new(),
            fill: Mobject2DFill::new(),
        }
    }

    /// Moves the current path point to the specified position.
    ///
    /// This operation ends the previous path (if any) and starts a new path
    /// beginning at the given point. If there was an active path with vertices,
    /// it will be finalized before starting the new path.
    pub fn move_to(&mut self, point: [f32; 3]) {
        self.finish();

        self.vertices.push(point);
    }

    /// Draws a straight line from the previous path point to the specified point.
    ///
    /// The line is added to the current active path.
    /// If no path is active or there is no previous path point,
    /// this operation will behave like [`Self::move_to`].
    pub fn line_to(&mut self, point: [f32; 3]) {
        self.vertices.push(point);
    }

    /// Strokes the most recently drawn path.
    ///
    /// This method finalizes the current path (if any) and marks it for
    /// stroking with the current stroke color.
    pub fn stroke(&mut self) {
        self.finish();

        let len = self.paths.len();

        if len == 0 {
            return;
        }

        self.paths[len - 1].stroked = true;
        self.paths[len - 1].stroke_color = self.stroke.color.clone();
    }

    /// Fills the most recently drawn path.
    ///
    /// This method finalizes the current path (if any) and marks it for
    /// filling with the current fill color.
    pub fn fill(&mut self) {
        self.finish();

        let len = self.paths.len();

        if len == 0 {
            return;
        }

        self.paths[len - 1].filled = true;
        self.paths[len - 1].fill_color = self.fill.color.clone();
    }

    /// Finalizes the current drawing path.
    ///
    /// This method is automatically called by [`Self::stroke`] and [`Self::fill`] methods,
    /// but can also be called manually when you want to finalize a path
    /// without applying stroke or fill.
    pub fn finish(&mut self) {
        if self.vertices.len() == 0 {
            return;
        }

        self.paths
            .push(Mobject2DPath::new(std::mem::take(&mut self.vertices)));
    }
}

impl InstanceUpdater for Mobject2D {
    fn update_instance(&self, instance: &mut RenderInstance) {
        self.fill.update_instance(&self.paths, instance);

        // SAFETY: `instance.children[0]` is initialized because:
        // 1. `Self::build_instance` ensures the first child exists and is properly initialized
        // 2. The `instance` structure maintains this invariant throughout its lifetime
        self.stroke
            .update_instance(&self.paths, &mut instance.children[0]);
    }
}

impl MeshLike for Mobject2D {
    fn identifier(&self) -> MraphicsID {
        self.identifier
    }

    fn build_instance(&self) -> mraphics_core::RenderInstance {
        let mut instance = RenderInstance::new(self.identifier, &self.fill.material);
        let mut stroke = RenderInstance::new(MraphicsID::acquire(), &self.stroke.material);

        self.fill.init_geometry_view(&mut instance.geometry);
        self.stroke.init_geometry_view(&mut stroke.geometry);

        self.fill.material.update_view(&mut instance.material);
        self.stroke.material.update_view(&mut stroke.material);

        instance.add_child(stroke);

        instance
    }
}
