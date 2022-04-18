use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 640;

use utd::color::*;
use utd::dna::*;
use utd::map3::*;
use utd::map3base::*;
use utd::map3gen::*;
use utd::math::*;
use utd::voronoi::*;
use utd::*;

/// Application state.
struct World {
    z: f32,
    phase: f32,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Texture Example")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance.
    fn new() -> Self {
        Self { z: 0.0, phase: 0.0 }
    }

    /// Update the `World` internal state.
    fn update(&mut self) {
        self.z += 0.001;
        self.phase += 0.05;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        #[allow(unused_variables)]
        let texture = palette(
            Space::HSL,
            0.6901543,
            0.9025886,
            posterize(
                9.943014,
                0.16815351,
                rotate(
                    8.86307,
                    voronoi(2309937501, 31.571928, tile_all(), 7, 3, 2),
                    voronoi(2538691872, 2.63204, tile_all(), 6, 5, 7),
                ),
            ),
        );
        /*
        let texture = palette(0.50937665, 0.7222409, posterize(3.8965485, 0.60872394, softmix3(5.2831173, vnoise(1974317952, 10.774254, tile_all()), voronoi(1974803501, 24.273146, tile_all(), 5, 9, 7))));
        */
        let mut dna = Dna::new(128, (self.z / 0.02) as u64);
        let texture = genmap3palette(20.0, &mut dna);

        println!("{}", texture.get_code());

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            let fx: f32 = x as f32 / WIDTH as f32;
            let fy: f32 = y as f32 / HEIGHT as f32;

            let value = texture.at(vec3a(fx, fy, self.z), None);

            let rgba = [
                (clamp01(value.x * 0.5 + 0.5) * 255.0) as u8,
                (clamp01(value.y * 0.5 + 0.5) * 255.0) as u8,
                (clamp01(value.z * 0.5 + 0.5) * 255.0) as u8,
                0xff,
            ];

            pixel.copy_from_slice(&rgba);
        }
    }
}
