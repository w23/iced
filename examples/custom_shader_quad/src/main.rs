use iced::mouse;
use iced::time::Instant;
use iced::widget::shader;
use iced::widget::shader::wgpu;
use iced::window;
use iced::{Element, Length, Rectangle, Size, Subscription};

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    aspect: f32,
}

struct CustomShaderPipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl CustomShaderPipeline {
    fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("CustomShaderPipeline shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    include_str!("shader.wgsl"),
                )),
            });

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("CustomShaderPipeline"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shader_quad uniform buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = pipeline.get_bind_group_layout(0);
        let uniform_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("shader_quad uniform bind group"),
                layout: &uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

        Self {
            pipeline,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    fn update(&mut self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(uniforms),
        );
    }

    fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fill color test"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_viewport(
            viewport.x as f32,
            viewport.y as f32,
            viewport.width as f32,
            viewport.height as f32,
            0.0,
            1.0,
        );
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);

        pass.draw(0..3, 0..1);
    }
}

#[derive(Debug)]
struct CustomShaderPrimitive {}

impl CustomShaderPrimitive {
    fn new() -> Self {
        Self {}
    }
}

impl shader::Primitive for CustomShaderPrimitive {
    fn prepare(
        &self,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: Rectangle,
        target_size: Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        if !storage.has::<CustomShaderPipeline>() {
            storage.store(CustomShaderPipeline::new(device, format));
        }

        let pipeline = storage.get_mut::<CustomShaderPipeline>().unwrap();

        pipeline.update(
            queue,
            &Uniforms {
                aspect: target_size.width as f32 / target_size.height as f32,
            },
        );
    }

    fn render(
        &self,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        _target_size: Size<u32>,
        viewport: Rectangle<u32>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let pipeline = storage.get::<CustomShaderPipeline>().unwrap();
        pipeline.render(target, encoder, viewport);
    }
}

struct CustomShaderProgram {}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
}

impl<Message> shader::Program<Message> for CustomShaderProgram {
    type State = ();
    type Primitive = CustomShaderPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        CustomShaderPrimitive::new()
    }
}

struct BasicShader {}

impl Default for BasicShader {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicShader {
    fn new() -> Self {
        Self {}
    }

    fn view(&self) -> Element<'_, Message> {
        let shader = shader(CustomShaderProgram {})
            .width(Length::Fill)
            .height(Length::Fill);

        shader.into()
    }

    fn update(&mut self, _message: Message) {}

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}

fn main() -> iced::Result {
    iced::program(
        "Custom Shader Quad - Iced",
        BasicShader::update,
        BasicShader::view,
    )
    .subscription(BasicShader::subscription)
    .run()
}
