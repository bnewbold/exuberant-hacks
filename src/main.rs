
#[macro_use]
extern crate glium; 

extern crate image;
extern crate getopts;

use std::env;
use std::u64;
use std::process::exit;
use getopts::Options;
use glium::glutin::os::unix::WindowBuilderExt;
    
mod util;
mod cow_vertex;
mod cow_face;
mod cow_hide;
mod cow_hoofs;
mod cow_horns;
mod cow_tail;
mod cow_udder;

fn run(window_id: Option<u64>) {

    use glium::{DisplayBuild, Surface};

    let mut t: f32 = 0.0;
    let mut z: f32;

    let win_builder: glium::glutin::WindowBuilder = match window_id {
        Some(id) =>
            glium::glutin::WindowBuilder::new()
                                         .from_existing_window(id),
        None => glium::glutin::WindowBuilder::new()
                                             .with_title(format!("Exuberant Cow!"))
                                             .with_depth_buffer(24),
    };
    let display = win_builder.build_glium().unwrap();

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

        t += 0.03;
        z = 0.5 * t.sin();

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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Turn, eg, "-root" into "--root"
    let args = util::convert_xscreensaver_args(args);

    let mut opts = Options::new();

    // Common Args (all screensavers)
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "window", "run in a window (IGNORED)");
    opts.optflag("", "root", "run in root window (IGNORED)");
    opts.optflag("", "fps", "show frames per second (IGNORED)");
    opts.optopt("", "window-id", "X window id number", "NUM");

    // Bovine-specific args
    opts.optflag("", "wire", "wireframe mode (IGNORED)");
    opts.optopt("c", "count", "how many cows? (1 to 9) (IGNORED)", "NUM");
    opts.optopt("", "delay", "inter-frame delay (0 to 100000) (IGNORED)", "NUM");
    opts.optopt("s", "speed", "how fast? ratio, with 1.0 as normal (IGNORED)", "NUM");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, opts);
            println!("");
            println!("{}", f.to_string());
            exit(-1);
        }
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        exit(0);
    }

    // if no "--window-id", try environment variable (arg has priority though)
    let window_id_string: Option<String> =
        matches.opt_str("window-id")
               .or(env::var("XSCREENSAVER_WINDOW").ok());

    let window_id = window_id_string.map(|id| match util::dechex2u64(&id) {
        Ok(y) => y,
        Err(e) => {
            println!("Couldn't parse numerical argument: {}", e);
            exit(-1); },
    });

    if window_id.is_some() {
        println!("Drawing on existing X window: 0x{:07X}", window_id.unwrap());
    }

    run(window_id);
}
