//! Plot ease_noise.png and fractal_noise.png for documentation.

use funutd::prelude::*;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("example1.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .x_label_area_size(10u32)
        .y_label_area_size(10u32)
        .build_cartesian_2d(0.0..1.0f64, 0.0f64..1.0f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let texture = palette(
        Space::HSL,
        0.50937665,
        0.7222409,
        0.0,
        1.0,
        posterize(
            3.8965485,
            0.60872394,
            softmix3(
                5.2831173,
                vnoise(1974317952, 10.774254, tile_all()),
                voronoi(1974803501, 24.273146, tile_all(), 5, 9, 7),
            ),
        ),
    );

    for x in 0..1024 {
        for y in 0..1024 {
            let v = texture.at(vec3a(x as f32 / 1024.0, y as f32 / 1024.0, 0.5), None);
            let u = v * 0.5 + 0.5;
            plotting_area.draw_pixel(
                (x as f64 / 1024.0, y as f64 / 1024.0),
                &RGBColor(
                    (u.x * 255.9).floor() as u8,
                    (u.y * 255.9).floor() as u8,
                    (u.z * 255.9).floor() as u8,
                ),
            )?;
        }
    }

    let root = BitMapBackend::new("example2.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .x_label_area_size(10u32)
        .y_label_area_size(10u32)
        .build_cartesian_2d(0.0..1.0f64, 0.0f64..1.0f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let texture = palette(
        Space::HSV,
        0.7194102,
        0.21881655,
        0.0,
        1.0,
        fractal(
            5.3895693,
            7,
            0.5545446,
            2.5686815,
            0.0022501,
            0.0,
            posterize(4.580785, 0.2511709, vnoise_basis(2690581512, tile_all())),
        ),
    );

    for x in 0..1024 {
        for y in 0..1024 {
            let v = texture.at(vec3a(x as f32 / 1024.0, y as f32 / 1024.0, 0.5), None);
            let u = v * 0.5 + 0.5;
            plotting_area.draw_pixel(
                (x as f64 / 1024.0, y as f64 / 1024.0),
                &RGBColor(
                    (u.x * 255.999).floor() as u8,
                    (u.y * 255.999).floor() as u8,
                    (u.z * 255.999).floor() as u8,
                ),
            )?;
        }
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file");
    println!("Result has been saved");

    Ok(())
}

#[test]
fn entry_point() {
    main().unwrap()
}
