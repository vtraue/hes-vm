use bytemuck::*;
use core::sync;
use interpreter::{
    env::{Env, ExternalFunction},
    slow_vm::{InstanceError, LocalValue, RuntimeError, Vm},
};
use notify::Watcher;
use parser::reader::{BytecodeReader, ParserError, ValueType, is_wasm_bytecode};
use rand::{Rng, rngs::ThreadRng};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{self, Receiver, Sender},
    },
    time::{Duration, Instant},
};
use thiserror::Error;
use ultraviolet::Mat4;
use validator::validator::{
    ReadAndValidateError, ValidateResult, read_and_validate, read_and_validate_wat,
};
use wgpu::{
    PresentMode,
    util::{DeviceExt, RenderEncoder},
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    error::EventLoopError,
    event::{KeyEvent, WindowEvent},
    event_loop::{self, ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("Unable to load wasm file: {0}")]
    UnableToLoadFile(io::Error),
    #[error("Error while parsing file: {0}")]
    UnableToParseFile(#[from] ReadAndValidateError),

    #[error("Invalid wasm code fileformat. Expected either .wat source code or raw wasm: {0}")]
    InvalidFileFormat(ParserError),

    #[error("Unable to read wat source code: {0}")]
    UnableToReadSourceCode(#[from] io::Error),

    #[error("Module required to export at least one run function.")]
    NoExportedFuncs,

    #[error("Module required to export a run function")]
    NoRunFunc,

    #[error("Module required to export an init function")]
    NoInitFunc,
    #[error("Unable to set up file watcher")]
    UnableToSetUpFileWatcher(#[from] notify::Error),

    #[error("Runtime error occured: {0}")]
    RuntimeError(#[from] RuntimeError),

    #[error("Unable to virtual machine: {0}")]
    UnableToInitVirtualMachine(#[from] InstanceError),
    #[error("Unable to create wgpu surface: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),

    #[error("Unable to request wgpu adapter: {0}")]
    RequestAdapter(#[from] wgpu::RequestAdapterError),

    #[error("Unable to request wgpu device: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("Unable to create event loop: {0}")]
    UnableToCreateEventLoop(#[from] EventLoopError),
}
#[derive(Debug)]
pub enum ConsoleKey {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    X,
    Y,
    R,
    L,
}
impl From<ConsoleKey> for u32 {
    fn from(value: ConsoleKey) -> Self {
        match value {
            ConsoleKey::Up => 0,
            ConsoleKey::Down => 1,
            ConsoleKey::Left => 2,
            ConsoleKey::Right => 3,
            ConsoleKey::A => 4,
            ConsoleKey::B => 5,
            ConsoleKey::X => 6,
            ConsoleKey::Y => 7,
            ConsoleKey::R => 8,
            ConsoleKey::L => 9,
        }
    }
}
impl ConsoleKey {
    pub fn from_winit_key(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::ArrowUp | KeyCode::KeyW => Some(ConsoleKey::Up),
            KeyCode::ArrowLeft | KeyCode::KeyA => Some(ConsoleKey::Left),
            KeyCode::ArrowRight | KeyCode::KeyD => Some(ConsoleKey::Right),
            KeyCode::ArrowDown | KeyCode::KeyS => Some(ConsoleKey::Down),
            KeyCode::KeyZ => Some(ConsoleKey::A),
            KeyCode::KeyB => Some(ConsoleKey::B),
            KeyCode::KeyX => Some(ConsoleKey::X),
            KeyCode::KeyY => Some(ConsoleKey::Y),
            KeyCode::KeyR => Some(ConsoleKey::R),
            KeyCode::KeyL => Some(ConsoleKey::L),
            _ => None,
        }
    }
}

/*
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
*/
/*
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
];
const INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];
*/
const FB_SIZE: (u32, u32) = (640, 360);
#[derive(Debug)]
struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    uniform_buffer: wgpu::Buffer,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    //index_buffer: wgpu::Buffer,
    //num_vertices: u32,
    //num_indices: u32,
    texture: wgpu::Texture,
    texture_size: wgpu::Extent3d,
    diffuse_bind_group: wgpu::BindGroup,
    start_time: Instant,
    rng: ThreadRng,
    clip_rect: (u32, u32, u32, u32),
}

impl State {
    pub async fn new(window: Arc<Window>) -> Result<Self, ConsoleError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        assert!(size.width > 0 && size.height > 0);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let matrix = ScalingMatrix::new(
            FB_SIZE.0 as f32,
            FB_SIZE.1 as f32,
            size.width as f32,
            size.height as f32,
        );
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("scaling matrix uniform"),
            contents: matrix.uniform_buffer.as_slice(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let vertices: [[f32; 2]; 3] = [[-1.0, -1.0], [3.0, -1.0], [-1.0, 3.0]];
        let vertices_slice = bytemuck::cast_slice(&vertices);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: vertices_slice,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: (vertices_slice.len() / vertices.len()) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            }],
        };

        let diffuse_bytes = include_bytes!("logo2.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: FB_SIZE.0,
            height: FB_SIZE.1,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("frame texture"),
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * FB_SIZE.0),
                rows_per_image: Some(FB_SIZE.1),
            },
            texture_size,
        );
        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 1.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                matrix.uniform_buffer.len() as u64
                            ),
                        },
                        count: None,
                    },
                ],
            });
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("diffuse_bind_group"),
        });
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
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

        /*
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;
        */
        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            vertex_buffer,
            //index_buffer,
            //num_vertices,
            //num_indices,
            diffuse_bind_group,
            texture: diffuse_texture,
            texture_size,
            start_time: Instant::now(),
            rng: rand::rng(),
            clip_rect: matrix.clip_rect(),
            uniform_buffer,
        })
    }

    pub fn update_framebuffer_data(&mut self, pixels: &[u8], width: u32, height: u32) {
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            self.texture_size,
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 && !self.is_surface_configured {
            let matrix = ScalingMatrix::new(
                FB_SIZE.0 as f32,
                FB_SIZE.1 as f32,
                width as f32,
                height as f32,
            );

            self.queue
                .write_buffer(&self.uniform_buffer, 0, &matrix.uniform_buffer);

            self.clip_rect = matrix.clip_rect();

            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true
        }
    }

    fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        //render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        //render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        render_pass.set_scissor_rect(
            self.clip_rect.0,
            self.clip_rect.1,
            self.clip_rect.2,
            self.clip_rect.3,
        );

        render_pass.draw(0..3, 0..1);
        drop(render_pass);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    pub fn fill_buffer_with_color(buffer: &mut [u8], r: u32, g: u32, b: u32, a: u32) {
        let buf: &mut [u32] = bytemuck::cast_slice_mut(buffer);
        let pixel = a << 24 | b << 16 | g << 8 | r;
        buf.fill(pixel);
    }

    pub fn draw_rectanlge_color(
        buffer: &mut [u8],
        pos_x: u32,
        pos_y: u32,
        width: u32,
        height: u32,
        r: u32,
        g: u32,
        b: u32,
        a: u32,
    ) {
        let pixel = a << 24 | b << 16 | g << 8 | r;
        for ele in (0..height) {
            let buffer_pos = (((pos_y + ele) * FB_SIZE.0 * 4) + (pos_x * 4)) as usize;
            let slice: &mut [u32] = bytemuck::cast_slice_mut(
                &mut buffer[buffer_pos..buffer_pos + (width as usize * 4)],
            );
            slice.fill(pixel);
        }
    }
}
impl Env for State {
    fn get_func(env: &str, name: &str) -> Option<ExternalFunction> {
        if env != "env" {
            return None;
        }
        //TODO:
        match name {
            "io_print_string" => Some(ExternalFunction {
                params: vec![ValueType::I32, ValueType::I32],
                result: vec![],
                id: 0,
            }),
            "gfx_paint" => Some(ExternalFunction {
                params: vec![ValueType::I32, ValueType::I32, ValueType::I32],
                result: vec![],
                id: 1,
            }),
            "io_print_sint" => Some(ExternalFunction {
                params: vec![ValueType::I32],
                result: vec![],
                id: 2,
            }),
            "gfx_clear_buffer_rgb" => Some(ExternalFunction {
                params: vec![
                    ValueType::I32, //buffer
                    ValueType::I32, //r
                    ValueType::I32, //g
                    ValueType::I32, //b
                ],
                result: vec![],
                id: 3,
            }),
            "gfx_draw_rect_rgb" => Some(ExternalFunction {
                params: vec![
                    ValueType::I32, //buffer
                    ValueType::I32, //x
                    ValueType::I32, //y
                    ValueType::I32, //width
                    ValueType::I32, //height
                    ValueType::I32, //r
                    ValueType::I32, //g
                    ValueType::I32, //b
                ],
                result: vec![],
                id: 4,
            }),
            "clock_get_time_passed_ms" => Some(ExternalFunction {
                params: vec![],
                result: vec![ValueType::I64],
                id: 5,
            }),

            "io_print_sint64" => Some(ExternalFunction {
                params: vec![ValueType::I64],
                result: vec![],
                id: 6,
            }),
            "rand_range_sint32" => Some(ExternalFunction {
                params: vec![ValueType::I32, ValueType::I32],
                result: vec![ValueType::I32],
                id: 7,
            }),
            _ => None,
        }
    }

    fn get_global(env: &str, name: &str) -> Option<interpreter::env::ExternalGlobal> {
        None
    }

    fn call(
        &mut self,
        vm: &mut Vm<Self>,
        params: &[LocalValue],
        results: &mut [LocalValue],
        func_id: usize,
    ) -> Result<(), usize> {
        match func_id {
            0 => {
                let ptr = params[0].u32();
                let count = params[1].u32();
                let data = vm
                    .get_bytes_from_mem(ptr as usize, count as usize)
                    .map_err(|_| 1_usize)?;
                let str = str::from_utf8(data).map_err(|_| 2_usize)?;
                print!("{str}");
                Ok(())
            }
            //paint
            1 => {
                let (ptr, width, height) = (params[0].u32(), params[1].u32(), params[2].u32());
                let data = vm
                    .get_bytes_from_mem(ptr as usize, (width * height * 4) as usize)
                    .map_err(|_| 1_usize)?;
                self.update_framebuffer_data(data, width, height);

                Ok(())
            }
            2 => Ok(println!("{}", params[0].i32())),

            //gfx_fill_buffer
            3 => {
                let ptr = params[0].u32();
                let (r, g, b, a) = (params[1].u32(), params[2].u32(), params[3].u32(), 0);
                let data = vm
                    .get_bytes_from_mem_mut(ptr as usize, (FB_SIZE.0 * FB_SIZE.1 * 4) as usize)
                    .map_err(|_| 1_usize)?;
                Self::fill_buffer_with_color(data, r, g, b, a);
                Ok(())
            }
            4 => {
                let ptr = params[0].u32();
                let (x, y, w, h, r, g, b, a) = (
                    params[1].u32(),
                    params[2].u32(),
                    params[3].u32(),
                    params[4].u32(),
                    params[5].u32(),
                    params[6].u32(),
                    params[7].u32(),
                    0,
                );
                let data = vm
                    .get_bytes_from_mem_mut(ptr as usize, (FB_SIZE.0 * FB_SIZE.1 * 4) as usize)
                    .map_err(|_| 1_usize)?;

                Self::draw_rectanlge_color(data, x, y, w, h, r, g, b, a);
                Ok(())
            }
            5 => {
                let duration = self.start_time.elapsed().as_millis();
                results[0] = LocalValue::I64(duration as u64);

                Ok(())
            }
            6 => Ok(println!("{}", params[0].i64())),
            7 => {
                let num = self.rng.random_range(params[0].i32()..params[1].i32());

                results[0] = LocalValue::S32(num as i32);
                Ok(())
            }

            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Funcs {
    init: usize,
    run: usize,
    input: Option<usize>,
}
impl Funcs {
    pub fn from_validate_result(result: &ValidateResult) -> Result<Self, ConsoleError> {
        let exports = result
            .bytecode
            .get_exports_as_map()
            .ok_or(ConsoleError::NoExportedFuncs)?;

        let run = exports
            .get_function_id("run")
            .ok_or(ConsoleError::NoRunFunc)?;
        let init = exports
            .get_function_id("init")
            .ok_or(ConsoleError::NoInitFunc)?;
        let input = exports.get_function_id("input");
        Ok(Self { init, run, input })
    }
}
#[derive(Debug)]
pub struct Executor {
    wasm_path: PathBuf,
    validate_result: ValidateResult,
    funcs: Funcs,
    init_func_result: Option<u32>,
    vm: Vm<State>,
}

impl Executor {
    //NOTE: (joh): Vielleicht sollten wir direkt Bytecode uebergeben?
    fn get_validate_result(
        reader: &mut impl BytecodeReader,
    ) -> Result<ValidateResult, ConsoleError> {
        let res = if is_wasm_bytecode(reader).map_err(|e| ConsoleError::InvalidFileFormat(e))? {
            read_and_validate(reader)
        } else {
            let mut code = String::new();
            reader.read_to_string(&mut code)?;
            read_and_validate_wat(code)
        }?;
        Ok(res)
    }

    pub fn new(path: PathBuf) -> Result<Self, ConsoleError> {
        let file = File::open(&path).map_err(|e| ConsoleError::UnableToLoadFile(e))?;
        let mut reader = BufReader::new(file);

        let validate_result = Self::get_validate_result(&mut reader)?;
        let funcs = Funcs::from_validate_result(&validate_result)?;

        let vm = Vm::init_from_validation_result(&validate_result)?;

        Ok(Executor {
            wasm_path: path,
            vm,
            validate_result,
            funcs,
            init_func_result: None,
        })
    }

    pub fn reload_all(&mut self, state: &mut State) -> Result<(), ConsoleError> {
        let file = File::open(&self.wasm_path).map_err(|e| ConsoleError::UnableToLoadFile(e))?;
        let mut reader = BufReader::new(file);
        self.validate_result = Self::get_validate_result(&mut reader)?;
        self.funcs = Funcs::from_validate_result(&self.validate_result)?;
        self.vm = Vm::init_from_validation_result(&self.validate_result)?;
        self.run_init(state)?;
        Ok(())
    }

    pub fn reload_code(&mut self, state: &mut State) -> Result<(), ConsoleError> {
        let file = File::open(&self.wasm_path).map_err(|e| ConsoleError::UnableToLoadFile(e))?;
        let mut reader = BufReader::new(file);
        self.validate_result = Self::get_validate_result(&mut reader)?;
        self.funcs = Funcs::from_validate_result(&self.validate_result)?;
        self.vm
            .reload_code(&self.validate_result.bytecode, &self.validate_result.info)?;
        Ok(())
    }

    fn run_init(&mut self, state: &mut State) -> Result<(), ConsoleError> {
        self.vm.set_func(self.funcs.init, vec![])?;

        let result = self.vm.run_func(
            &self.validate_result.bytecode,
            &self.validate_result.info,
            state,
        )?;

        println!("Init done!\n");
        assert!(result.len() == 1);
        self.init_func_result = Some(result[0].u32());

        Ok(())
    }

    fn run_frame(
        &mut self,
        state: &mut State,
        width: u32,
        height: u32,
    ) -> Result<(), RuntimeError> {
        let args: [LocalValue; 3] = [
            LocalValue::I32(self.init_func_result.unwrap()),
            LocalValue::I32(width),
            LocalValue::I32(height),
        ];
        self.vm.set_func(self.funcs.run, args)?;
        self.vm.run_func(
            &self.validate_result.bytecode,
            &self.validate_result.info,
            state,
        )?;
        Ok(())
    }

    fn run_input(
        &mut self,
        state: &mut State,
        key: ConsoleKey,
        pressed: bool,
    ) -> Result<(), RuntimeError> {
        let args: [LocalValue; 3] = [
            LocalValue::I32(self.init_func_result.unwrap()),
            LocalValue::I32(key.into()),
            LocalValue::I32(pressed.into()),
        ];
        self.vm.set_func(self.funcs.input.unwrap(), args)?;
        self.vm.run_func(
            &self.validate_result.bytecode,
            &self.validate_result.info,
            state,
        )?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct App {
    state: Option<State>,
    exec: Executor,
    auto_hot_reload: bool,
    file_watch_rec: Receiver<notify::Result<notify::Event>>,
    file_watcher: notify::PollWatcher,
}
impl App {
    pub fn new(path: PathBuf, auto_hot_reload: bool) -> Result<Self, ConsoleError> {
        let (file_watch_sender, file_watch_rec) = mpsc::channel();
        let config = notify::Config::default()
            .with_poll_interval(Duration::from_secs(2))
            .with_compare_contents(true);

        let mut file_watcher = notify::PollWatcher::new(file_watch_sender, config)?;

        file_watcher.watch(&path, notify::RecursiveMode::Recursive)?;
        let exec = Executor::new(path)?;
        let app = Self {
            state: None,
            exec,
            auto_hot_reload,
            file_watch_rec,
            file_watcher,
        };

        Ok(app)
    }
    pub fn run(path: PathBuf) -> Result<(), ConsoleError> {
        let event_loop = EventLoop::with_user_event().build()?;
        let mut app = App::new(path, true)?;
        event_loop.run_app(&mut app).unwrap();
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes =
            Window::default_attributes().with_inner_size(LogicalSize::new(FB_SIZE.0, FB_SIZE.1));
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        let mut state = pollster::block_on(State::new(window)).unwrap();

        self.exec.run_init(&mut state).unwrap();
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Stopping...");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                let state = self.state.as_mut().unwrap();
                state.resize(size.width, size.height)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => match (code, key_state.is_pressed()) {
                (KeyCode::KeyR, true) => {
                    let state = self.state.as_mut().unwrap();
                    self.exec.reload_all(state).unwrap();
                }
                (KeyCode::KeyT, true) => {
                    let state = self.state.as_mut().unwrap();
                    self.exec.reload_code(state).unwrap();
                }

                (key, pressed) => {
                    let state = self.state.as_mut().unwrap();

                    match self.exec.funcs.input {
                        Some(_) => {
                            if let Some(k) = ConsoleKey::from_winit_key(code) {
                                self.exec.run_input(state, k, pressed).unwrap();
                            }
                        }
                        None => todo!(),
                    }
                    state.handle_key(event_loop, code, key_state.is_pressed());
                }
            },
            WindowEvent::RedrawRequested => {
                //TODO: (joh): Besseres Error Handling
                //println!("start frame!");
                use std::time::Instant;
                let now = Instant::now();

                let state = match &mut self.state {
                    Some(canvas) => canvas,
                    None => return,
                };
                for ev in self.file_watch_rec.try_iter() {
                    match ev {
                        Ok(e) => match e.kind {
                            notify::EventKind::Modify(_) => {
                                println!("blub!\n");
                                if self.auto_hot_reload {
                                    self.exec.reload_code(state).unwrap();
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }

                self.exec.run_frame(state, FB_SIZE.0, FB_SIZE.1).unwrap();

                let elapsed = now.elapsed();
                //println!("Time for update: {:.2?}ms", elapsed.as_millis());

                let size = state.window.inner_size();

                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        panic!("Unable to render {}", e);
                    }
                }
            }

            _ => {}
        }
    }
}

//Von: https://github.com/parasyte/pixels
pub struct ScalingMatrix {
    transform: Mat4,
    clip_rect: (u32, u32, u32, u32),
    uniform_buffer: Vec<u8>,
}
impl ScalingMatrix {
    pub fn new(
        texture_width: f32,
        texture_height: f32,
        screen_width: f32,
        screen_height: f32,
    ) -> Self {
        let width_ratio = (screen_width / texture_width);
        let height_ratio = (screen_height / texture_height);
        let scale = width_ratio.min(height_ratio);
        let scaled_width = texture_width * scale;
        let scaled_height = texture_height * scale;
        let sw = scaled_width / screen_width;
        let sh = scaled_height / screen_height;
        let tx = (screen_width / 2.0).fract() / screen_width;
        let ty = (screen_height / 2.0).fract() / scaled_height;
        #[rustfmt::skip]
        let transform: [f32; 16] = [
            sw,  0.0, 0.0, 0.0,
            0.0, sh,  0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            tx,  ty,  0.0, 1.0,
        ];

        // Create a clipping rectangle
        let clip_rect = {
            let scaled_width = scaled_width.min(screen_width);
            let scaled_height = scaled_height.min(screen_height);
            let x = ((screen_width - scaled_width) / 2.0) as u32;
            let y = ((screen_height - scaled_height) / 2.0) as u32;

            (x, y, scaled_width as u32, scaled_height as u32)
        };

        let mat = Mat4::from(transform);

        // Compute the constant buffer
        let mut uniform_buffer = Vec::new();
        uniform_buffer.extend_from_slice(mat.as_byte_slice());
        uniform_buffer.extend_from_slice(&texture_width.to_le_bytes());
        uniform_buffer.extend_from_slice(&texture_height.to_le_bytes());
        uniform_buffer.extend_from_slice(&(1.0 / texture_width).to_le_bytes());
        uniform_buffer.extend_from_slice(&(1.0 / texture_height).to_le_bytes());

        Self {
            transform: mat,
            clip_rect,
            uniform_buffer,
        }
    }

    pub(crate) fn clip_rect(&self) -> (u32, u32, u32, u32) {
        self.clip_rect
    }
}
