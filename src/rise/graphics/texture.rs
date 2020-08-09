pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler
}

impl Texture {
    pub fn create_depth_texture(render_context: &crate::graphics::RenderContext) -> Self {

        let size = wgpu::Extent3d { // 2.
            width: render_context.sc_desc.width,
            height: render_context.sc_desc.height,
            depth: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT // 3.
                | wgpu::TextureUsage::SAMPLED 
                | wgpu::TextureUsage::COPY_SRC,
        };
        let texture = render_context.device.create_texture(&desc);

        let view = texture.create_default_view();
        let sampler = render_context.device.create_sampler(&wgpu::SamplerDescriptor { // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::LessEqual, // 5.
        });

        
        Self {
            texture,
            view,
            sampler
        }
    }

    pub fn get_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}