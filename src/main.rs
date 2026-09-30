mod framebuffer;

use framebuffer::Framebuffer;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Default)]
struct App {
    // Порядок важен: `Framebuffer` занимает `Window` на `'static`,
    // а поля роняются сверху вниз — framebuffer должен умереть первым.
    framebuffer: Option<Framebuffer>,
    window: Option<Window>,
    counter: u32,
}

fn draw(fb: &mut Framebuffer) {
    fb.clear([0, 0, 0]);

    for y in 0..fb.height() {
        for x in 0..fb.width() {
            let r = (x % 256) as u8;
            let g = (y % 256) as u8;
            let b = ((x + y) % 256) as u8;
            fb.set_pixel(x, y, [r, g, b]);
        }
    }

    fb.fill_rect(50, 50, 200, 150, [255, 0, 0]);
    fb.fill_rect(550, 400, 200, 150, [0, 0, 255]);
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("RustLife")
                    .with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
            )
            .unwrap();
        match Framebuffer::new(&window, WIDTH, HEIGHT) {
            Ok(fb) => {
                self.framebuffer = Some(fb);
                self.window = Some(window);
            }
            Err(e) => {
                eprintln!("failed to create framebuffer: {e}");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window.as_ref().map(|w| w.id()) != Some(id) {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(fb) = self.framebuffer.as_mut() {
                    fb.resize_surface(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(fb) = self.framebuffer.as_mut() {
                    draw(fb);
                    
                    if self.counter > 100 || self.counter < 0{
                        self.counter = 0;
                    }
                    fb.set_pixel(self.counter, self.counter,[255, 255, 255]);
                    self.counter = self.counter + 1;
                    
                    if let Err(e) = fb.present() {
                        eprintln!("failed to present: {e}");
                        event_loop.exit();
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}