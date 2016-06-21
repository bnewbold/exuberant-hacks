
extern crate exuberant_hacks;
extern crate getopts;

#[macro_use]
extern crate glium; 

use exuberant_hacks::{ExuberantHack, run_hack, main_helper, make_display};
use exuberant_hacks::util;
use getopts::Options;
use glium::Surface;

mod cow_model;

struct ExuberantBovines {
    display: glium::Display,
    program: glium::Program,
    model_vertices: Vec<glium::VertexBuffer<cow_model::Vertex>>,
}
 
impl ExuberantBovines {

    pub fn new(display: glium::Display) -> ExuberantBovines {

        let face_vertices = glium::VertexBuffer::new(
            &display, &cow_model::COW_FACE_VERTICES).unwrap();
        let hide_vertices = glium::VertexBuffer::new(
            &display, &cow_model::COW_HIDE_VERTICES).unwrap();
        let hoofs_vertices = glium::VertexBuffer::new(
            &display, &cow_model::COW_HOOFS_VERTICES).unwrap();
        let horns_vertices = glium::VertexBuffer::new(
            &display, &cow_model::COW_HORNS_VERTICES).unwrap();
        let tail_vertices = glium::VertexBuffer::new(
            &display, &cow_model::COW_TAIL_VERTICES).unwrap();
        let udder_vertices = glium::VertexBuffer::new(
            &display, &cow_model::COW_UDDER_VERTICES).unwrap();

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

        return ExuberantBovines {
            display: display,
            program: program,
            model_vertices: vec![face_vertices,
                                 hide_vertices,
                                 hoofs_vertices,
                                 horns_vertices,
                                 tail_vertices,
                                 udder_vertices],
        };
    }
}

impl ExuberantHack for ExuberantBovines {

    fn draw_frame(&mut self, t: f64) -> Result<(), String> {

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let z: f32 = 0.5 * t.sin() as f32;

        // Drawing Pipeline
        let mut target = self.display.draw();

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
                [-0.5,   z,  2.0,  1.0 ]],
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

        for part_vertices in &self.model_vertices {

            target.draw(part_vertices,
                        &indices,
                        &self.program,
                        &uniforms,
                        &params).unwrap();
        }

        target.finish().or(Err("Failure rendering".to_string()))
    }

    fn get_display(&self) -> &glium::Display {
        &self.display
    }
}

fn main() {

    let mut opts = Options::new();
    opts.optopt("c", "count", "how many cows? (1 to 9) (IGNORED)", "NUM");
    opts.optopt("s", "speed", "how fast? ratio, with 1.0 as normal (IGNORED)", "NUM");
    opts.optflag("", "wireframe", "wireframe mode (IGNORED)");

    let conf = main_helper(opts);
    let dislpay = make_display(&conf);
    let mut hack = ExuberantBovines::new(dislpay);

    // Here is where you would configure the hack based on command line options

    // Ok, actually run it (loops forever)
    run_hack(&mut hack, &conf);
}
