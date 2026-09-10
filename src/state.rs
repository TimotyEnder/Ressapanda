use crate::{
    brushes::brush::Brush,
    camera::{Camera, CameraLookDirection, CameraUniform},
    camera_controller::CameraController,
    color::VoxelColor,
    cursor_loader::{self, CursorLoader},
    depth_texture::DepthTexture,
    select_mode::{
        select_mode::{SelectMode, select_mode_from_name},
        single_select_mode::SingleSelectMode,
    },
    tools::{
        add::Add,
        key_input_manager::KeyInputManager,
        tool::{Tool, tool_from_name},
    },
    ui_data::UIData,
    vertex::{CUBE_INDICES, CUBE_VERTICES, Vertex},
    voxel_instance::RawVoxelInstance,
    voxel_scene::{VoxelScene, VoxelSceneDirection},
};
use anyhow::Ok;
use cgmath::Point3;
use egui::{Color32, Image, Panel, accesskit::Role::Label, epaint, load::SizedTexture, menu};
use egui_wgpu::RendererOptions;
use std::{iter, sync::Arc};
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta, TouchPhase},
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};
pub const TEXTURE_DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct State {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    pub config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    pub camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    pub voxel_scene: VoxelScene,
    voxel_instance_buffer: wgpu::Buffer,
    depth_texture: DepthTexture,
    camera_controller: CameraController,
    key_input_manager: KeyInputManager,
    current_tool: Box<dyn Tool>,
    current_select_mode: Box<dyn SelectMode>,
    last_mouse_position_recorded: PhysicalPosition<f64>,
    next_frame_instance_count: usize,
    current_brush: Brush,
    cursor_loader: CursorLoader,
    egui_ctx: egui::Context, // own clone, used for run_ui + tessellate
    pub egui_winit_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    egui_paint_jobs: Vec<epaint::ClippedPrimitive>,
    egui_textures_delta: egui::TexturesDelta,
    ui_info: UIData,
}
impl State {
    pub async fn new(
        window: Arc<Window>,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) -> anyhow::Result<State> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off, // Trace path
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(CUBE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let camera = Camera::new(
            (0.0, 5.0, 10.0).into(),
            (0.0, 0.0, 0.0).into(),
            cgmath::Vector3::unit_y(),
            config.width as f32 / config.height as f32,
            45.0,
            0.1,
            200.0,
        );
        let camera_controller = CameraController::new(0.01, &camera);
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let mut voxel_scene = VoxelScene::new();
        let voxel_instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(voxel_scene.prepare_buffer_contents()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let next_frame_instance_count = voxel_scene.get_voxel_instance_count();
        let depth_texture = DepthTexture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layour"),
                bind_group_layouts: &[Some(&camera_bind_group_layout)],
                immediate_size: 0,
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(Vertex::desc()), Some(RawVoxelInstance::desc())],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: TEXTURE_DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let tool_selector = KeyInputManager::new();
        let current_tool: Box<dyn Tool> = Box::new(Add {});
        let current_select_mode: Box<dyn SelectMode> = Box::new(SingleSelectMode::new());
        let current_brush = Brush {
            color: VoxelColor::default(),
        };

        let cursor_loader = CursorLoader::new(event_loop);

        //UI
        let egui_ctx = egui::Context::default();
        let egui_winit_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &*window, // &dyn HasDisplayHandle
            Some(window.scale_factor() as f32),
            window.theme(),
            Some(device.limits().max_texture_dimension_2d as usize),
        );
        let egui_renderer = egui_wgpu::Renderer::new(
            &device,
            config.format,
            egui_wgpu::RendererOptions::default(),
        );

        let mut visuals = egui::Visuals::dark();

        visuals.selection.bg_fill = egui::Color32::from_hex("#FFFFFF").unwrap();
        visuals.selection.stroke.color = egui::Color32::from_hex("#000000").unwrap();
        visuals.widgets.inactive.bg_stroke.color =
            egui::Color32::from_hex("#000000").unwrap_or_default();
        visuals.widgets.hovered.bg_stroke.color =
            egui::Color32::from_hex("#DB8758").unwrap_or_default();
        visuals.widgets.active.bg_stroke.color =
            egui::Color32::from_hex("#141414").unwrap_or_default();
        visuals.widgets.open.weak_bg_fill = egui::Color32::from_hex("#1a1a1a").unwrap_or_default();
        visuals.panel_fill = egui::Color32::from_hex("#3b3b3b").unwrap_or_default();
        visuals.window_fill = egui::Color32::from_hex("#3b3b3b").unwrap_or_default();
        visuals.override_text_color = Some(egui::Color32::WHITE);
        egui_ctx.set_theme(egui::Theme::Dark);
        egui_ctx.set_visuals(visuals);

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            vertex_buffer,
            index_buffer,
            render_pipeline,
            camera,
            camera_buffer,
            camera_uniform,
            camera_bind_group,
            voxel_instance_buffer,
            voxel_scene,
            depth_texture,
            camera_controller,
            key_input_manager: tool_selector,
            current_tool,
            current_select_mode,
            last_mouse_position_recorded: PhysicalPosition { x: 0.0, y: 0.0 },
            next_frame_instance_count,
            current_brush,
            cursor_loader,
            egui_winit_state,
            egui_renderer,
            egui_ctx: egui_ctx.clone(),
            egui_paint_jobs: Vec::new(),
            egui_textures_delta: egui::TexturesDelta::default(),
            ui_info: UIData::new(egui_ctx.clone()),
        })
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        self.key_input_manager.modifier_inputs(key, pressed);
        self.key_input_manager.camera_preset_positions_inputs(
            key,
            pressed,
            &mut self.camera,
            &mut self.camera_controller,
        );
        if let Some(tool) = self.key_input_manager.tool_selection_inputs(key, pressed) {
            self.current_tool = tool;
        }
        if let Some(selection_mode) = self
            .key_input_manager
            .select_mode_selection_inputs(key, pressed)
        {
            self.current_select_mode = selection_mode;
        }
        self.key_input_manager.move_commands_inputs(
            key,
            pressed,
            &mut self.voxel_scene,
            &self.camera,
        );
    }
    pub fn set_tool(&mut self, tool: Box<dyn Tool>) {
        self.current_tool = tool;
    }
    pub fn handle_mouse_button(
        &mut self,
        event_loop: &ActiveEventLoop,
        button: MouseButton,
        pressed: bool,
    ) {
        self.camera_controller.handle_mouse_button(button, pressed);

        if button == MouseButton::Left && pressed {
            self.current_select_mode.mouse_down(
                self.last_mouse_position_recorded.x,
                self.last_mouse_position_recorded.y,
                &self.camera,
                &mut self.voxel_scene,
                &self.config,
                &mut self.current_tool,
                &self.current_brush,
            );
        } else if button == MouseButton::Left && !pressed {
            self.current_select_mode.mouse_up(
                self.last_mouse_position_recorded.x,
                self.last_mouse_position_recorded.y,
                &self.camera,
                &mut self.voxel_scene,
                &self.config,
                &mut self.current_tool,
                &self.current_brush,
            );
        }
    }
    pub fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta, phase: TouchPhase) {
        self.camera_controller.handle_mouse_wheel(delta, phase);
    }
    pub fn handle_cursor_moved(&mut self, pos: PhysicalPosition<f64>) {
        self.camera_controller.handle_mouse_position(
            pos.x,
            pos.y,
            self.config.height as f32,
            &self.camera,
        );
        self.last_mouse_position_recorded = pos;
    }
    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.update_camera();
        self.update_temporary_voxel_generation_on_hover();
        self.update_voxel_buffers();
        self.ui_update();
        self.cursor_loader.change_cursor(
            self.window.clone(),
            &self.current_select_mode,
            &self.current_tool,
        );
    }
    fn ui_update(&mut self) {
        let raw_input = self.egui_winit_state.take_egui_input(&self.window);
        let egui_ctx = self.egui_ctx.clone(); // clone req'd: run_ui borrows ctx, closure borrows self
        let full_output = egui_ctx.run_ui(raw_input, |ui| self.ui(ui));

        self.egui_winit_state
            .handle_platform_output(&self.window, full_output.platform_output);
        self.egui_paint_jobs =
            egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
        self.egui_textures_delta = full_output.textures_delta;
    }
    fn update_temporary_voxel_generation_on_hover(&mut self) {
        self.current_select_mode.temp_draw_on_mouse_hover(
            self.last_mouse_position_recorded.x,
            self.last_mouse_position_recorded.y,
            &self.camera,
            &mut self.voxel_scene,
            &self.config,
            &mut self.current_tool,
            &self.current_brush,
        );
    }
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Skip this frame
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // You could recreate the devices and all resources
                // created with it here, but we'll just bail
                anyhow::bail!("Lost device :(");
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.078,
                            g: 0.078,
                            b: 0.078,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            render_pass.set_vertex_buffer(1, self.voxel_instance_buffer.slice(..));
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.draw_indexed(
                0..CUBE_INDICES.len() as u32,
                0,
                0..self.next_frame_instance_count as u32,
            );
        }

        //UI render
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };

        for (id, deltas) in &self.egui_textures_delta.set {
            for delta in deltas {
                self.egui_renderer
                    .update_texture(&self.device, &self.queue, *id, delta);
            }
        }

        let ui_cmd_buffers = self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &self.egui_paint_jobs,
            &screen_descriptor,
        );
        self.egui_textures_delta.clear();
        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // draw OVER the voxels, don't clear
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None, // egui must NOT depth-test vs voxels
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            self.egui_renderer.render(
                &mut render_pass.forget_lifetime(),
                &self.egui_paint_jobs,
                &screen_descriptor,
            );
        }

        self.queue
            .submit(iter::once(encoder.finish()).chain(ui_cmd_buffers));
        self.queue.present(output);
        return Ok(());
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.is_surface_configured = true;
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.camera.update_aspect(width as f32, height as f32);
            self.update_camera();
            self.depth_texture =
                DepthTexture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }
    fn update_camera(&mut self) {
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }
    fn update_voxel_buffers(&mut self) {
        if !self.voxel_scene.is_voxel_scene_changed() {
            return;
        }
        self.next_frame_instance_count = self.voxel_scene.get_voxel_instance_count();
        let bytes = bytemuck::cast_slice(self.voxel_scene.prepare_buffer_contents());
        if bytes.len() as u64 > self.voxel_instance_buffer.size() {
            self.voxel_instance_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytes,
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            self.queue
                .write_buffer(&self.voxel_instance_buffer, 0, bytes);
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui) {
        let version = env!("CARGO_PKG_VERSION");
        let top_panel_width = (self.config.height as f32 * 0.02).round();
        Panel::top("options_panel")
            .exact_size(top_panel_width)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("Ressapanda {}", version));
                    menu::MenuBar::new().ui(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("New").clicked() {
                                // Handle "New" action
                            }
                            if ui.button("Open").clicked() {
                                // Handle "Open" action
                            }
                            if ui.button("Save").clicked() {
                                // Handle "Save" action
                            }
                        });
                        ui.menu_button("Model", |ui| {
                            if ui.button("Move to the Left").clicked() {
                                let move_vec = VoxelSceneDirection::Left.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui.button("Move to the Right").clicked() {
                                let move_vec = VoxelSceneDirection::Right.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui.button("Move Back").clicked() {
                                let move_vec =
                                    VoxelSceneDirection::Backwards.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui.button("Move Forward").clicked() {
                                let move_vec =
                                    VoxelSceneDirection::Forwards.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui.button("Move Up").clicked() {
                                let move_vec =
                                    VoxelSceneDirection::UpLeftSteer.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui.button("Move Down").clicked() {
                                let move_vec =
                                    VoxelSceneDirection::DownRightSteer.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            ui.separator();
                            if ui.button("Rotate to the Left").clicked() {
                                let (axis, deg) =
                                    VoxelSceneDirection::Left.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui.button("Rotate to the Right").clicked() {
                                let (axis, deg) =
                                    VoxelSceneDirection::Right.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui.button("Rotate Back").clicked() {
                                let (axis, deg) =
                                    VoxelSceneDirection::Backwards.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui.button("Rotate Forward").clicked() {
                                let (axis, deg) =
                                    VoxelSceneDirection::Forwards.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui.button("Steer to the Left").clicked() {
                                let (axis, deg) = VoxelSceneDirection::UpLeftSteer
                                    .rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui.button("Steer to the Right").clicked() {
                                let (axis, deg) = VoxelSceneDirection::DownRightSteer
                                    .rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            ui.separator();
                            if ui.button("Reposition to Center").clicked() {
                                self.voxel_scene.reposition_to_calculated_center();
                            }
                        });
                        ui.menu_button("View", |ui| {
                            let target = Point3::new(0.0, 0.0, 0.0);
                            if ui.button("View along X+").clicked() {
                                let eye = CameraLookDirection::Xplus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui.button("View along Z+").clicked() {
                                let eye = CameraLookDirection::Zplus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui.button("View along Z-").clicked() {
                                let eye = CameraLookDirection::Zminus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui.button("View along X-").clicked() {
                                let eye = CameraLookDirection::Xminus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui.button("View straight from top").clicked() {
                                let eye = CameraLookDirection::Down.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                        });
                    });
                });
            });
        let spacing = self.config.height as f32 * 0.05;
        let mut top_offset = top_panel_width;
        let brush_window = egui::Window::new("Brush")
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, [0.0, top_offset])
            .collapsible(false)
            .auto_sized()
            .show(ui, |ui| {
                let swatch = egui::Button::new("")
                    .fill(self.ui_info.ui_brush_color)
                    .corner_radius(0)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE));
                let size = self.config.width as f32 * 0.05;
                if ui.add_sized([size, size], swatch).clicked() {
                    self.ui_info.show_color_picker = !self.ui_info.show_color_picker;
                }
            });
        if let Some(inner) = brush_window {
            top_offset = inner.response.rect.max.y + spacing;
        }
        let selection_mode_window = egui::Window::new("Selection Modes")
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, [0.0, top_offset])
            .collapsible(false)
            .auto_sized()
            .show(ui, |ui| {
                ["Single", "Area"]
                    .iter()
                    .for_each(|name| self.selection_mode_toggle_button(ui, *name));
            });
        if let Some(inner) = selection_mode_window {
            top_offset = inner.response.rect.max.y + spacing;
        }
        let tools_window = egui::Window::new("Tools")
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, [0.0, top_offset])
            .collapsible(false)
            .auto_sized()
            .show(ui, |ui| {
                ["Add", "Subs", "Del"]
                    .iter()
                    .for_each(|name| self.tool_toggle_button(ui, *name));
            });
        if self.ui_info.show_color_picker {
            egui::Window::new("Brush Color")
                .auto_sized()
                .collapsible(false)
                .open(&mut self.ui_info.show_color_picker)
                .show(ui, |ui| {
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = 300.0;
                        if egui::color_picker::color_picker_color32(
                            ui,
                            &mut self.ui_info.ui_brush_color,
                            egui::color_picker::Alpha::OnlyBlend,
                        ) {
                            self.ui_info.color_hex_input_string =
                                self.ui_info.ui_brush_color.to_hex();
                        }
                    });
                    ui.label("Color Hex:");
                    let hex_color_input =
                        ui.text_edit_singleline(&mut self.ui_info.color_hex_input_string);
                    if hex_color_input.changed() {
                        if let Some(color) =
                            Color32::from_hex(&self.ui_info.color_hex_input_string).ok()
                        {
                            self.ui_info.ui_brush_color = color
                        }
                    }
                });
            self.current_brush = Brush {
                color: VoxelColor::from_egui_color(self.ui_info.ui_brush_color),
            }
        }
    }
    fn tool_toggle_button(&mut self, ui: &mut egui::Ui, tool_name: &'static str) {
        let Some(texture) = self.ui_info.icon_loader.get_icon_texture(tool_name) else {
            return;
        };
        let sized = SizedTexture::new(texture.id(), [32.0, 32.0]);
        let img = Image::new(sized);
        let button = egui::Button::image(img);
        let response = ui.add(button);
        if response.clicked() {
            if let Some(tool) = tool_from_name(tool_name) {
                self.current_tool = tool;
            }
        }
        if self.current_tool.name() == tool_name {
            response.highlight();
        }
    }
    fn selection_mode_toggle_button(&mut self, ui: &mut egui::Ui, mode_name: &'static str) {
        let Some(texture) = self.ui_info.icon_loader.get_icon_texture(mode_name) else {
            return;
        };
        let sized = SizedTexture::new(texture.id(), [32.0, 32.0]);
        let img = Image::new(sized);
        let button = egui::Button::image(img);
        let response = ui.add(button);
        if response.clicked() {
            if let Some(mode) = select_mode_from_name(mode_name) {
                self.current_select_mode = mode;
            }
        }
        if self.current_select_mode.name() == mode_name {
            response.highlight();
        }
    }
}
