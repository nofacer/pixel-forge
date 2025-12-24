use std::sync::Mutex;
use wgpu::{Adapter, Device, Instance, Queue};

pub struct RenderContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

pub struct AppState {
    pub render_context: Mutex<Option<RenderContext>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            render_context: Mutex::new(None),
        }
    }

    pub async fn initialize(&self) -> Result<String, String> {
        let instance = Instance::default();

        // Request an adapter (physical device)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None, // We are doing offscreen rendering mostly
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find an appropriate adapter")?;

        // Request a device (logical device) and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Pixel Forge Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to create device: {}", e))?;

        let info = format!("Initialized WebGPU on: {:?}", adapter.get_info().name);

        let context = RenderContext {
            instance,
            adapter,
            device,
            queue,
        };

        *self.render_context.lock().unwrap() = Some(context);

        Ok(info)
    }

    pub async fn render(&self, graph: crate::graph::Graph) -> Result<Vec<u8>, String> {
        let mut context_guard = self.render_context.lock().map_err(|e| e.to_string())?;
        let context = context_guard.as_mut().ok_or("WebGPU not initialized")?;

        // 1. Determine "Output" color from the graph
        // For MVP: Find the first Node connected to an Output Node, or just find a ColorNode
        let mut clear_color = wgpu::Color::BLACK;

        // Simple logic: Find first color node
        if let Some(color_node) = graph.nodes.iter().find(|n| n.node_type == "colorNode") {
            if let Some(c) = &color_node.data.color {
                clear_color = wgpu::Color {
                    r: c.r as f64 / 255.0,
                    g: c.g as f64 / 255.0,
                    b: c.b as f64 / 255.0,
                    a: c.a as f64,
                };
            }
        }

        // 2. Create a texture to render to
        let texture_size = 256u32;
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size,
                height: texture_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Output Texture"),
            view_formats: &[],
        };
        let texture = context.device.create_texture(&texture_desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 3. Render Pass (Clear to color)
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // 4. Readback buffer
        let u32_size = std::mem::size_of::<u32>() as u32;
        let output_buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("Output Buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = context.device.create_buffer(&output_buffer_desc);

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * texture_size),
                    rows_per_image: Some(texture_size),
                },
            },
            texture_desc.size,
        );

        println!("Submitting work...");
        context.queue.submit(Some(encoder.finish()));

        // 5. Map buffer and read
        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();

        println!("Mapping buffer...");
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            println!("Map callback fired!");
            let _ = tx.send(result);
        });

        println!("Polling device...");
        context.device.poll(wgpu::Maintain::Wait);
        println!("Device polled. Waiting for channel...");

        rx.recv().unwrap().map_err(|e| e.to_string())?;
        println!("Channel received.");

        let data = buffer_slice.get_mapped_range();
        let result = data.to_vec();
        drop(data);
        output_buffer.unmap();

        Ok(result)
    }
}
