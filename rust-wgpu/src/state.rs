// use wgpu::include_wgsl;

// pub struct State {
//     pub surface: wgpu::Surface,
//     pub device: wgpu::Device,
//     pub queue: wgpu::Queue,
//     pub config: wgpu::SurfaceConfiguration,
//     pub render_pipeline: wgpu::RenderPipeline,

//     pub size: winit::dpi::PhysicalSize<u32>,
// }

// impl State {
//     // Creating some of the wgpu types requires async code
//     pub async fn new(window: &glfw::Window) -> Self {
//         todo!()
//         let size = window.inner_size();
//         let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), dx12_shader_compiler: wgpu::Dx12Compiler::Fxc });

//         for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
//             println!("{:?}", adapter.get_info())
//         }

//         let surface = unsafe { instance.create_surface(window).unwrap() };
//         let adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::HighPerformance,
//                 compatible_surface: Some(&surface),
//                 force_fallback_adapter: false,
//             })
//             .await
//             .unwrap();

//         let (device, queue) = adapter
//             .request_device(
//                 &wgpu::DeviceDescriptor {
//                     features: wgpu::Features::empty(),
//                     limits: wgpu::Limits::default(),
//                     label: None,
//                 },
//                 None, // Trace path
//             )
//             .await
//             .unwrap();

//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface.get_preferred_format(&adapter).unwrap(),
//             width: size.width,
//             height: size.height,
//             // present_mode: wgpu::PresentMode::Immediate,
//             present_mode: wgpu::PresentMode::Immediate,
//         };

//         surface.configure(&device, &config);

//         let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));

//         let render_pipeline_layout =
//             device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//                 label: Some("Render Pipeline Layout"),
//                 bind_group_layouts: &[],
//                 push_constant_ranges: &[],
//             });

//         let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: Some("Render Pipeline"),
//             layout: Some(&render_pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &shader,
//                 entry_point: "vs_main", // 1.
//                 buffers: &[], // 2.
//             },
//             fragment: Some(wgpu::FragmentState { // 3.
//                 module: &shader,
//                 entry_point: "fs_main",
//                 targets: &[wgpu::ColorTargetState { // 4.
//                     format: config.format,
//                     blend: Some(wgpu::BlendState::REPLACE),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 }],
//             }),

//             primitive: wgpu::PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::TriangleList, // 1.
//                 strip_index_format: None,
//                 front_face: wgpu::FrontFace::Ccw, // 2.
//                 cull_mode: Some(wgpu::Face::Back),
//                 // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
//                 polygon_mode: wgpu::PolygonMode::Fill,
//                 // Requires Features::DEPTH_CLIP_CONTROL
//                 unclipped_depth: false,
//                 // Requires Features::CONSERVATIVE_RASTERIZATION
//                 conservative: false,
//             },
//             depth_stencil: None, // 1.
//             multisample: wgpu::MultisampleState {
//                 count: 1, // 2.
//                 mask: !0, // 3.
//                 alpha_to_coverage_enabled: false, // 4.
//             },
//             multiview: None, // 5.
//         });

//         Self {
//             surface,
//             device,
//             queue,
//             config,
//             size,
//             render_pipeline,
//         }
//     }

//     pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         if new_size.width > 0 && new_size.height > 0 {
//             self.size = new_size;
//             self.config.width = new_size.width;
//             self.config.height = new_size.height;
//             self.surface.configure(&self.device, &self.config);
//         }
//     }

//     pub fn input(&mut self) -> bool {
//         // if key == Key::Q && mods == &Modifiers::Control {
//         //     return false;
//         // }

//         false
//     }

//     pub fn update(&mut self) {
//         // todo!()
//     }

//     pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//         let output = self.surface.get_current_texture()?;
//         let view = output
//             .texture
//             .create_view(&wgpu::TextureViewDescriptor::default());
//         let mut encoder = self
//             .device
//             .create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                 label: Some("Render Encoder"),
//             });

//         {
//             let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: Some("Render Pass"),
//                 // color_attachments: &[wgpu::RenderPassColorAttachment {
//                 //     view: &view,
//                 //     resolve_target: None,
//                 //     ops: wgpu::Operations {
//                 //         load: wgpu::LoadOp::Clear(wgpu::Color {
//                 //             r: 1.0,
//                 //             g: 1.0,
//                 //             b: 0.3,
//                 //             a: 1.0,
//                 //         }),
//                 //         store: true,
                        
//                 //     },
//                 // }],
//                 color_attachments: &[None],
//                 depth_stencil_attachment: None,
//             });

//             render_pass.set_pipeline(&self.render_pipeline); // 2.
//             render_pass.draw(0..3, 0..1); // 3.
//         }

//         // submit will accept anything that implements IntoIter
//         self.queue.submit(std::iter::once(encoder.finish()));
//         output.present();

//         Ok(())
//     }
// }
