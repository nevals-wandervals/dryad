use sdl3::{Sdl, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

use crate::client::{
    MetadataApp,
    traits::{AppBuilder, AppRun, EventHandle},
};

pub struct SdlClient {
    metadata_app: MetadataApp<'static, u32>,
    canvas: Canvas<Window>,
    sdl_context: Sdl,
}

impl AppRun<'static, u32, sdl3::EventPump> for SdlClient {}

impl AppBuilder<'static, u32> for SdlClient {
    fn init(mut md: MetadataApp<'static, u32>) -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(md.title(), *md.width(), *md.height())
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window.into_canvas();

        md.set_is_init(true);

        Self {
            metadata_app: md,
            canvas,
            sdl_context,
        }
    }

    fn update(&mut self) {}

    fn render(&mut self) {
        let canvas = &mut self.canvas;
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();
    }

    fn get_metadata(&self) -> &MetadataApp<'static, u32> {
        &self.metadata_app
    }

    fn get_mut_metadata(&mut self) -> &mut MetadataApp<'static, u32> {
        &mut self.metadata_app
    }
}

impl EventHandle<sdl3::EventPump> for SdlClient {
    fn event_init(&self) -> sdl3::EventPump {
        self.sdl_context.event_pump().unwrap()
    }

    fn event_handler(&mut self, event_pump: &mut sdl3::EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.metadata_app.is_run = false,
                _ => {}
            }
        }
    }
}
