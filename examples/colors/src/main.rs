use mraphics::{
    BasicMaterial, Canvas, Color, LogicalTimeline, PerspectiveCamera, Point3D, constants::*,
};

fn main() {
    let mut canvas = Canvas::new(LogicalTimeline::new(), PerspectiveCamera::default());

    // Resize the canvas to take a snapshot
    // canvas.resize((1920, 1080));

    // Array of built-in color hex strings from Mraphics constants
    let color_strs = [
        RED_A, RED_B, RED_C, RED_D, RED_E, //
        BLUE_A, BLUE_B, BLUE_C, BLUE_D, BLUE_E, //
        YELLOW_A, YELLOW_B, YELLOW_C, YELLOW_D, YELLOW_E, //
        GREEN_A, GREEN_B, GREEN_C, GREEN_D, GREEN_E, //
        GRAY_A, GRAY_B, GRAY_C, GRAY_D, GRAY_E, //
        WHITE, BLACK,
    ];
    let mut colors: Vec<Color<f32>> = Vec::new();

    // Parse each hex string into a Color object
    for color_str in color_strs {
        colors.push(Color::from_hex_str(color_str).unwrap());
    }

    let col_num = 5;
    let mut point_index = 0;
    for color in &colors {
        let width = 3.0;
        let unit = width / col_num as f32;

        let row = point_index / col_num;
        let col = point_index % col_num;
        let center = [
            (((col as f32) - ((col_num - 1) as f32 / 2.0)) / col_num as f32) * width,
            -width / 2.0 + (unit * row as f32),
            0.0,
        ];

        let mut point = Point3D::default()
            .with_radius(unit / 2.0)
            .with_center(center);

        // Apply material with the current color
        point.material = BasicMaterial::new().with_color(color);

        // Add the point to the canvas for rendering
        // Returns a handle that can be used for animations (not used here)
        canvas.add_mesh(point);

        point_index += 1;
    }

    // Output a snapshot of the 0th second
    // canvas.snapshot(0.0, "snapshot.png");

    // Start the rendering loop, this will freeze the program
    canvas.run();
}
