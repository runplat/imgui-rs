use std::os::raw::c_void;

use crate::math::MintVec2;
use crate::math::MintVec4;
use crate::render::renderer::TextureId;
use crate::sys;
use crate::Ui;

/// Builder for an image widget
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct Image {
    texture_id: TextureId,
    size: [f32; 2],
    uv0: [f32; 2],
    uv1: [f32; 2],
    tint_col: [f32; 4],
    border_col: [f32; 4],
}

impl Image {
    /// Creates a new image builder with the given texture and size
    #[doc(alias = "Image")]
    pub fn new(texture_id: TextureId, size: impl Into<MintVec2>) -> Image {
        Image {
            texture_id,
            size: size.into().into(),
            uv0: [0.0, 0.0],
            uv1: [1.0, 1.0],
            tint_col: [1.0, 1.0, 1.0, 1.0],
            border_col: [0.0, 0.0, 0.0, 0.0],
        }
    }
    /// Sets the image size
    #[deprecated(note = "just set the size in the `new` constructor.")]
    pub fn size(mut self, size: impl Into<MintVec2>) -> Self {
        self.size = size.into().into();
        self
    }
    /// Sets uv0 (default `[0.0, 0.0]`)
    pub fn uv0(mut self, uv0: impl Into<MintVec2>) -> Self {
        self.uv0 = uv0.into().into();
        self
    }
    /// Sets uv1 (default `[1.0, 1.0]`)
    pub fn uv1(mut self, uv1: impl Into<MintVec2>) -> Self {
        self.uv1 = uv1.into().into();
        self
    }
    /// Sets the tint color (default: no tint color)
    pub fn tint_col(mut self, tint_col: impl Into<MintVec4>) -> Self {
        self.tint_col = tint_col.into().into();
        self
    }
    /// Sets the border color (default: no border)
    pub fn border_col(mut self, border_col: impl Into<MintVec4>) -> Self {
        self.border_col = border_col.into().into();
        self
    }
    /// Builds the image
    pub fn build(self, _: &Ui) {
        unsafe {
            sys::igImage(
                self.texture_id.id() as *mut c_void,
                self.size.into(),
                self.uv0.into(),
                self.uv1.into(),
                self.tint_col.into(),
                self.border_col.into(),
            );
        }
    }
}

/// Builder for an image button widget
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct ImageButton<'ui, StrId> {
    str_id: StrId,
    texture_id: TextureId,
    size: [f32; 2],
    uv0: [f32; 2],
    uv1: [f32; 2],
    bg_col: [f32; 4],
    tint_col: [f32; 4],
    ui: &'ui Ui,
}

#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct ImageButtonDeprecated {
    texture_id: TextureId,
    size: [f32; 2],
    uv0: [f32; 2],
    uv1: [f32; 2],
    frame_padding: i32,
    bg_col: [f32; 4],
    tint_col: [f32; 4],
}

impl ImageButton<'static, ()> {
    /// Creates a new image button builder with the given texture and size
    #[deprecated(since = "0.10.0", note = "Use `ui.image_button_config(...)` instead")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(texture_id: TextureId, size: impl Into<MintVec2>) -> ImageButtonDeprecated {
        ImageButtonDeprecated {
            texture_id,
            size: size.into().into(),
            uv0: [0.0, 0.0],
            uv1: [1.0, 1.0],
            frame_padding: -1,
            bg_col: [0.0, 0.0, 0.0, 0.0],
            tint_col: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

impl<'ui, StrId: AsRef<str>> ImageButton<'ui, StrId> {
    /// Creates a new image button builder with the given texture and size
    #[deprecated(since = "0.10.0", note = "Use `ui.image_button_config(...)` instead")]
    pub fn new_with_id(
        ui: &'ui Ui,
        str_id: StrId,
        texture_id: TextureId,
        size: impl Into<MintVec2>,
    ) -> Self {
        Self {
            str_id,
            texture_id,
            size: size.into().into(),
            uv0: [0.0, 0.0],
            uv1: [1.0, 1.0],
            bg_col: [0.0, 0.0, 0.0, 0.0],
            tint_col: [1.0, 1.0, 1.0, 1.0],
            ui,
        }
    }

    /// Sets the image button size
    #[deprecated(note = "just set the size in the `new` constructor.")]
    pub fn size(mut self, size: impl Into<MintVec2>) -> Self {
        self.size = size.into().into();
        self
    }
    /// Sets uv0 (default `[0.0, 0.0]`)
    pub fn uv0(mut self, uv0: impl Into<MintVec2>) -> Self {
        self.uv0 = uv0.into().into();
        self
    }
    /// Sets uv1 (default `[1.0, 1.0]`)
    pub fn uv1(mut self, uv1: impl Into<MintVec2>) -> Self {
        self.uv1 = uv1.into().into();
        self
    }
    /// Sets the background color (default: no background color)
    pub fn background_col(mut self, bg_col: impl Into<MintVec4>) -> Self {
        self.bg_col = bg_col.into().into();
        self
    }
    /// Sets the tint color (default: no tint color)
    pub fn tint_col(mut self, tint_col: impl Into<MintVec4>) -> Self {
        self.tint_col = tint_col.into().into();
        self
    }
    /// Builds the image button
    pub fn build(self) -> bool {
        unsafe {
            sys::igImageButton(
                self.ui.scratch_txt(self.str_id),
                self.texture_id.id() as *mut c_void,
                self.size.into(),
                self.uv0.into(),
                self.uv1.into(),
                self.bg_col.into(),
                self.tint_col.into(),
            )
        }
    }
}

impl ImageButtonDeprecated {
    /// Sets the image button size
    #[deprecated(note = "just set the size in the `new` constructor.")]
    pub fn size(mut self, size: impl Into<MintVec2>) -> Self {
        self.size = size.into().into();
        self
    }
    /// Sets uv0 (default `[0.0, 0.0]`)
    pub fn uv0(mut self, uv0: impl Into<MintVec2>) -> Self {
        self.uv0 = uv0.into().into();
        self
    }
    /// Sets uv1 (default `[1.0, 1.0]`)
    pub fn uv1(mut self, uv1: impl Into<MintVec2>) -> Self {
        self.uv1 = uv1.into().into();
        self
    }
    /// Sets the background color (default: no background color)
    pub fn background_col(mut self, bg_col: impl Into<MintVec4>) -> Self {
        self.bg_col = bg_col.into().into();
        self
    }
    /// Sets the tint color (default: no tint color)
    pub fn tint_col(mut self, tint_col: impl Into<MintVec4>) -> Self {
        self.tint_col = tint_col.into().into();
        self
    }
    /// Sets the frame padding (default: uses frame padding from style).
    ///
    /// - `< 0`: uses frame padding from style (default)
    /// - `= 0`: no framing
    /// - `> 0`: set framing size
    pub fn frame_padding(mut self, frame_padding: i32) -> Self {
        self.frame_padding = frame_padding;
        self
    }
    /// Builds the image button
    pub fn build(self, _: &Ui) -> bool {
        unsafe {
            sys::igPushID_Ptr(self.texture_id.id() as *const _);

            if self.frame_padding >= 0 {
                sys::igPushStyleVar_Vec2(
                    sys::ImGuiStyleVar_FramePadding as i32,
                    [self.frame_padding as f32, self.frame_padding as f32].into(),
                );
            }

            let res = sys::igImageButton(
                b"#image".as_ptr().cast(),
                self.texture_id.id() as *mut c_void,
                self.size.into(),
                self.uv0.into(),
                self.uv1.into(),
                self.bg_col.into(),
                self.tint_col.into(),
            );

            if self.frame_padding >= 0 {
                sys::igPopStyleVar(1);
            }

            sys::igPopID();

            res
        }
    }
}

impl Ui {
    pub fn image_button(
        &self,
        str_id: impl AsRef<str>,
        texture_id: TextureId,
        size: impl Into<MintVec2>,
    ) -> bool {
        ImageButton {
            str_id,
            texture_id,
            size: size.into().into(),
            uv0: [0.0, 0.0],
            uv1: [1.0, 1.0],
            bg_col: [0.0, 0.0, 0.0, 0.0],
            tint_col: [1.0, 1.0, 1.0, 1.0],
            ui: self,
        }
        .build()
    }

    pub fn image_button_config<IdStr: AsRef<str>>(
        &self,
        str_id: IdStr,
        texture_id: TextureId,
        size: impl Into<MintVec2>,
    ) -> ImageButton<'_, IdStr> {
        ImageButton {
            str_id,
            texture_id,
            size: size.into().into(),
            uv0: [0.0, 0.0],
            uv1: [1.0, 1.0],
            bg_col: [0.0, 0.0, 0.0, 0.0],
            tint_col: [1.0, 1.0, 1.0, 1.0],
            ui: self,
        }
    }
}
