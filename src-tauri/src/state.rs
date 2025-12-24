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
}
