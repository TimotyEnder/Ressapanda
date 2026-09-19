use crate::{
    brushes::brush::Brush,
    camera::{Camera, CameraLookDirection, CameraUniform},
    camera_controller::CameraController,
    color::{
        ACTIVE_BG_STROKE, BLACK, OPEN_WEAK_BG_FILL, ORANGE, PANEL_FILL, SUB_PANEL_FILL, VoxelColor,
        WHITE,
    },
    cursor_loader::CursorLoader,
    depth_texture::DepthTexture,
    save::{load_from_file, save_to_file},
    select_mode::{
        select_mode::{SelectMode, select_mode_from_name, select_mode_tooltip_from_name},
        single_select_mode::SingleSelectMode,
    },
    tools::{
        add::Add,
        key_input_manager::KeyInputManager,
        tool::{Tool, tool_from_name, tool_tooltip_from_name},
    },
    ui_data::{FileAction, UIData},
    vertex::{CUBE_INDICES, CUBE_VERTICES, Vertex},
    voxel_instance::RawVoxelInstance,
    voxel_scene::{VoxelScene, VoxelSceneDirection},
};
use cgmath::Point3;
use egui::{Align2, Color32, FontId, Frame, Image, Panel, Rect, epaint, load::SizedTexture, menu};
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
    pub orienting_cross_camera: Camera,
    orienting_cross_camera_uniform: CameraUniform,
    orienting_cross_camera_buffer: wgpu::Buffer,
    orienting_cross_camera_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    pub voxel_scene: VoxelScene,
    pub orienting_cross_scene: VoxelScene,
    voxel_instance_buffer: wgpu::Buffer,
    orienting_cross_instance_buffer: wgpu::Buffer,
    depth_texture: DepthTexture,
    camera_controller: CameraController,
    key_input_manager: KeyInputManager,
    current_tool: Box<dyn Tool>,
    current_select_mode: Box<dyn SelectMode>,
    last_mouse_position_recorded: PhysicalPosition<f64>,
    next_frame_instance_count: usize,
    current_brush: Brush,
    cursor_loader: CursorLoader,
    egui_ctx: egui::Context,
    pub egui_winit_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    egui_paint_jobs: Vec<epaint::ClippedPrimitive>,
    egui_textures_delta: egui::TexturesDelta,
    ui_info: UIData,
}
impl State {
    pub fn get_window_name(&self) -> String {
        let saved = self.voxel_scene.get_saved();
        if let Some(ref path) = self.ui_info.current_save_path {
            if let Some(name) = path.file_name() {
                return format!(
                    "Ressapanda!:{}{}",
                    name.display(),
                    if saved { "" } else { "*" }
                );
            }
        }
        return format!("Ressapanda!{}", if saved { "" } else { "*" });
    }
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
        // Imagine reading tutorials well :< better late then never
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
            1000.0,
        );
        let orienting_cross_camera = Camera::new(
            (0.0, 5.0, 10.0).into(),
            (0.0, 0.0, 0.0).into(),
            cgmath::Vector3::unit_y(),
            config.width as f32 / config.height as f32,
            45.0,
            0.1,
            20.0,
        );
        let camera_controller = CameraController::new(0.01, &camera);
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let mut orienting_cross_camera_uniform = CameraUniform::new();
        orienting_cross_camera_uniform.update_view_proj(&orienting_cross_camera);
        let orienting_cross_camera_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[orienting_cross_camera_uniform]),
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
        let orienting_cross_camera_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: orienting_cross_camera_buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            });

        let mut voxel_scene = VoxelScene::new();
        let mut orienting_cross_scene = VoxelScene::orientating_cross_scene();
        let voxel_instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(voxel_scene.prepare_buffer_contents()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let orienting_cross_instance_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Orienting Cross Instance Buffer"),
                contents: bytemuck::cast_slice(orienting_cross_scene.prepare_buffer_contents()),
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
            &*window,
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

        visuals.selection.bg_fill = WHITE;
        visuals.selection.stroke.color = BLACK;
        visuals.widgets.inactive.bg_stroke.color = BLACK;
        visuals.widgets.hovered.bg_stroke.color = ORANGE;
        visuals.widgets.active.bg_stroke.color = ACTIVE_BG_STROKE;
        visuals.widgets.open.weak_bg_fill = OPEN_WEAK_BG_FILL;
        visuals.panel_fill = PANEL_FILL;
        visuals.window_fill = PANEL_FILL;
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
            orienting_cross_instance_buffer,
            voxel_scene,
            orienting_cross_scene,
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
            orienting_cross_camera_uniform,
            orienting_cross_camera,
            orienting_cross_camera_buffer,
            orienting_cross_camera_bind_group,
        })
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn handle_key(&mut self, _event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        self.key_input_manager.modifier_inputs(key, pressed);
        if self.key_input_manager.save_input(key, pressed) {
            self.conditional_save();
        }
        if self.key_input_manager.resize_input(key, pressed) {
            self.ui_info.voxel_grid_resize_popup = !self.ui_info.voxel_grid_resize_popup;
        }
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
        _event_loop: &ActiveEventLoop,
        button: MouseButton,
        pressed: bool,
    ) {
        self.camera_controller.handle_mouse_button(button, pressed);

        if button == MouseButton::Left && pressed {
            self.current_select_mode.mouse_down(
                self.last_mouse_position_recorded.x,
                self.last_mouse_position_recorded.y,
                self.key_input_manager.get_modifier_keys_status(),
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
                self.key_input_manager.get_modifier_keys_status(),
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
    pub fn handle_focus(&mut self) {
        self.cursor_loader.change_cursor(
            self.window.as_ref(),
            &self.current_select_mode,
            &self.current_tool,
        );
    }
    pub fn update(&mut self) {
        self.window.set_title(&self.get_window_name());
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_controller
            .update_orienting_cross_camera(&mut self.orienting_cross_camera);
        self.update_camera();
        self.update_temporary_voxel_generation_on_hover();
        self.update_voxel_buffers();
        self.ui_update();
        self.cursor_loader.change_cursor(
            self.window.as_ref(),
            &self.current_select_mode,
            &self.current_tool,
        );
    }
    fn ui_update(&mut self) {
        let raw_input = self.egui_winit_state.take_egui_input(&self.window);
        let egui_ctx = self.egui_ctx.clone();
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
            self.key_input_manager.get_modifier_keys_status(),
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
                // created with it here, but we'll just bail cuz fuck that
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
                label: Some(" Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: crate::color::srgb_to_linear(0.3094) as f64,
                            g: crate::color::srgb_to_linear(0.3094) as f64,
                            b: crate::color::srgb_to_linear(0.3094) as f64,
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
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(" Orienting Cross Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
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
            let size = 100.0;
            let margin = 10.0;
            let offset = 10.0;
            let vx = self.config.width as f32 - size - margin + offset;
            let vy = self.config.height as f32 - size - margin - offset;
            render_pass.set_viewport(vx, vy, size, size, 0.0, 1.0);
            render_pass.set_vertex_buffer(1, self.orienting_cross_instance_buffer.slice(..));
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.orienting_cross_camera_bind_group, &[]);
            render_pass.draw_indexed(
                0..CUBE_INDICES.len() as u32,
                0,
                0..self.orienting_cross_scene.get_voxel_instance_count() as u32,
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
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
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
        self.orienting_cross_camera_uniform
            .update_view_proj(&self.orienting_cross_camera);
        self.queue.write_buffer(
            &self.orienting_cross_camera_buffer,
            0,
            bytemuck::cast_slice(&[self.orienting_cross_camera_uniform]),
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
    fn conditional_save(&mut self) {
        if let Some(ref path) = self.ui_info.current_save_path {
            match save_to_file(&self.voxel_scene, &path) {
                Err(e) => log::error!("Save failed: {e}"),
                _ => {
                    self.voxel_scene.set_saved();
                }
            }
        } else {
            self.ui_info.file_dialog.set_user_data(FileAction::Save);
            self.ui_info.file_dialog.save_file();
        }
    }
    fn orientation_legend(&self, ui: &mut egui::Ui) {
        let mut job = egui::text::LayoutJob::default();
        let fmt = |color| egui::TextFormat {
            font_id: FontId::proportional(30.0),
            color,
            ..Default::default()
        };

        job.append("X", 0.0, fmt(egui::Color32::from_rgb(255, 0, 0)));
        job.append(" ", 0.0, fmt(WHITE));
        job.append("Y", 0.0, fmt(egui::Color32::from_rgb(0, 255, 0)));
        job.append(" ", 0.0, fmt(WHITE));
        job.append("Z", 0.0, fmt(egui::Color32::from_rgb(0, 0, 255)));

        egui::Window::new("Orientation Legend")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, [-15.0, 0.0])
            .frame(Frame::NONE)
            .show(ui, |ui| {
                ui.label(job);
            });
    }
    fn ui(&mut self, ui: &mut egui::Ui) {
        let version = env!("CARGO_PKG_VERSION");
        let top_panel_width = 23.0;
        Panel::top("options_panel")
            .exact_size(top_panel_width)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("Ressapanda {}", version));
                    menu::MenuBar::new().ui(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("New").clicked() {
                                self.voxel_scene = VoxelScene::new();
                                self.ui_info.current_save_path = None;
                            }
                            if ui.button("Open").clicked() {
                                self.ui_info.file_dialog.set_user_data(FileAction::Open);
                                self.ui_info.file_dialog.pick_file();
                            }
                            if ui.button("Save").clicked() {
                                self.conditional_save();
                            }
                            if ui.button("Save As").clicked() {
                                self.ui_info.file_dialog.set_user_data(FileAction::Save);
                                self.ui_info.file_dialog.save_file();
                            }
                        });
                        ui.menu_button("Model", |ui| {
                            if ui
                                .button("Move to the Left")
                                .on_hover_text("(Shortcut:Alt+A)")
                                .clicked()
                            {
                                let move_vec = VoxelSceneDirection::Left.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui
                                .button("Move to the Right")
                                .on_hover_text("(Shortcut:Alt+D)")
                                .clicked()
                            {
                                let move_vec = VoxelSceneDirection::Right.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui
                                .button("Move Back")
                                .on_hover_text("(Shortcut:Alt+S)")
                                .clicked()
                            {
                                let move_vec =
                                    VoxelSceneDirection::Backwards.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui
                                .button("Move Forward")
                                .on_hover_text("(Shortcut:Alt+W)")
                                .clicked()
                            {
                                let move_vec =
                                    VoxelSceneDirection::Forwards.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui
                                .button("Move Up")
                                .on_hover_text("(Shortcut:Alt+Q)")
                                .clicked()
                            {
                                let move_vec =
                                    VoxelSceneDirection::UpLeftSteer.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            if ui
                                .button("Move Down")
                                .on_hover_text("(Shortcut:Alt+E)")
                                .clicked()
                            {
                                let move_vec =
                                    VoxelSceneDirection::DownRightSteer.move_vector(&self.camera);
                                self.voxel_scene.move_by_vector(move_vec);
                            }
                            ui.separator();
                            if ui
                                .button("Rotate to the Left")
                                .on_hover_text("(Shortcut:Shift+A)")
                                .clicked()
                            {
                                let (axis, deg) =
                                    VoxelSceneDirection::Left.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui
                                .button("Rotate to the Right")
                                .on_hover_text("(Shortcut:Shift+D)")
                                .clicked()
                            {
                                let (axis, deg) =
                                    VoxelSceneDirection::Right.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui
                                .button("Rotate Back")
                                .on_hover_text("(Shortcut:Shift+S)")
                                .clicked()
                            {
                                let (axis, deg) =
                                    VoxelSceneDirection::Backwards.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui
                                .button("Rotate Forward")
                                .on_hover_text("(Shortcut:Shift+W)")
                                .clicked()
                            {
                                let (axis, deg) =
                                    VoxelSceneDirection::Forwards.rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui
                                .button("Steer to the Left")
                                .on_hover_text("(Shortcut:Shift+Q)")
                                .clicked()
                            {
                                let (axis, deg) = VoxelSceneDirection::UpLeftSteer
                                    .rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            if ui
                                .button("Steer to the Right")
                                .on_hover_text("(Shortcut:Shift+E)")
                                .clicked()
                            {
                                let (axis, deg) = VoxelSceneDirection::DownRightSteer
                                    .rotate_parameters(&self.camera);
                                self.voxel_scene.rotate_around_center(axis, deg);
                            }
                            ui.separator();
                            if ui
                                .button("Reposition to Center")
                                .on_hover_text("(Shortcut:Alt+C)")
                                .clicked()
                            {
                                self.voxel_scene.reposition_to_calculated_center();
                            }
                        });
                        ui.menu_button("View", |ui| {
                            let target = Point3::new(0.0, 0.0, 0.0);
                            if ui
                                .button("View along X+")
                                .on_hover_text("(Shortcut:1)")
                                .clicked()
                            {
                                let eye = CameraLookDirection::Xplus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui
                                .button("View along Z+")
                                .on_hover_text("(Shortcut:2)")
                                .clicked()
                            {
                                let eye = CameraLookDirection::Zplus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui
                                .button("View along Z-")
                                .on_hover_text("(Shortcut:3)")
                                .clicked()
                            {
                                let eye = CameraLookDirection::Zminus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui
                                .button("View along X-")
                                .on_hover_text("(Shortcut:4)")
                                .clicked()
                            {
                                let eye = CameraLookDirection::Xminus.eye_position();
                                self.camera_controller.set_preset_orbit(eye);
                                self.camera.set_position(target, target + eye);
                            }
                            if ui
                                .button("View straight from top")
                                .on_hover_text("(Shortcut:5)")
                                .clicked()
                            {
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
                let brush_color_button = egui::Button::new("")
                    .fill(self.ui_info.ui_brush_color)
                    .corner_radius(0)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE));
                let size = 50.0;
                if ui
                    .add_sized([size, size], brush_color_button)
                    .on_hover_text("Brush Color")
                    .clicked()
                {
                    self.ui_info.show_color_picker = !self.ui_info.show_color_picker;
                    self.ui_info.color_selected = true;
                }
            });

        let _voxel_set_window = egui::Window::new("Voxel Groups")
            .title_frame(self.styled_title_frame(ui))
            .default_width(280.0)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_TOP, [0.0, top_offset])
            .show(ui, |ui| {
                Panel::top("Voxel Group Controls").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let Some(button) =
                            self.voxel_group_control_button_with_name("Add_Voxel_Group")
                        else {
                            return;
                        };
                        if ui
                            .add(button)
                            .on_hover_text("Add another voxel group to the scene")
                            .clicked()
                        {
                            self.voxel_scene.reset_input_state();
                            self.voxel_scene.add_voxel_group();
                            self.ui_info.voxel_group_rename_request_focus_flag = true;
                        }
                        let Some(button) =
                            self.voxel_group_control_button_with_name("Delete_Voxel_Group")
                        else {
                            return;
                        };
                        if ui
                            .add(button)
                            .on_hover_text("Remove the selected voxel group")
                            .clicked()
                        {
                            self.ui_info.voxel_group_removal_popup = true;
                        }

                        let Some(button) =
                            self.voxel_group_control_button_with_name("Merge_Voxel_Group")
                        else {
                            return;
                        };
                        if ui
                            .add(button)
                            .on_hover_text("Merge the selected vertex group to the one below it")
                            .clicked()
                        {
                            self.voxel_scene.merge_voxel_group();
                        }
                        let Some(button) =
                            self.voxel_group_control_button_with_name("Duplicate_Voxel_Group")
                        else {
                            return;
                        };
                        if ui
                            .add(button)
                            .on_hover_text("Duplicate the selected voxel group.")
                            .clicked()
                        {
                            self.voxel_scene.reset_input_state();
                            self.voxel_scene.duplicate_voxel_group();
                        }

                        let Some(button) =
                            self.voxel_group_control_button_with_name("Move_Voxel_Group_Up")
                        else {
                            return;
                        };
                        if ui
                            .add(button)
                            .on_hover_text(
                                "Move the selected vertex group up one place in the hierarchy",
                            )
                            .clicked()
                        {
                            self.voxel_scene.shift_voxel_group_up();
                        }

                        let Some(button) =
                            self.voxel_group_control_button_with_name("Move_Voxel_Group_Down")
                        else {
                            return;
                        };
                        if ui
                            .add(button)
                            .on_hover_text(
                                "Move the selected vertex group down one place in the hierarchy",
                            )
                            .clicked()
                        {
                            self.voxel_scene.shift_voxel_group_down();
                        }
                    });
                });

                egui::ScrollArea::vertical()
                    .max_height(self.config.height as f32 * 0.3)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            for (index, editable) in
                                self.voxel_scene.get_voxel_group_edit_flags_and_indexes()
                            {
                                self.voxel_group_menu_element(ui, index, editable);
                                ui.add_space(10.0);
                            }
                        });
                    });
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
                ["Single", "Area", "Extended"]
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
        if let Some(inner) = tools_window {
            top_offset = inner.response.rect.max.y + spacing;
        }
        let _actions_window = egui::Window::new("Actions")
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, [0.0, top_offset])
            .collapsible(false)
            .auto_sized()
            .show(ui, |ui| {
                self.resize_voxel_grid_button(ui);
            });
        self.popups(ui);
        self.file_save_dialog(ui);
        self.orientation_legend(ui);
    }

    fn file_save_dialog(&mut self, ui: &mut egui::Ui) {
        self.ui_info.file_dialog.update(ui.ctx());
        if let Some(path) = self.ui_info.file_dialog.take_picked() {
            match self.ui_info.file_dialog.user_data() {
                Some(FileAction::Open) => match load_from_file(&path) {
                    Ok(scene_saved) => self.voxel_scene = scene_saved,
                    Err(e) => log::error!("Open failed: {e}"),
                },
                Some(FileAction::Save) => {
                    if let Err(e) = save_to_file(&self.voxel_scene, &path) {
                        log::error!("Save failed: {e}");
                    } else {
                        self.voxel_scene.set_saved();
                    }
                }
                _ => {}
            }
            self.ui_info.current_save_path = Some(path);
        }
    }
    fn popups(&mut self, ui: &mut egui::Ui) {
        if self.ui_info.voxel_group_removal_popup {
            egui::Window::new("Remove Voxel Group {}?")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui, |ui| {
                    ui.label(format!(
                        "Are you sure you want to remove the '{:?}' voxel group(s)?",
                        self.voxel_scene.get_current_voxel_group_names(),
                    ));
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.voxel_scene.remove_voxel_group();
                            self.ui_info.voxel_group_removal_popup = false;
                        }
                        if ui.button("No").clicked() {
                            self.ui_info.voxel_group_removal_popup = false;
                        }
                    });
                });
        }

        if self.ui_info.show_color_picker {
            egui::Window::new("Brush Color")
                .auto_sized()
                .title_frame(self.styled_title_frame(ui))
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
                            self.current_brush = Brush {
                                color: VoxelColor::from_egui_color(self.ui_info.ui_brush_color),
                            };
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
                    ui.label("Recent Colors:");
                    let colors: Vec<&Color32> = self
                        .ui_info
                        .last_used_colors
                        .get_colors_ref()
                        .iter()
                        .rev()
                        .collect();
                    ui.horizontal(|ui| {
                        for color in colors {
                            Self::last_used_color_button(
                                ui,
                                color,
                                &mut self.current_brush,
                                &mut self.ui_info.ui_brush_color,
                                &mut self.ui_info.last_color_added,
                            );
                        }
                    })
                });
        } else if self.ui_info.color_selected {
            self.ui_info.color_selected = false;
            if self.ui_info.last_color_added != self.ui_info.ui_brush_color {
                self.ui_info
                    .last_used_colors
                    .push(self.ui_info.ui_brush_color);
                self.ui_info.last_color_added = self.ui_info.ui_brush_color;
            }
        }
        if self.ui_info.voxel_grid_resize_popup {
            egui::Window::new("Resize Voxel Grid")
                .fixed_size([200.0, 50.0])
                .title_frame(self.styled_title_frame(ui))
                .collapsible(false)
                .open(&mut self.ui_info.voxel_grid_resize_popup)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let responce_width = ui.add_sized(
                            [80.0, 15.0],
                            egui::TextEdit::singleline(
                                &mut self.ui_info.voxel_grid_resize_width_string,
                            ),
                        );
                        ui.label("X");
                        let responce_height = ui.add_sized(
                            [80.0, 15.0],
                            egui::TextEdit::singleline(
                                &mut self.ui_info.voxel_grid_resize_length_string,
                            ),
                        );
                        if self.ui_info.voxel_grid_resize_focus_flag {
                            self.ui_info.voxel_grid_resize_focus_flag = false;
                            responce_width.request_focus();
                        }
                        if responce_height.union(responce_width).lost_focus() {
                            self.voxel_scene.resize_voxel_grid_dimensions(
                                &self.ui_info.voxel_grid_resize_width_string,
                                &self.ui_info.voxel_grid_resize_length_string,
                            );
                        }
                    })
                });
        } else {
            let (width, length) = self.voxel_scene.get_voxel_grid_dimensions();
            self.ui_info.voxel_grid_resize_width_string = width.to_string();
            self.ui_info.voxel_grid_resize_length_string = length.to_string();
        }
        if !self.ui_info.voxel_grid_resize_popup && !self.ui_info.voxel_grid_resize_focus_flag {
            self.ui_info.voxel_grid_resize_focus_flag = true;
        }
    }
    fn styled_title_frame(&self, ui: &mut egui::Ui) -> egui::Frame {
        let title_frame = egui::Frame::window(&ui.style())
            .fill(OPEN_WEAK_BG_FILL)
            .stroke(egui::Stroke::NONE);
        title_frame
    }
    fn tool_toggle_button(&mut self, ui: &mut egui::Ui, tool_name: &'static str) {
        let Some(texture) = self.ui_info.icon_loader.get_icon_texture(tool_name) else {
            return;
        };
        let sized = SizedTexture::new(texture.id(), [50.0, 50.0]);
        let img = Image::new(sized);
        let button = egui::Button::image(img);
        let mut response = ui.add(button);
        response = response.on_hover_text(tool_tooltip_from_name(tool_name).unwrap_or_default());
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
        let sized = SizedTexture::new(texture.id(), [50.0, 50.0]);
        let img = Image::new(sized);
        let button = egui::Button::image(img);
        let mut response = ui.add(button);
        response =
            response.on_hover_text(select_mode_tooltip_from_name(mode_name).unwrap_or_default());
        if response.clicked() {
            if let Some(mode) = select_mode_from_name(mode_name) {
                self.current_select_mode = mode;
            }
        }
        if self.current_select_mode.name() == mode_name {
            response.highlight();
        }
    }
    fn last_used_color_button(
        ui: &mut egui::Ui,
        color: &Color32,
        brush: &mut Brush,
        ui_brush_color: &mut Color32,
        last_color_added: &mut Color32,
    ) {
        let last_color_button = egui::Button::new("")
            .fill(*color)
            .corner_radius(0)
            .stroke(egui::Stroke::new(1.0, egui::Color32::WHITE));
        if ui.add_sized([25.0, 25.0], last_color_button).clicked() {
            *brush = Brush {
                color: VoxelColor::from_egui_color(*color),
            };
            *ui_brush_color = *color;
            *last_color_added = *color;
        }
    }
    fn voxel_group_control_button_with_name(
        &self,
        button_icon_name: &'static str,
    ) -> Option<egui::Button<'_>> {
        let Some(texture) = self.ui_info.icon_loader.get_icon_texture(button_icon_name) else {
            return None;
        };
        let sized = SizedTexture::new(texture.id(), [25.0, 25.0]);
        let img = Image::new(sized);
        Some(egui::Button::image(img))
    }
    fn voxel_group_menu_element(&mut self, ui: &mut egui::Ui, index: usize, editable: bool) {
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(260.0, 50.0), egui::Sense::click());
        ui.painter().rect_filled(rect, 0, SUB_PANEL_FILL);
        if !editable {
            if let Some(name) = self.voxel_scene.get_voxel_group_name_ref(index) {
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    name,
                    FontId::proportional(15.0),
                    WHITE,
                );
            }
        } else {
            if let Some(input) = self.voxel_scene.get_voxel_group_name_ref_mut(index) {
                let size = egui::vec2(150.0, 20.0);
                let offset = 20.0;
                let pos = egui::pos2(
                    rect.center().x + offset - (size.x / 2.0),
                    rect.center().y - (size.y / 2.0),
                );
                let response = ui.put(
                    Rect::from_min_size(pos, size),
                    egui::TextEdit::singleline(input),
                );
                if self.ui_info.voxel_group_rename_request_focus_flag {
                    response.request_focus();
                    self.ui_info.voxel_group_rename_request_focus_flag = false;
                }
                if response.lost_focus() {
                    self.voxel_scene.stop_voxel_group_edit(index);
                }
            }
        }
        if self.voxel_scene.get_current_voxel_groups().contains(&index) {
            ui.painter().rect_stroke(
                rect,
                0,
                egui::Stroke::new(3.0, ORANGE),
                egui::StrokeKind::Inside,
            );
        } else {
            ui.painter().rect_stroke(
                rect,
                0,
                egui::Stroke::new(2.0, BLACK),
                egui::StrokeKind::Inside,
            );
        }
        if response.hovered() {
            ui.painter().rect_stroke(
                rect,
                0,
                egui::Stroke::new(1.0, WHITE),
                egui::StrokeKind::Inside,
            );
        }
        if response.clicked() {
            let mod_stat = self.key_input_manager.get_modifier_keys_status();
            if mod_stat.control_modifier {
                self.voxel_scene
                    .toggle_presence_in_current_voxel_group_selection(index);
            } else if mod_stat.shift_modifier {
                self.voxel_scene.shift_select_toggle_presence(index);
            } else {
                self.voxel_scene.set_current_voxel_group(vec![index]);
            }
            self.voxel_scene.reset_input_state();
        }
        if response.double_clicked() {
            self.voxel_scene.make_voxel_group_name_editable(index);
            self.ui_info.voxel_group_rename_request_focus_flag = true;
        }
        let Some(button) = self.voxel_group_visibility_button(index).take() else {
            return;
        };
        let size = egui::vec2(30.0, 30.0);
        let gap = 8.0;
        let pos = egui::pos2(rect.left() + gap, rect.center().y - (size.y / 2.0));
        let response = ui.put(egui::Rect::from_min_size(pos, size), button);
        if response.clicked() {
            self.voxel_scene
                .set_voxel_group_visibility(index, !self.voxel_scene.is_voxel_group_visible(index));
        }
    }
    fn voxel_group_visibility_button(&self, index: usize) -> Option<egui::Button<'_>> {
        let Some(texture) = self.ui_info.icon_loader.get_icon_texture({
            if self.voxel_scene.is_voxel_group_visible(index) {
                "Voxel_Group_Visible"
            } else {
                "Voxel_Group_Invisible"
            }
        }) else {
            return None;
        };
        let sized = SizedTexture::new(texture.id(), [25.0, 25.0]);
        let img = Image::new(sized);
        Some(egui::Button::image(img).fill(Color32::from_white_alpha(0)))
    }
    fn resize_voxel_grid_button(&mut self, ui: &mut egui::Ui) {
        let Some(texture) = self
            .ui_info
            .icon_loader
            .get_icon_texture("Resize_Voxel_Grid")
        else {
            return;
        };
        let sized = SizedTexture::new(texture.id(), [50.0, 50.0]);
        let img = Image::new(sized);
        let button = egui::Button::image(img);
        let responce = ui.add(button);
        if responce
            .on_hover_text("Resize the voxel grid (Shortcut:Z)")
            .clicked()
        {
            self.ui_info.voxel_grid_resize_popup = !self.ui_info.voxel_grid_resize_popup;
        }
    }
}
