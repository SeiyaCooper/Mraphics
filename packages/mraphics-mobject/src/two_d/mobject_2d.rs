use mraphics_core::{
    BasicMaterial, Color, GeometryView, InstanceUpdater, Material, MeshLike, MraphicsID,
    RenderInstance,
};

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

struct Mobject2DStroke {
    color: Color<f32>,
    material: BasicMaterial,
}

impl Mobject2DStroke {
    fn new() -> Self {
        Self {
            // SAFETY: `BLACK` is a valid hex color string defined in `crate::constants`.
            color: Color::from_hex_str(mraphics_core::constants::BLACK).unwrap(),

            material: BasicMaterial::new(),
        }
    }

    fn init_geometry_view(&self, view: &mut GeometryView) {
        view.add_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            mraphics_core::constants::POSITION_ATTR_INDEX,
            bytemuck::cast_slice::<f32, u8>(&[]).to_vec(),
        );
    }

    fn update_geometry_view(&self, paths: &Vec<Mobject2DPath>, view: &mut GeometryView) {
        let mut vertices = Vec::new();

        fn to_homogeneous(point: &[f32; 3]) -> [f32; 4] {
            [point[0], point[1], point[2], 1.0]
        }

        fn build_polygon(points: &Vec<[f32; 3]>, output: &mut Vec<f32>) {
            // We need at least three points to build a polygon.
            if points.is_empty() || points.len() < 3 {
                return;
            }

            let first = &points[0];

            // SAFETY: Indices are within the valid range.
            // 1. Range `1..(points.len() - 1)` ensures `i ∈ [1, len - 2]`
            // 2. Thus `i < len` and `i + 1 < len` for all iterations
            for i in 1..(points.len() - 1) {
                output.extend_from_slice(&to_homogeneous(first));
                output.extend_from_slice(&to_homogeneous(&points[i]));
                output.extend_from_slice(&to_homogeneous(&points[i + 1]));
            }
        }

        for path in paths {
            build_polygon(&path.vertices, &mut vertices);
        }

        // SAFETY: This attribute exists because we initialized it in `Self::init_geometry_view`
        view.set_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
        )
        .unwrap()
    }
}

struct Mobject2DFill {
    color: Color<f32>,
    material: BasicMaterial,
}

impl Mobject2DFill {
    fn new() -> Self {
        Self {
            // SAFETY: `WHITE` is a valid hex color string defined in `crate::constants`.
            color: Color::from_hex_str(mraphics_core::constants::WHITE).unwrap(),

            material: BasicMaterial::new(),
        }
    }

    fn init_geometry_view(&self, view: &mut GeometryView) {
        view.add_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            mraphics_core::constants::POSITION_ATTR_INDEX,
            bytemuck::cast_slice::<f32, u8>(&[]).to_vec(),
        );
    }

    fn update_geometry_view(&self, paths: &Vec<Mobject2DPath>, view: &mut GeometryView) {
        let mut vertices = Vec::new();
        let mut vertex_count = 0;

        fn to_homogeneous(point: &[f32; 3]) -> [f32; 4] {
            [point[0], point[1], point[2], 1.0]
        }

        fn build_polygon(points: &Vec<[f32; 3]>, output: &mut Vec<f32>, count: &mut u32) {
            // We need at least three points to build a polygon.
            if points.is_empty() || points.len() < 3 {
                return;
            }

            let first = &points[0];

            // SAFETY: Indices are within the valid range.
            // 1. Range `1..(points.len() - 1)` ensures `i ∈ [1, len - 2]`
            // 2. Thus `i < len` and `i + 1 < len` for all iterations
            for i in 1..(points.len() - 1) {
                output.extend_from_slice(&to_homogeneous(first));
                output.extend_from_slice(&to_homogeneous(&points[i]));
                output.extend_from_slice(&to_homogeneous(&points[i + 1]));

                *count += 3
            }
        }

        for path in paths {
            build_polygon(&path.vertices, &mut vertices, &mut vertex_count);
        }

        // SAFETY: This attribute exists because we initialized it in `Self::init_geometry_view`
        view.set_attribute(
            mraphics_core::constants::POSITION_ATTR_LABEL,
            Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
        )
        .unwrap();

        view.indices = mraphics_core::GeometryIndices::Sequential(vertex_count);
    }
}

pub struct Mobject2D {
    identifier: MraphicsID,

    vertices: Vec<[f32; 3]>,
    paths: Vec<Mobject2DPath>,

    stroke: Mobject2DStroke,
    fill: Mobject2DFill,
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
        self.fill
            .update_geometry_view(&self.paths, &mut instance.geometry);

        // SAFETY: `instance.children[0]` is initialized because:
        // 1. `Self::build_instance` ensures the first child exists and is properly initialized
        // 2. The `instance` structure maintains this invariant throughout its lifetime
        self.stroke
            .update_geometry_view(&self.paths, &mut instance.children[0].geometry);
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

        self.update_instance(&mut instance);

        instance
    }
}
