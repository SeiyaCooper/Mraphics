use crate::{
    AsIntermediate, Geometry, GeometryIndices, GeometryUpdater, Material, Mesh, Transformable,
    Vertices,
};
use nalgebra::Vector3;

#[derive(Clone)]
pub struct Cube {
    pub width: f32,
    pub height: f32,
    pub depth: f32,

    pub vertices: Vertices,
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            depth: 1.0,

            vertices: Vertices::new(),
        }
    }
}

impl GeometryUpdater for Cube {
    fn update_view(&self, view: &mut super::GeometryView) {
        self.vertices.update_view(view);
        view.indices = GeometryIndices::Sequential(self.vertices.data.len() as u32);
    }
}

impl Geometry for Cube {
    fn init_view(&self, view: &mut super::GeometryView) {
        view.add_attribute(
            crate::constants::POSITION_ATTR_LABEL,
            crate::constants::POSITION_ATTR_INDEX,
            bytemuck::cast_slice::<f32, u8>(&self.vertices.data.concat()).to_vec(),
        );
    }

    fn init(&mut self) {
        let vertices = &mut self.vertices.data;

        fn to_homogeneous(point: &Vector3<f32>, w: f32) -> [f32; 4] {
            std::array::from_fn(|i| if i < 3 { point[i] } else { w })
        }

        let mut build_plane =
            |position: Vector3<f32>, width_len: f32, height_len: f32, normal: Vector3<f32>| {
                let mut height = normal.yzx();
                height.set_magnitude(height_len);

                let mut width = height.cross(&normal);
                width.set_magnitude(width_len);

                vertices.push(to_homogeneous(&position, 1.0));
                vertices.push(to_homogeneous(&(position + width), 1.0));
                vertices.push(to_homogeneous(&(position + width + height), 1.0));
                vertices.push(to_homogeneous(&(position + height), 1.0));
                vertices.push(to_homogeneous(&position, 1.0));
                vertices.push(to_homogeneous(&(position + width + height), 1.0));
            };

        let w = self.width;
        let h = self.height;
        let d = self.depth;

        build_plane(
            Vector3::new(-w / 2.0, -h / 2.0, -d / 2.0),
            w,
            h,
            Vector3::z(),
        );
        build_plane(
            Vector3::new(-w / 2.0, -h / 2.0, d / 2.0),
            w,
            h,
            Vector3::z(),
        );
        build_plane(
            Vector3::new(w / 2.0, -h / 2.0, -d / 2.0),
            h,
            d,
            Vector3::x(),
        );
        build_plane(
            Vector3::new(-w / 2.0, -h / 2.0, d / 2.0),
            h,
            d,
            -Vector3::x(),
        );
        build_plane(
            Vector3::new(-w / 2.0, h / 2.0, -d / 2.0),
            d,
            w,
            Vector3::y(),
        );
        build_plane(
            Vector3::new(w / 2.0, -h / 2.0, -d / 2.0),
            d,
            w,
            -Vector3::y(),
        );
    }
}

impl<M: Material> AsIntermediate for Mesh<Cube, M> {
    type Intermediate = Vertices;
    fn as_intermediate(&self) -> Self::Intermediate {
        self.geometry.vertices.clone()
    }
}

impl<M: Material> Transformable for Mesh<Cube, M> {
    fn apply_transform<Trans: Fn(&[f32; 3]) -> [f32; 3]>(
        &self,
        transform: Trans,
    ) -> Self::Intermediate {
        self.geometry.vertices.apply_transform(|vertex| {
            let transformed = transform(&[vertex[0], vertex[1], vertex[2]]);
            [transformed[0], transformed[1], transformed[2], 1.0]
        })
    }
}
