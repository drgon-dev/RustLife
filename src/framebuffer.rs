use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::window::Window;

pub struct Framebuffer {
    pixels: Pixels<'static>,
    width: u32,
    height: u32,
}

impl Framebuffer {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Self, pixels::Error> {
        // SAFETY: `Pixels<'static>` требует, чтобы заимствованное окно жило вечно.
        // Контракт нарушить нельзя, если `Framebuffer` уничтожается раньше `Window`:
        // поля `App` объявлены в порядке `framebuffer`, `window`, а Rust роняет
        // поля сверху вниз.
        let window: &'static Window = unsafe { &*(window as *const Window) };

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window);
        let pixels = PixelsBuilder::new(width, height, surface_texture).build()?;

        Ok(Self {
            pixels,
            width,
            height,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = ((y as usize) * self.width as usize + x as usize) * 4;
        self.pixels.frame_mut()[index..index + 4].copy_from_slice(&[color[0], color[1], color[2], 255]);
    }

    pub fn clear(&mut self, color: [u8; 3]) {
        let frame = self.pixels.frame_mut();
        for chunk in frame.as_chunks_mut::<4>().0 {
            chunk.copy_from_slice(&[color[0], color[1], color[2], 255]);
        }
    }

    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: [u8; 3]) {
        let x_end = x.saturating_add(w).min(self.width);
        let y_end = y.saturating_add(h).min(self.height);
        let frame = self.pixels.frame_mut();
        for py in y..y_end {
            let row = py as usize * self.width as usize * 4;
            for px in x..x_end {
                let index = row + px as usize * 4;
                frame[index..index + 4].copy_from_slice(&[color[0], color[1], color[2], 255]);
            }
        }
    }

    /// Изменяет разрешение пиксельного буфера, очищая его содержимое.
    #[allow(dead_code)]
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        if let Err(e) = self.pixels.resize_buffer(width, height) {
            eprintln!("resize_buffer failed: {e}");
            return;
        }
        let extent = self.pixels.context().texture_extent;
        self.width = extent.width;
        self.height = extent.height;
    }

    /// Подгоняет surface (окно) под новый размер; пиксельный буфер не меняется.
    pub fn resize_surface(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        if let Err(e) = self.pixels.resize_surface(width, height) {
            eprintln!("resize_surface failed: {e}");
        }
    }

    pub fn present(&mut self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}
