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
        0.16073917,
        0.4019736,
        0.2500405,
        0.23468459,
        displace(
            0.10390177,
            layer(
                2.4686515,
                layer(
                    3.0395193,
                    noise(4077839245, 11.842703, tile_all()),
                    vnoise(1246086663, 12.16001, Ease::Smooth5, tile_all()),
                ),
                rotate(
                    9.744911,
                    vnoise(3984989388, 8.905142, Ease::Smooth5, tile_all()),
                    vnoise(168447214, 5.8911786, Ease::Smooth5, tile_all()),
                ),
            ),
            reflect(
                1.884496,
                vec3(0.5632216, -0.31983083, -0.7500508),
                fractal(
                    7.6917915,
                    5,
                    0.50210387,
                    2.4051504,
                    0.0,
                    2.0523214,
                    worley_basis(3902470283, Ease::Id, tile_all(), 7, 10, 17),
                ),
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
        Space::HSL,
        0.550072,
        0.22867462,
        0.16526648,
        0.31744483,
        saturate(
            8.597511,
            fractal(
                5.8026867,
                4,
                0.5642442,
                2.114186,
                0.07022904,
                1.0159429,
                displace(
                    0.15992701,
                    voronoi_basis(1401237949, Ease::Id, tile_all(), 10, 25, 7),
                    worley_basis(785949362, Ease::Id, tile_all(), 0, 22, 14),
                ),
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
