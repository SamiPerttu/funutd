use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

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
        let texture = posterize(
            3.0981002,
            0.6513046,
            voronoi(0, 7.175018, tile_none(), 10, 3, 1),
        );
        let mut dna = Dna::new(64, (self.z / 0.02) as u64);
        let texture = genmap3(10.0, &mut dna);

        println!("{}", texture.get_code());

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            let fx: f32 = x as f32 / WIDTH as f32;
            let fy: f32 = y as f32 / HEIGHT as f32;

            //let texture = saturate(10.0, vnoise(0, 5.0, tile_all()));
            //let texture = reflect(10.0, vec3a(0.0, 0.0, 0.0), vnoise(0, 5.0, tile_xy()));
            //let texture = vreflect(5.0 + 4.0 * self.phase.sin(), vnoise(1, 10.0, tile_none()));
            //let texture = overdrive(5.0, vnoise(0, 8.0, tile_all()));
            //let texture = posterize(3.0, 0.5, vnoise(0, 10.0, tile_none()));

            //let texture = softmix3(3.7292252, vnoise(0, 2.2867033, tile_none()), voronoi(0, 60.37787, tile_none(), 12, 5, 8));

            let value = texture.at(vec3a(fx, fy, self.z));

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
