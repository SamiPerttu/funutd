//! Draw texture in a window.

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
        let size = LogicalSize::new(256.0, 256.0);
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
            world.draw(pixels.frame_mut());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape)
                || input.close_requested()
                || input.destroyed()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height).unwrap();
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
        self.phase = (self.phase + 1.0 / 128.0) % 1.0;
        self.z += 1.0 / 128.0;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        let texture = palette(
            0.69615453,
            0.063456625,
            0.9537466,
            0.6067893,
            0.297114,
            0.98029304,
            0.9030997,
            0.5589955,
            0.32888702,
            camo(
                2785669928,
                18.095413,
                Ease::Cubed,
                Distance::Norm8,
                tile_all(),
                0.0,
                0.73462117,
                0.011281766,
            ),
        );

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width) as i16;
            let y = (i / self.width) as i16;
            let fx: f32 = x as f32 / self.width as f32;
            let fy: f32 = y as f32 / self.height as f32;

            let value = texture.at_frequency(vec3a(fx, fy, self.z), None);

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
