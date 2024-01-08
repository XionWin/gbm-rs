use crate::{Device, Surface, def::{SurfaceFormat, FormatModifier}};

#[derive(Debug)]
pub struct Gbm {
    pub drm: drm_rs::core::Drm,
    pub surface: Surface,
    pub surface_format: SurfaceFormat,
    pub format_modifiers: Vec<FormatModifier>,
    pub width: libc::c_int,
    pub height: libc::c_int,
}

impl Gbm {
    pub fn new(drm: drm_rs::core::Drm, surface_format: SurfaceFormat, format_modifiers: Vec<FormatModifier>) -> Self
    {
        let width = drm.crtc.get_width();
        let height = drm.crtc.get_height();
        let surface = Surface::new_with_modifiers(Device::new(drm.get_fd()), width, height, surface_format, &format_modifiers);
        Self{
            drm,
            surface,
            surface_format,
            format_modifiers,
            width,
            height,
        }
    }
    pub fn get_drm(&self) -> &drm_rs::core::Drm {
        &self.drm
    }
    
    pub fn get_surface(&self) -> &Surface {
        &self.surface
    }
    pub fn get_surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }

    
    pub fn get_width(&self) -> libc::c_int {
        self.width
    }
    pub fn get_height(&self) -> libc::c_int {
        self.height
    }
}