use crate::{GadgetData, Geometry, GeometryIndices};
use nalgebra::Vector3;

pub struct Cube {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
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
        }
    }
}

impl Geometry for Cube {
    fn update_view(&self, view: &mut super::GeometryView) {
        let mut vertices: Vec<f32> = Vec::new();

        let mut build_plane =
            |position: Vector3<f32>, width_len: f32, height_len: f32, normal: Vector3<f32>| {
                let mut height = normal.yzx();
                height.set_magnitude(height_len);

                let mut width = height.cross(&normal);
                width.set_magnitude(width_len);

                vertices.extend(position.iter().chain(std::iter::once(&1.0)));
                vertices.extend((position + width).iter().chain(std::iter::once(&1.0)));
                vertices.extend(
                    (position + width + height)
                        .iter()
                        .chain(std::iter::once(&1.0)),
                );
                vertices.extend((position + height).iter().chain(std::iter::once(&1.0)));
                vertices.extend(position.iter().chain(std::iter::once(&1.0)));
                vertices.extend(
                    (position + width + height)
                        .iter()
                        .chain(std::iter::once(&1.0)),
                );
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

        view.reset_vertices();

        view.attributes.push(GadgetData {
            label: String::from(crate::constants::POSITION_ATTR_LABEL),
            index: crate::constants::POSITION_ATTR_INDEX,
            data: Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
            needs_update_value: true,
            needs_update_buffer: true,
        });
        view.indices = GeometryIndices::Sequential(vertices.len() as u32);
    }
}
