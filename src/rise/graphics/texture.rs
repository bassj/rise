use std::fmt;

#[derive(Debug, Clone)]
struct TextureError {}

impl std::error::Error for TextureError {}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error creating texture")
    }
}

pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler
}

impl Texture {
    
    pub fn get_view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn get_texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn get_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

pub struct TextureBuilder<'a> {
    render_context: &'a crate::graphics::RenderContext,
    texture_desc: wgpu::TextureDescriptor<'a>,
    sampler_desc: wgpu::SamplerDescriptor<'a>
}

impl<'a> TextureBuilder<'a> {
    pub fn new(r: &'a crate::graphics::RenderContext) -> Self {
        let size =  wgpu::Extent3d {
            width: 0,
            height: 0,
            depth: 1
        };
        
        let texture_desc = wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT // 3.
                | wgpu::TextureUsage::SAMPLED 
                | wgpu::TextureUsage::COPY_SRC,
        };

        let sampler_desc = wgpu::SamplerDescriptor { // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(wgpu::CompareFunction::LessEqual), // 5.
            ..Default::default()
        };

        TextureBuilder {
            render_context: r,
            texture_desc,
            sampler_desc
        }
    }

    pub fn make_depth_texture(self) -> Self {
        let width = self.render_context.sc_desc.width;
        let height = self.render_context.sc_desc.height;

        self.with_size(width, height)
            .with_format(wgpu::TextureFormat::Depth32Float)
            .with_usage(wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::OUTPUT_ATTACHMENT)
            .with_label("Depth Texture")
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.texture_desc.size.width = width;
        self.texture_desc.size.height = height;
        self
    }

    pub fn with_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.texture_desc.format = format;
        self
    }

    pub fn with_usage(mut self, usage: wgpu::TextureUsage) -> Self {
        self.texture_desc.usage = usage;
        self
    }

    pub fn with_label(mut self, label: &'static str) -> Self {
        self.texture_desc.label = Some(label);
        self
    }

    pub fn load_file<P: AsRef<std::path::Path>>(mut self, path: P) -> Result<Texture, Box<dyn std::error::Error>> {
        let diffuse = image::open(path)?;

        let diffuse_rgba = diffuse.to_rgba();

        let dimensions = diffuse_rgba.dimensions();

        let render_context = self.render_context;

        self.sampler_desc = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };

        self.texture_desc = wgpu::TextureDescriptor {
            // All textures are stored as 3d, we represent our 2d texture
            // by setting depth to 1.
            size: wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth: 1,
            },
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            // SAMPLED tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("Image Texture"),
        };

        let size = self.texture_desc.size;

        let texture = self.build();

        render_context.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::TextureCopyView {
                texture: texture.get_texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            // The actual pixel data
            &diffuse_rgba,
            // The layout of the texture
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        Ok(texture)
    }

    pub fn build(self) -> Texture {

        let texture = self.render_context.device.create_texture(&self.texture_desc);
    
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = self.render_context.device.create_sampler(&self.sampler_desc);

        Texture {
            view,
            sampler,
            texture
        }
    }
}