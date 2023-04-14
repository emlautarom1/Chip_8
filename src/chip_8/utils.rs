use glutin_window::GlutinWindow as Window;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;

pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

/// OpenGL version used
pub const OPENGL: OpenGL = OpenGL::V3_2;

/// Build a Window for displaying the VM
pub fn build_window() -> Window {
    WindowSettings::new("Chip 8", [1280, 640])
        .graphics_api(OPENGL)
        .exit_on_esc(true)
        .build()
        .unwrap()
}

/// Build a GLGraphics instance, needed to render on screen
pub fn build_graphics() -> GlGraphics {
    GlGraphics::new(OPENGL)
}
