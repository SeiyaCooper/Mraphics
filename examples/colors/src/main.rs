use mraphics::{BasicMaterial, Canvas, Color, LogicalTimeline, Point3D, constants::*};
use nalgebra::Vector3;

fn main() {
    let mut canvas = Canvas::new(LogicalTimeline::new());
    canvas.clear_color = Color::from_hex_str("#1f1f22ff").unwrap();

    let color_strs = [
        RED_A, RED_B, RED_C, RED_D, RED_E, //
        BLUE_A, BLUE_B, BLUE_C, BLUE_D, BLUE_E, //
        YELLOW_A, YELLOW_B, YELLOW_C, YELLOW_D, YELLOW_E, //
        GREEN_A, GREEN_B, GREEN_C, GREEN_D, GREEN_E, //
        GRAY_A, GRAY_B, GRAY_C, GRAY_D, GRAY_E, //
        WHITE, BLACK,
    ];
    let mut colors: Vec<Color<f32>> = Vec::new();

    for color_str in color_strs {
        colors.push(Color::from_hex_str(color_str).unwrap());
    }

    let col_num = 5;
    let mut point_index = 0;
    for color in &colors {
        let width = 3.0;
        let unit = width / col_num as f32;

        let mut point = Point3D::default().with_radius(unit / 2.0);
        point.material = BasicMaterial::new().with_color(color);
        canvas.add_mesh(&point);

        let row = point_index / col_num;
        let col = point_index % col_num;

        let mut scene = canvas.scene.borrow_mut();
        let instance = scene.acquire_instance_mut(point.identifier).unwrap();

        instance.move_to(&Vector3::new(
            (((col as f32) - ((col_num - 1) as f32 / 2.0)) / col_num as f32) * width,
            -width / 2.0 + (unit * row as f32),
            0.0,
        ));

        point_index += 1;
    }

    canvas.run();
}
