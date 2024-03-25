use iced::mouse;
use iced::time::Instant;
use iced::widget::shader;
use iced::widget::shader::wgpu;
use iced::window;
use iced::{Element, Length, Rectangle, Size, Subscription};

const WORKGROUP_X: u32 = 8;
const WORKGROUP_Y: u32 = 8;

fn main() -> iced::Result {
    iced::program(
        "Custom Compute Shader - Iced",
        ComputeShader::update,
        ComputeShader::view,
    )
    .subscription(ComputeShader::subscription)
    .run()
}

struct ComputeShaderPipeline {
    pipeline: wgpu::ComputePipeline,
    bind_group: Option<wgpu::BindGroup>,
}

impl ComputeShaderPipeline {
    fn new(device: &wgpu::Device) -> Self {
        let shader =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("ComputeShaderPipeline shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    include_str!("compute.wgsl"),
                )),
                /* alternative: GLSL shader
                source: wgpu::ShaderSource::Glsl{
                    shader: std::borrow::Cow::Borrowed(include_str!("compute.glsl")),
                    stage: wgpu::naga::ShaderStage::Compute,
                    defines: wgpu::naga::FastHashMap::default(),
                },
                */
            });
        let pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("ComputeShaderPipeline"),
                layout: None,
                module: &shader,
                entry_point: "main",
            });

        Self {
            pipeline,
            bind_group: None,
        }
    }

    fn update(&mut self, device: &wgpu::Device, target: &wgpu::TextureView) {
        let bind_group_layout = self.pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ComputeShaderPipeline bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(target),
            }],
        });

        self.bind_group = Some(bind_group);
    }

    fn render(
        &self,
        _target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        if let Some(ref bind_group) = self.bind_group {
            let mut pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("compute pass"),
                    timestamp_writes: None,
                });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, bind_group, &[]);

            let x = (viewport.width + WORKGROUP_X - 1) / WORKGROUP_X;
            let y = (viewport.height + WORKGROUP_Y - 1) / WORKGROUP_Y;
            let z = 1;

            pass.dispatch_workgroups(x, y, z);
        }
    }
}

#[derive(Debug)]
struct ComputeShaderPrimitive {}

impl ComputeShaderPrimitive {
    fn new() -> Self {
        Self {}
    }
}

impl shader::Primitive for ComputeShaderPrimitive {
    fn prepare(
        &self,
        _format: wgpu::TextureFormat,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _bounds: Rectangle,
        target: &wgpu::TextureView,
        _target_size: Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        if !storage.has::<ComputeShaderPipeline>() {
            storage.store(ComputeShaderPipeline::new(device));
        }

        let pipeline = storage.get_mut::<ComputeShaderPipeline>().unwrap();

        //upload data to GPU
        pipeline.update(device, target);
    }

    fn render(
        &self,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        _target_size: Size<u32>,
        viewport: Rectangle<u32>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        //at this point our pipeline should always be initialized
        let pipeline = storage.get::<ComputeShaderPipeline>().unwrap();

        //render primitive
        pipeline.render(target, encoder, viewport);
    }
}

struct ComputeShaderProgram {}

#[derive(Debug, Clone)]
enum Message {
    Tick(Instant),
}

impl<Message> shader::Program<Message> for ComputeShaderProgram {
    type State = ();
    type Primitive = ComputeShaderPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        ComputeShaderPrimitive::new()
    }
}

struct ComputeShader {}

impl Default for ComputeShader {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputeShader {
    fn new() -> Self {
        Self {}
    }

    fn view(&self) -> Element<'_, Message> {
        let shader = shader(ComputeShaderProgram {})
            .width(Length::Fill)
            .height(Length::Fill);

        shader.into()
    }

    fn update(&mut self, _message: Message) {}

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}
