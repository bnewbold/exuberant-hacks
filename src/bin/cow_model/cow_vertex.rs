
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
}
implement_vertex!(Vertex, position, normal);
