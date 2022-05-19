use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use funutd::math::*;
use funutd::prelude::*;

/// Application state.
struct World {
    pub z: f32,
    pub phase: f32,
    pub width: usize,
    pub height: usize,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(512.0, 512.0);
        WindowBuilder::new()
            .with_title("Texture Example")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    let mut world = World::new(window_size.width as usize, window_size.height as usize);

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
    fn new(width: usize, height: usize) -> Self {
        Self {
            z: 0.0,
            phase: 0.0,
            width,
            height,
        }
    }

    /// Update the `World` internal state.
    fn update(&mut self) {
        self.phase = (self.phase + 1.0 / 32.0) % 1.0;
        self.z += 1.0 / 32.0;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        let mut dna = Dna::new(self.z as u64);
        let texture = genmap3palette(20.0, &mut dna);

        if self.phase > 0.0 && self.phase < 1.0 / 24.0 {
            println!("{}", texture.get_code());
        }

        //let texture = palette(Space::HSV, 0.0, 1.0, 0.0, 0.0, fractal(2.0, 8, 0.5, 2.0, 0.0, 0.0, noise_basis(1, tile_none())));
        //let texture = palette(Space::HSV, 0.0, 1.0, 0.0, 1.0, fractal(2.0, 8, 0.5, 2.0, 0.0, 0.0, vnoise_basis(1, tile_none())));
        //let texture = palette(Space::HSV, 0.0, 0.2, 0.0, 1.0, noise(1, 32.0, tile_none()));
        //let texture = palette(Space::HSV, 0.192, 0.418, 0.015, 0.403, fractal(4.660271, 4, 0.76092863, 2.532766, 0.11465196, 0.0, voronoi_basis(3581209750, tile_all(), 12, 0, 6)));
        //let texture = palette(Space::HSL, 0.739, 0.448, 0.281, 0.665, displace(0.18501252, voronoi(278220278, 17.42609, tile_all(), 0, 7, 10), voronoi(3737477767, 4.2395425, tile_all(), 2, 1, 0)));
        //let texture = palette(Space::HSL, 0.406, 0.814, 0.483, 0.329, fractal(3.4102457, 7, 0.4548667, 2.789417, 0.0, 0.0, vnoise_basis(2316030952, tile_all())));
        //let texture = palette(Space::HSV, 0.111, 0.712, 0.368, 0.051, displace(0.10156162, rotate(9.893959, noise(4137245708, 8.33033, tile_all()), voronoi(1284792858, 4.5874896, tile_all(), 7, 3, 3)), fractal(5.103115, 4, 0.47705963, 2.7184772, 0.0, 1.3609127, voronoi_basis(749463054, tile_all(), 5, 10, 2))));
        //let texture = palette(Space::HSL, 0.296, 0.515, 0.212, 0.331, saturate(5.5411787, rotate(5.0376725, voronoi(1660873412, 25.0088, tile_all(), 11, 0, 11), noise(2384626526, 4.481734, tile_all()))));

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width) as i16;
            let y = (i / self.width) as i16;
            let fx: f32 = x as f32 / self.width as f32;
            let fy: f32 = y as f32 / self.height as f32;

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
