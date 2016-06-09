
#[macro_use]
extern crate glium; 
extern crate image;
    
mod util;
mod cow_vertex;
mod cow_face;
mod cow_hide;
mod cow_hoofs;
mod cow_horns;
mod cow_tail;
mod cow_udder;

fn run() {

    use glium::{DisplayBuild, Surface};

    let mut t: f32 = 0.0;
    let mut x: f32;
    
    let display = glium::glutin::WindowBuilder::new()
                  .with_title(format!("Exuberant Cow!"))
                  .with_depth_buffer(24)
                  .build_glium()
                  .unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let face_vertices = glium::VertexBuffer::new(
        &display, &cow_face::COW_FACE_VERTICES).unwrap();
    let hide_vertices = glium::VertexBuffer::new(
        &display, &cow_hide::COW_HIDE_VERTICES).unwrap();
    let hoofs_vertices = glium::VertexBuffer::new(
        &display, &cow_hoofs::COW_HOOFS_VERTICES).unwrap();
    let horns_vertices = glium::VertexBuffer::new(
        &display, &cow_horns::COW_HORNS_VERTICES).unwrap();
    let tail_vertices = glium::VertexBuffer::new(
        &display, &cow_tail::COW_TAIL_VERTICES).unwrap();
    let udder_vertices = glium::VertexBuffer::new(
        &display, &cow_udder::COW_UDDER_VERTICES).unwrap();

    let vertex_shader_src = r#"
        #version 140 
        
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;
        
        in vec3 position; 
        in vec3 normal;
        
        out vec3 v_normal;
        out vec3 v_position;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#; 

    let fragment_shader_src = r#"
        #version 140

        uniform vec3 u_light;

        in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;

        const vec3 ambient_color = vec3(0.63, 0.43, 0.36);
        const vec3 diffuse_color = vec3(0.5, 0.5, 0.5);
        const vec3 specular_color = vec3(0.0, 0.0, 0.0);

        void main() {
            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.);
            color = vec4(ambient_color +
                         diffuse * diffuse_color +
                         specular * specular_color, 1.0);
        }
    "#;


    let program = glium::Program::from_source(
        &display,
        vertex_shader_src,
        fragment_shader_src,
        None).unwrap();

    loop {

        t += 0.02;
        x = 0.5 * t.sin();

        // Drawing Pipeline
        let mut target = display.draw();

        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        // Calculate Perspective Matrix
        let fov: f32 = 3.141592 / 3.0;  // 120 deg
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        let perspective = [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ];

        let light = [-1.0, 0.4, 0.9f32];

        let view = util::view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);
        let uniforms = uniform! {
            model: [
                [ 0.3, 0.0,  0.0,  0.0 ],
                [ 0.0, 0.3,  0.0,  0.0 ],
                [ 0.0, 0.0,  0.3,  0.0 ],
                [   x, 0.0,  2.0,  1.0 ]],
            u_light: light,
            perspective: perspective,
            view: view,
        };

        // Set black background
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            // NB: would enable if all models were "closed" ("sealed", no gaps)
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };

        for part_vertices in vec![&face_vertices,
                                  &hide_vertices,
                                  &hoofs_vertices,
                                  &horns_vertices,
                                  &tail_vertices,
                                  &udder_vertices] {

            target.draw(part_vertices,
                        &indices,
                        &program,
                        &uniforms,
                        &params).unwrap();
        }

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
        // XXX: sleep here for 10ms
    }

}

fn main() {
    run();
}
