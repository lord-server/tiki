pub struct Mesh {
    data: Vec<f32>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        v.write_data(&mut self.data);
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }
}

pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub texcoord: glam::Vec2,
}

impl Vertex {
    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: 8 * 4,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 3 * 4,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 6 * 4,
                    shader_location: 2,
                },
            ],
        }
    }

    pub fn write_data(&self, data: &mut Vec<f32>) {
        data.extend_from_slice(&self.position.to_array());
        data.extend_from_slice(&self.normal.to_array());
        data.extend_from_slice(&self.texcoord.to_array());
    }
}
