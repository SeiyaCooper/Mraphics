use crate::{
    geometry::{Attribute, Geometry, GeometryView},
    impl_inner_geometry_view,
};
use nalgebra::Vector3;

pub struct CubeDescriptor {
    width: f32,
    height: f32,
    depth: f32,
    color: Vector3<f32>,
}

impl Default for CubeDescriptor {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
            color: Vector3::new(0.8, 0.732, 0.314),
        }
    }
}

pub struct Cube {
    pub inner: Geometry,
}

impl Cube {
    pub fn new(desc: &CubeDescriptor) -> Self {
        let mut out = Self {
            inner: Geometry::new(),
        };

        let mut vertices: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();

        let mut build_plane = |position: Vector3<f32>, normal: Vector3<f32>| {
            let mut height = normal.yzx();
            height.set_magnitude(desc.height);

            let mut width = height.cross(&normal);
            width.set_magnitude(desc.width);

            vertices.extend(position.iter());
            vertices.extend((position + width).iter());
            vertices.extend((position + width + height).iter());
            vertices.extend((position + height).iter());
            vertices.extend(position.iter());
            vertices.extend((position + width + height).iter());

            colors.extend(desc.color.iter());
            colors.extend(desc.color.iter());
            colors.extend(desc.color.iter());
            colors.extend(desc.color.iter());
            colors.extend(desc.color.iter());
            colors.extend(desc.color.iter());
        };

        let w = desc.width;
        let h = desc.height;
        let d = desc.depth;

        build_plane(Vector3::new(-w / 2.0, -h / 2.0, -d / 2.0), Vector3::z());
        build_plane(Vector3::new(-w / 2.0, -h / 2.0, d / 2.0), Vector3::z());
        build_plane(Vector3::new(w / 2.0, -h / 2.0, -d / 2.0), Vector3::x());
        build_plane(Vector3::new(-w / 2.0, -h / 2.0, d / 2.0), -Vector3::x());
        build_plane(Vector3::new(-w / 2.0, h / 2.0, -d / 2.0), Vector3::y());
        build_plane(Vector3::new(w / 2.0, -h / 2.0, -d / 2.0), -Vector3::y());

        out.attributes_mut().push(Attribute {
            label: String::from(crate::constants::POSITION_ATTR_LABEL),
            index: crate::constants::POSITION_ATTR_INDEX,
            data: Vec::from(bytemuck::cast_slice::<f32, u8>(&vertices)),
            needs_update_value: true,
            needs_update_buffer: true,
        });

        out.attributes_mut().push(Attribute {
            label: String::from(crate::constants::COLOR_ATTR_LABEL),
            index: crate::constants::COLOR_ATTR_INDEX,
            data: Vec::from(bytemuck::cast_slice::<f32, u8>(&colors)),
            needs_update_value: true,
            needs_update_buffer: true,
        });

        out
    }
}

impl_inner_geometry_view!(Cube);
