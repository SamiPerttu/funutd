//! Texture evolver GUI. WIP.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use funutd::prelude::*;
use rayon::prelude::*;
use std::sync::mpsc;
use std::thread;

/// Convert texture value to u8.
pub fn convert_u8(x: f32) -> u8 {
    ((x * 0.5 + 0.5).min(1.0).max(0.0) * 255.99999).floor() as u8
}

#[derive(Default, Clone)]
struct Row {
    pub origin: Vec3,
    pub width: usize,
    pub data: Vec<egui::Color32>,
}

impl Row {
    pub fn point(&self, x: usize) -> Vec3 {
        self.origin + vec3(x as f32 / self.width as f32, 0.0, 0.0)
    }
}

const SLOTS: usize = 4;

struct ImageMessage {
    pub slot: usize,
    pub image: egui::ColorImage,
}

struct RenderMessage {
    pub slot: usize,
    pub texture: Box<dyn Texture>,
}

struct RenderSlot {
    /// Current texture being rendered.
    texture: Box<dyn Texture>,
    /// Width of level 0.
    width: usize,
    /// Height of level 0.
    height: usize,
    /// Current MIP level.
    level: usize,
    /// Next row to render.
    row: usize,
    /// All rows for current MIP level, rendered or not.
    rows: Vec<Row>,
}

impl RenderSlot {
    pub fn new() -> Self {
        Self {
            texture: Box::new(zero()),
            width: 1024,
            height: 1024,
            level: 4,
            row: 0,
            rows: Vec::new(),
        }
    }
    fn level_width(&self) -> usize {
        self.width >> self.level
    }
    fn level_height(&self) -> usize {
        self.height >> self.level
    }
    fn compute_image(&self) -> Option<egui::ColorImage> {
        if self.row != self.rows.len() {
            return None;
        }
        assert!(self.rows.len() == self.level_height());
        let mut pixels = Vec::with_capacity(self.level_width() * self.level_height());
        for y in 0..self.rows.len() {
            pixels.extend_from_slice(&self.rows[y].data[..]);
        }
        Some(egui::ColorImage {
            size: [self.level_width(), self.level_height()],
            pixels,
        })
    }
    /// Sets the texture.
    pub fn set_texture(&mut self, texture: Box<dyn Texture>) {
        self.texture = texture;
        self.level = 4;
        self.row = 0;
        self.rows = Vec::new();
    }
    /// Computes some more pixels. Returns whether we did any work.
    pub fn advance(&mut self) -> bool {
        if self.row == self.rows.len() {
            if self.level == 0 {
                // Final state is a fully rendered texture.
                return false;
            }
            self.row = 0;
            self.level -= 1;
            self.rows.resize(self.level_height(), Row::default());
            for i in (1..self.rows.len() >> 1).rev() {
                self.rows.swap(i, i * 2);
            }
            for i in 0..self.rows.len() {
                self.rows[i].width = self.level_width();
                self.rows[i].origin = vec3(0.0, i as f32 / self.level_height() as f32, 0.5);
                if !self.rows[i].data.is_empty() {
                    self.rows[i]
                        .data
                        .resize(self.width >> self.level, egui::Color32::default());
                    for j in (0..self.rows[i].width >> 1).rev() {
                        self.rows[i].data[j * 2] = self.rows[i].data[j];
                    }
                }
            }
        }
        // Compute up to 128 rows in parallel.
        let batch_rows = (self.rows.len() - self.row).min(128);
        self.rows[self.row..self.row + batch_rows]
            .par_iter_mut()
            .for_each(|row| {
                let is_progressive = !row.data.is_empty();
                if !is_progressive {
                    row.data.resize(row.width, egui::Color32::default());
                }
                for x in 0..row.width {
                    if is_progressive && x & 1 == 0 {
                        continue;
                    }
                    let v = self.texture.at(row.point(x).into(), None);
                    row.data[x] =
                        egui::Color32::from_rgb(convert_u8(v.x), convert_u8(v.y), convert_u8(v.z));
                }
            });
        self.row += batch_rows;
        true
    }
}

struct ImageSlot {
    pub image: Option<egui::TextureHandle>,
    pub texture: Box<dyn Texture>,
    pub dna: Dna,
}

impl ImageSlot {
    pub fn get_texture(&mut self) -> Box<dyn Texture> {
        self.dna.reset();
        genmap3palette(50.0, &mut self.dna)
    }
}

fn main() {
    let (tx_render, rx_render): (mpsc::Sender<RenderMessage>, mpsc::Receiver<RenderMessage>) =
        mpsc::channel();
    let (tx_image, rx_image): (mpsc::Sender<ImageMessage>, mpsc::Receiver<ImageMessage>) =
        mpsc::channel();

    let app = EditorApp::new(tx_render, rx_image);

    thread::spawn(move || {
        let mut slot: Vec<RenderSlot> = Vec::new();
        for _ in 0..SLOTS {
            slot.push(RenderSlot::new());
        }
        let mut slot_index = 0;
        let mut no_progress = 0;

        loop {
            let progress = slot[slot_index].advance();
            if progress {
                no_progress = 0;
                if let Some(image) = slot[slot_index].compute_image() {
                    if tx_image
                        .send(ImageMessage {
                            slot: slot_index,
                            image,
                        })
                        .is_err()
                    {
                        continue;
                    }
                }
            } else {
                no_progress += 1;
            }
            slot_index = (slot_index + 1) % SLOTS;

            // If we cannot progress in any of the slots, we wait for a message.
            if no_progress >= SLOTS {
                if let Ok(message) = rx_render.recv() {
                    slot[message.slot].set_texture(message.texture);
                    no_progress = 0;
                }
            }
            while let Ok(message) = rx_render.try_recv() {
                slot[message.slot].set_texture(message.texture);
                no_progress = 0;
            }
        }
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some((1090.0, 640.0).into()),
        ..Default::default()
    };

    eframe::run_native(
        "Texture Evolver",
        options,
        Box::new(move |_cc| Box::new(app)),
    );
}

struct EditorApp {
    rnd: Rnd,
    can_exit: bool,
    is_exiting: bool,
    slot: Vec<ImageSlot>,
    focus_slot: usize,
    tx_render: mpsc::Sender<RenderMessage>,
    rx_image: mpsc::Receiver<ImageMessage>,
}

impl EditorApp {
    fn new(tx_render: mpsc::Sender<RenderMessage>, rx_image: mpsc::Receiver<ImageMessage>) -> Self {
        let mut app = Self {
            rnd: Rnd::from_time(),
            can_exit: false,
            is_exiting: false,
            slot: Vec::new(),
            focus_slot: 0,
            tx_render,
            rx_image,
        };
        for i in 0..SLOTS {
            let dna = Dna::new(1024, app.rnd.next_u64());
            let mut slot = ImageSlot {
                image: None,
                dna,
                texture: Box::new(zero()),
            };
            slot.texture = slot.get_texture();
            if app
                .tx_render
                .send(RenderMessage {
                    slot: i,
                    texture: slot.get_texture(),
                })
                .is_ok()
            {}
            app.slot.push(slot);
        }
        app
    }
    pub fn mutate(&mut self, source: usize) {
        self.focus_slot = source;
        for mutate_i in 0..SLOTS {
            if mutate_i == source {
                continue;
            }
            self.slot[mutate_i].dna = Dna::mutate(&self.slot[source].dna, self.rnd.next_u64(), 0.2);
            self.slot[mutate_i].texture = self.slot[mutate_i].get_texture();
            if self
                .tx_render
                .send(RenderMessage {
                    slot: mutate_i,
                    texture: self.slot[mutate_i].get_texture(),
                })
                .is_ok()
            {}
        }
    }
}

impl eframe::App for EditorApp {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.can_exit
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        while let Ok(message) = self.rx_image.try_recv() {
            self.slot[message.slot].image = Some(ctx.load_texture("", message.image));
        }

        egui::SidePanel::left("mosaic panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(image) = self.slot[0].image.clone() {
                    let button = egui::ImageButton::new(&image, (256.0, 256.0));
                    let response = ui.add(button);
                    if response.clicked() {
                        self.mutate(0);
                    }
                    if response.hovered() {
                        self.focus_slot = 0;
                    }
                }
                if let Some(image) = self.slot[1].image.clone() {
                    let button = egui::ImageButton::new(&image, (256.0, 256.0));
                    let response = ui.add(button);
                    if response.clicked() {
                        self.mutate(1);
                    }
                    if response.hovered() {
                        self.focus_slot = 1;
                    }
                }
            });
            ui.horizontal(|ui| {
                if let Some(image) = self.slot[2].image.clone() {
                    let button = egui::ImageButton::new(&image, (256.0, 256.0));
                    let response = ui.add(button);
                    if response.clicked() {
                        self.mutate(2);
                    }
                    if response.hovered() {
                        self.focus_slot = 2;
                    }
                }
                if let Some(image) = self.slot[3].image.clone() {
                    let button = egui::ImageButton::new(&image, (256.0, 256.0));
                    let response = ui.add(button);
                    if response.clicked() {
                        self.mutate(3);
                    }
                    if response.hovered() {
                        self.focus_slot = 3;
                    }
                }
            });
        });

        egui::SidePanel::right("big panel").show(ctx, |ui| {
            if let Some(image) = self.slot[self.focus_slot].image.clone() {
                let button = egui::ImageButton::new(&image, (512.0, 512.0));
                if ui.add(button).clicked() {
                    self.mutate(self.focus_slot);
                }
            }
            let code = self.slot[self.focus_slot].texture.get_code();
            ui.horizontal_wrapped(|ui| {
                if ui.button("Copy").clicked() {
                    let mut clipboard = arboard::Clipboard::new().unwrap();
                    clipboard.set_text(code.clone()).unwrap();
                }
            });
            ui.code(code);
        });

        if self.is_exiting {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.is_exiting = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.can_exit = true;
                            frame.quit();
                        }
                    });
                });
        }
        ctx.request_repaint();
    }
}
