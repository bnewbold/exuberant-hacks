
extern crate exuberant_hacks;
extern crate getopts;

#[macro_use]
extern crate glium; 

use exuberant_hacks::{ExuberantHack, run_hack, main_helper, make_display};
use getopts::Options;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

struct ExuberantPlasma {
    display: glium::Display,
    program: glium::Program,
}
 
impl ExuberantPlasma {

    pub fn new(display: glium::Display) -> ExuberantPlasma {

        let vertex_shader_src = r#" #version 140 
            
            in vec2 position;
            out vec2 v_coords;

            void main() {
                v_coords = position;
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#; 

        // This fragment shader verbatim from:
        // http://www.bidouille.org/prog/plasma
        let fragment_shader_src = r#" #version 140

            precision mediump float;
            #define PI 3.1415926535897932384626433832795
            
            uniform float u_time;
            uniform vec2 u_k;
            varying vec2 v_coords;
            
            void main() {
                float v = 0.0;
                vec2 c = v_coords * u_k - u_k/2.0;
                v += sin((c.x+u_time));
                v += sin((c.y+u_time)/2.0);
                v += sin((c.x+c.y+u_time)/2.0);
                c += u_k/2.0 * vec2(sin(u_time/3.0), cos(u_time/2.0));
                v += sin(sqrt(c.x*c.x+c.y*c.y+1.0)+u_time);
                v = v/2.0;
                vec3 col = vec3(1, sin(PI*v), cos(PI*v));
                gl_FragColor = vec4(col*.5 + .5, 1);
            }
        "#;


        let program = glium::Program::from_source(
            &display,
            vertex_shader_src,
            fragment_shader_src,
            None).unwrap();

        ExuberantPlasma {
            display: display,
            program: program,
        }
    }
}

impl ExuberantHack for ExuberantPlasma {

    fn draw_frame(&mut self, t: f64) -> Result<(), String> {

        let vertex1 = Vertex { position: [-1.0, -1.0] };
        let vertex2 = Vertex { position: [ 3.0, -1.0] };
        let vertex3 = Vertex { position: [-1.0,  3.0] };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Drawing Pipeline
        let mut target = self.display.draw();

        let uniforms = uniform! {
            u_time: (t % (12.0 * 3.141592)) as f32,
            u_k: [10.0 as f32, 10.0 as f32],
        };

        // Set black background
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let params = glium::DrawParameters {
            .. Default::default()
        };

        target.draw(&vertex_buffer,
                    &indices,
                    &self.program,
                    &uniforms,
                    &params).unwrap();

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
    let mut hack = ExuberantPlasma::new(dislpay);

    // Here is where you would configure the hack based on command line options

    // Ok, actually run it (loops forever)
    run_hack(&mut hack, &conf);
}
