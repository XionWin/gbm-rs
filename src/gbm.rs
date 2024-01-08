use std::os::unix::prelude::RawFd;

use crate::{Device, Surface, def::{SurfaceFormat, FormatModifier}};

#[derive(Debug)]
pub struct Gbm {
    pub drm: drm_rs::core::Drm,
    pub fd: RawFd,
    pub surface: Surface,
    pub surface_format: SurfaceFormat,
    pub format_modifiers: Vec<FormatModifier>,
    pub width: libc::c_int,
    pub height: libc::c_int,
}

impl Gbm {
    pub fn new(drm: drm_rs::core::Drm, surface_format: SurfaceFormat, format_modifiers: Vec<FormatModifier>) -> Self
    {
        let fd = drm.fd();
        let crtc = drm.get_crtc();
        let width = crtc.get_width();
        let height = crtc.get_height();
        let surface = Surface::new_with_modifiers(Device::new(fd), width, height, surface_format, &format_modifiers);
        Self{
            drm,
            fd,
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