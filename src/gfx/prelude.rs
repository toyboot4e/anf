//! FNA types

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayMode {
    pub format: fna3d::SurfaceFormat,
    // u32??
    pub width: u32,
    pub height: u32,
}

/// Defines a set of graphic capabilities.
pub enum GraphicsProfile {
    /// Use a limited set of graphic features and capabilities, allowing the game to support the widest variety of devices.
    Reach,
    /// Use the largest available set of graphic features and capabilities to target devices, that have more enhanced graphic capabilities.
    HiDef,
}

impl DisplayMode {
    // fn aspect_ratio()
    // pub fn title_safe_area(&self) -> Rect
}

pub struct DisplayModeCollection {
    modes: Vec<DisplayMode>,
}

impl DisplayModeCollection {
    pub fn fmt2modes(&self, format: fna3d::SurfaceFormat) -> Vec<DisplayMode> {
        let mut res = Vec::new();
        for mode in self.modes.iter() {
            if mode.format == format {
                res.push(mode.clone()); // TODO: should it not be cloned?
            }
        }
        res
    }
}

pub struct GraphicsAdapter {
    // id: DeviceId,
    supported_display_modes: DisplayModeCollection,
    adapters: Vec<GraphicsAdapter>,
}

impl GraphicsAdapter {
    // Gets a bool indicating whether `current_display_mode  has a Width:Height ratio corresponding
    // to a widescreen `DisplayMode` Common widescreen modes include 16:9, 16:10 and 2:1.
    // public bool IsWideScreen
    // {
    //     get
    //     {
    //         /* Common non-widescreen modes: 4:3, 5:4, 1:1
    //          * Common widescreen modes: 16:9, 16:10, 2:1
    //          * XNA does not appear to account for rotated displays on the desktop
    //          */
    //         const float limit = 4.0f / 3.0f;
    //         float aspect = CurrentDisplayMode.AspectRatio;
    //         return aspect > limit;
    //     }
    // }

    // internal GraphicsAdapter(
    //     DisplayModeCollection modes,
    //     string name,
    //     string description
    // ) {
    //     SupportedDisplayModes = modes;
    //     DeviceName = name;
    //     Description = description;
    //     UseNullDevice = false;
    //     UseReferenceDevice = false;
    // }

    pub fn is_profile_supported(prof: GraphicsProfile) -> bool {
        /* TODO: This method could be genuinely useful!
         * Maybe look into the difference between Reach/HiDef and add the
         * appropriate properties to the GLDevice.
         * -flibit
         */
        return true;
    }

    pub fn QueryRenderTargetFormat(
        prof: GraphicsProfile,
        format: fna3d::SurfaceFormat,
        depthFormat: fna3d::DepthFormat,
        multiSampleCount: usize,
        // out SurfaceFormat selectedFormat,
        // out DepthFormat selectedDepthFormat,
        // out int selectedMultiSampleCount
    ) -> (bool, fna3d::SurfaceFormat, fna3d::DepthFormat, usize) {
        /* Per the OpenGL 3.0 Specification, section 3.9.1,
         * under "Required Texture Formats". These are the
         * formats required for renderbuffer support.
         *
         * TODO: Per the 4.5 Specification, section 8.5.1,
         * RGB565, RGB5_A1, RGBA4 are also supported.
         * -flibit
         */
        let new_fmt = if !matches!(
            format,
            fna3d::SurfaceFormat::Color
                | fna3d::SurfaceFormat::Rgba1010102
                | fna3d::SurfaceFormat::Rg32
                | fna3d::SurfaceFormat::Rgba64
                | fna3d::SurfaceFormat::Single
                | fna3d::SurfaceFormat::Vector2
                | fna3d::SurfaceFormat::Vector4
                | fna3d::SurfaceFormat::HalfSingle
                | fna3d::SurfaceFormat::HalfVector2
                | fna3d::SurfaceFormat::HalfVector4
                | fna3d::SurfaceFormat::HdrBlendable
        ) {
            format
        } else {
            fna3d::SurfaceFormat::Color
        };
        let new_depth = depthFormat;
        let new_count = 0; // Okay, sure, sorry.

        let b = format == new_fmt && depthFormat == new_depth && multiSampleCount == new_count;
        (b, new_fmt, new_depth, new_count)
    }

    pub fn query_back_buffer_format(
        prof: GraphicsProfile,
        format: fna3d::SurfaceFormat,
        depthFormat: fna3d::DepthFormat,
        multiSampleCount: usize,
    ) -> (bool, fna3d::SurfaceFormat, fna3d::DepthFormat, usize) {
        let new_fmt = fna3d::SurfaceFormat::Color; // Seriously?
        let new_depth = depthFormat;
        let new_count = 0; // Okay, sure, sorry.

        let b = format == new_fmt && depthFormat == new_depth && multiSampleCount == new_count;

        (b, new_fmt, new_depth, new_count)
    }
}
