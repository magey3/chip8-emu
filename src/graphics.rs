use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{self, VideoSubsystem};

pub enum DisplayEvent {
	Quit,
}

pub struct Display {
	canvas: Canvas<Window>,
}
impl Display {
	pub fn new(context: &sdl2::Sdl) -> Self {
		let video = context.video().unwrap();
		let window = video
			.window("Chip-8 Emulator", 640, 320)
			.position_centered()
			.opengl()
			.build()
			.unwrap();
		let mut canvas = window.into_canvas().present_vsync().build().unwrap();

		canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
		canvas.clear();
		canvas.present();

		Self { canvas }
	}
	pub fn update(&mut self, c: &sdl2::Sdl, screen: &[bool]) -> Vec<DisplayEvent> {
		let mut event_pump = c.event_pump().unwrap();
		let mut ret = Vec::new();
		for e in event_pump.poll_iter() {
			match e {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => ret.push(DisplayEvent::Quit),
				_ => {}
			}
		}

		self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
		self.canvas.clear();

		for i in 0..screen.len() {
			if screen[i] {
				self.canvas
					.set_draw_color(pixels::Color::RGB(255, 255, 255));
				const SIZE: u32 = 10;
				self.canvas
					.fill_rect(Rect::new(
						i as i32 * SIZE as i32 % 640,
						(i as i32 * SIZE as i32) / 640 * SIZE as i32,
						SIZE,
						SIZE,
					))
					.unwrap();
			}
		}

		self.canvas.present();
		ret
	}
}
