use std::sync::Mutex;
use wgpu::util::DeviceExt;
use wgpu::{Adapter, Device, Instance, Queue};

pub struct RenderContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub mix_pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
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

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find an appropriate adapter")?;

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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Mix Shader"),
            source: wgpu::ShaderSource::Wgsl(crate::shaders::MIX_SHADER.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Mix Bind Group Layout"),
            entries: &[
                // Texture A
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Texture B
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Uniforms (Factor)
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Mix Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let mix_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Mix Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let info = format!("Initialized WebGPU on: {:?}", adapter.get_info().name);

        let context = RenderContext {
            instance,
            adapter,
            device,
            queue,
            mix_pipeline,
            bind_group_layout,
        };

        *self.render_context.lock().unwrap() = Some(context);

        Ok(info)
    }

    pub async fn render(&self, graph: crate::graph::Graph) -> Result<Vec<u8>, String> {
        let mut context_guard = self.render_context.lock().map_err(|e| e.to_string())?;
        let context = context_guard.as_mut().ok_or("WebGPU not initialized")?;

        let output_node = graph
            .nodes
            .iter()
            .find(|n| n.node_type == "outputNode")
            .ok_or("No output node found")?;

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut textures = Vec::new();

        struct Evaluator<'a> {
            context: &'a RenderContext,
            encoder: &'a mut wgpu::CommandEncoder,
            graph: &'a crate::graph::Graph,
            textures: &'a mut Vec<wgpu::Texture>,
        }

        impl<'a> Evaluator<'a> {
            fn create_texture(&mut self, label: &str) -> wgpu::TextureView {
                let texture = self
                    .context
                    .device
                    .create_texture(&wgpu::TextureDescriptor {
                        size: wgpu::Extent3d {
                            width: 256,
                            height: 256,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING
                            | wgpu::TextureUsages::RENDER_ATTACHMENT
                            | wgpu::TextureUsages::COPY_SRC,
                        label: Some(label),
                        view_formats: &[],
                    });
                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                self.textures.push(texture);
                view
            }

            fn evaluate(&mut self, node_id: &str) -> Result<wgpu::TextureView, String> {
                let node = self.graph.get_node(node_id).ok_or("Node not found")?;

                match node.node_type.as_str() {
                    "colorNode" => {
                        let color = node
                            .data
                            .color
                            .as_ref()
                            .map(|c| wgpu::Color {
                                r: c.r as f64 / 255.0,
                                g: c.g as f64 / 255.0,
                                b: c.b as f64 / 255.0,
                                a: c.a as f64,
                            })
                            .unwrap_or(wgpu::Color::BLACK);

                        let view = self.create_texture(&format!("Color {}", node.data.label));

                        {
                            let _pass =
                                self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Clear Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(color),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });
                        }
                        Ok(view)
                    }
                    "mixNode" => {
                        let edge_a = self.graph.edges.iter().find(|e| {
                            e.target == node.id && e.target_handle.as_deref() == Some("a")
                        });
                        let edge_b = self.graph.edges.iter().find(|e| {
                            e.target == node.id && e.target_handle.as_deref() == Some("b")
                        });

                        println!(
                            "MixNode '{}': Edge A found? {}, Edge B found? {}",
                            node.data.label,
                            edge_a.is_some(),
                            edge_b.is_some()
                        );

                        let view_a = if let Some(e) = edge_a {
                            self.evaluate(&e.source)?
                        } else {
                            self.create_black_texture()?
                        };

                        let view_b = if let Some(e) = edge_b {
                            self.evaluate(&e.source)?
                        } else {
                            self.create_black_texture()?
                        };

                        let output_view = self.create_texture(&format!("Mix {}", node.data.label));

                        let factor = node.data.factor.unwrap_or(0.5);
                        let uniform_data = [factor, 0.0, 0.0, 0.0]; // 16 bytes
                        let uniform_buffer = self.context.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("Mix Buffer"),
                                contents: bytemuck::cast_slice(&uniform_data),
                                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                            },
                        );

                        let sampler =
                            self.context
                                .device
                                .create_sampler(&wgpu::SamplerDescriptor {
                                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                                    mag_filter: wgpu::FilterMode::Linear,
                                    min_filter: wgpu::FilterMode::Linear,
                                    mipmap_filter: wgpu::FilterMode::Nearest,
                                    ..Default::default()
                                });

                        let bind_group =
                            self.context
                                .device
                                .create_bind_group(&wgpu::BindGroupDescriptor {
                                    label: Some("Mix Bind Group"),
                                    layout: &self.context.bind_group_layout,
                                    entries: &[
                                        wgpu::BindGroupEntry {
                                            binding: 0,
                                            resource: wgpu::BindingResource::TextureView(&view_a),
                                        },
                                        wgpu::BindGroupEntry {
                                            binding: 1,
                                            resource: wgpu::BindingResource::TextureView(&view_b),
                                        },
                                        wgpu::BindGroupEntry {
                                            binding: 2,
                                            resource: wgpu::BindingResource::Sampler(&sampler),
                                        },
                                        wgpu::BindGroupEntry {
                                            binding: 3,
                                            resource: wgpu::BindingResource::Buffer(
                                                wgpu::BufferBinding {
                                                    buffer: &uniform_buffer,
                                                    offset: 0,
                                                    size: None,
                                                },
                                            ),
                                        },
                                    ],
                                });

                        {
                            let mut pass =
                                self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                    label: Some("Mix Pass"),
                                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                        view: &output_view,
                                        resolve_target: None,
                                        ops: wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                            store: wgpu::StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            pass.set_pipeline(&self.context.mix_pipeline);
                            pass.set_bind_group(0, &bind_group, &[]);
                            pass.draw(0..3, 0..1);
                        }

                        Ok(output_view)
                    }
                    "outputNode" => {
                        let edge = self.graph.edges.iter().find(|e| e.target == node.id);
                        if let Some(e) = edge {
                            self.evaluate(&e.source)
                        } else {
                            self.create_black_texture()
                        }
                    }
                    _ => Err(format!("Unknown node type: {}", node.node_type)),
                }
            }

            fn create_black_texture(&mut self) -> Result<wgpu::TextureView, String> {
                let view = self.create_texture("Black Default");
                {
                    let _pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Clear Black Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                }
                Ok(view)
            }
        }

        let final_texture_view = {
            let mut evaluator = Evaluator {
                context: context,
                encoder: &mut encoder,
                graph: &graph,
                textures: &mut textures,
            };
            evaluator.evaluate(&output_node.id)?
        };

        // Render result to readback texture using Mix Pipeline (Copy)
        let result_texture = context.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Readback Texture"),
            view_formats: &[],
        });
        let result_view = result_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Dummy texture and view for the second input of simple copy (mix with factor 0)
        let dummy_tex = textures.last().unwrap();
        let dummy_view = dummy_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let factor = 0.0f32; // Take 100% of A (final_texture_view)
        let uniform_data = [factor, 0.0, 0.0, 0.0];
        let uniform_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Copy Buffer"),
                contents: bytemuck::cast_slice(&uniform_data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let sampler = context
            .device
            .create_sampler(&wgpu::SamplerDescriptor::default());

        let bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Copy Bind Group"),
                layout: &context.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&final_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&dummy_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &uniform_buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                ],
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Final Copy Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &result_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&context.mix_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        // Readback
        let u32_size = std::mem::size_of::<u32>() as u32;
        let output_buffer_size = (u32_size * 256 * 256) as wgpu::BufferAddress;
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
                texture: &result_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * 256),
                    rows_per_image: Some(256),
                },
            },
            wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
        );

        context.queue.submit(Some(encoder.finish()));

        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();

        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        context.device.poll(wgpu::Maintain::Wait);
        rx.recv().unwrap().map_err(|e| e.to_string())?;

        let data = buffer_slice.get_mapped_range();
        let result = data.to_vec();
        drop(data);
        output_buffer.unmap();

        Ok(result)
    }
}
