//! Implementation of a client for displaying simulations for different platforms and rendering libraries.

use std::borrow::Cow;
pub mod sdl;
pub mod traits;

#[derive(Debug, Clone, Default)]
pub struct MetadataApp<'a, T> {
    title: Cow<'a, str>,
    width: T,
    height: T,
    is_run: bool,
    is_init: bool,
}

impl<'a, T> MetadataApp<'a, T> {
    pub fn with_title(mut self, title: Cow<'a, str>) -> Self {
        self.title = title;

        self
    }

    pub fn title(&self) -> &Cow<'a, str> {
        &self.title
    }

    pub fn with_width(mut self, width: T) -> Self {
        self.width = width;

        self
    }

    pub fn width(&self) -> &T {
        &self.width
    }

    pub fn with_height(mut self, height: T) -> Self {
        self.height = height;

        self
    }

    pub fn height(&self) -> &T {
        &self.height
    }

    pub fn is_run(&self) -> bool {
        self.is_run
    }

    pub fn set_is_run(&mut self, value: bool) {
        self.is_run = value;
    }

    pub fn is_init(&self) -> bool {
        self.is_init
    }

    pub fn set_is_init(&mut self, value: bool) {
        self.is_init = value;
    }
}
