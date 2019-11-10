use super::triangle::{get_two_triangles, Triangle, TriangleColor};
use glutin::{GlContext, GlWindow};
use nalgebra::Vector3 as v3;
extern crate rand;
use super::generate_trail;
use crate::lib::util::random::random_v3;
use gl::types::*;
use rand::{thread_rng as rng, Rng};
use std::{
    ffi::CString,
    mem,
    os::raw::c_void,
    ptr, str,
    time::{Duration, SystemTime},
};

pub fn gen_window() -> (GlWindow, glutin::EventsLoop) {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }
    (gl_window, events_loop)
}

pub fn render_loop(gl_window: GlWindow, mut events_loop: glutin::EventsLoop) {
    let num = 10;
    let vertex_buffer = create_vertex_attributes(
        num,
        vec![&create_position_attribute, &create_color_attribute],
    );
    // println!("vertex buffer is {:?}", vertex_buffer);
    // let vertex_buffer = get_two_triangles();

    let (vao, vbo) = bind_buffer(&vertex_buffer);
    // println!("hei");
    let (mut ebo_buffer, mut ebo): (Vec<GLuint>, GLuint) = (sequential_ebo_indices(num), 0);
    // println!("ebo buffer is {:?}", ebo_buffer);
    unsafe {
        gl::BindVertexArray(vao);
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (ebo_buffer.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            &ebo_buffer[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW,
        );
        // gl::BindVertexArray(0);
    }
    let shader_program: GLuint = get_shader_program();

    let mut now = SystemTime::now();
    let time_per_frame = Duration::from_secs_f32(30.0 / 60.0);

    let mut last_vertex_index = 0;

    let lebo = ebo_buffer.len();
    let mut leimo = lebo - 1; // last ebo index minus one
    let mut lei = 0; // last ebo index
    let mut leipo = 1; // last ebo index plus one

    let mut should_close = false;
    while !should_close {
        should_close = poll_events(&mut events_loop, &gl_window);
        now = on_next_update_do(now, time_per_frame, &mut || {
            last_vertex_index = last_vertex_index % num;
            lei = lei % lebo;
            leimo = leimo % lebo;
            leipo = leipo % lebo;
            // println!("indices are {}, {}", leimo, lei);
            let new = new_position();
            let c = new.normalize();
            let new = v3::new(
                -1.0 + new[0] * 2.0,
                -1.0 + new[1] * 2.0,
                -1.0 + new[2] * 2.0,
            );
            let new_data = vec![new[0], new[1], new[2] /* c[0], c[1], c[2] */];
            modify_buffer(3 * last_vertex_index, new_data, vbo);
            let new_ebo_indices: Vec<GLuint> = vec![ebo_buffer[leimo], ebo_buffer[lei]];
            // println!("ebo_buffer is {:?}", ebo_buffer);
            // let mut ebo_buffer_copy = ebo_buffer.clone();
            let temp = ebo_buffer[lei];
            ebo_buffer[leipo] = ebo_buffer[lei];
            ebo_buffer[lei] = ebo_buffer[leimo];
            println!("{:?} becomes {:?}", (lei, leipo), (ebo_buffer[lei], ebo_buffer[leipo]));
            // println!("new_ebo_indices is {:?}", new_ebo_indices);
            unsafe {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferSubData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (lei * mem::size_of::<GLuint>()) as GLsizeiptr,
                    (new_ebo_indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                    &new_ebo_indices[0] as *const u32 as *const c_void,
                );
                // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
            // println!("Hei");
            last_vertex_index += 1;
            lei += 2;
            leimo += 2;
            leipo += 2;
        });
        unsafe {
            gl::ClearColor(0.39, 0.58, 0.92, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);
            // gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::LINES,
                ebo_buffer.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(), // offset i think
            );
            // gl::DrawArrays(gl::TRIANGLES, 0, vertex_buffer.len() as i32);
        }
        gl_window.swap_buffers().unwrap();
    }
}

fn sequential_ebo_indices(num_vertices: usize) -> Vec<GLuint> {
    let num_line_segments = num_vertices - 1;
    let mut indices: Vec<GLuint> = vec![0; num_line_segments * 2];
    for i in 0..num_line_segments {
        indices[(2 * i) as usize] = i as GLuint;
        indices[(2 * i + 1) as usize] = (i + 1) as GLuint;
    }
    indices
}

fn create_vertex_attributes(num: usize, generators: Vec<&dyn Fn() -> Vec<f32>>) -> Vec<f32> {
    (0..num)
        .flat_map(|_| generators.iter().flat_map(|generator| generator()))
        .collect()
}

fn create_position_attribute() -> Vec<f32> {
    let v = random_v3();
    vec![v.x, v.y, v.z]
}

fn create_color_attribute() -> Vec<f32> {
    let v = random_v3().normalize();
    vec![v.x, v.y, v.z]
}

fn get_vertex_attributes_3_close_packed(num: usize) -> Vec<f32> {
    (0..num)
        .map(|_| new_position())
        .map(|v| v3::new(-1.0 + v[0] * 2.0, -1.0 + v[1] * 2.0, -1.0 + v[2] * 2.0))
        .flat_map(|v| vec![v[0], v[1], v[2]])
        .collect()
}

fn get_num_vertex_attributes(num: usize) -> Vec<f32> {
    (0..num)
        .map(|_| new_position())
        .map(|v| v3::new(-1.0 + v[0] * 2.0, -1.0 + v[1] * 2.0, -1.0 + v[2] * 2.0))
        .flat_map(|v| {
            let c = v.normalize();
            vec![v[0], v[1], v[2], c[0], c[1], c[2]]
        })
        .collect()
}

fn on_next_update_do(
    mut now: SystemTime,
    time_per_frame: Duration,
    func: &mut dyn FnMut() -> (),
) -> SystemTime {
    match now.elapsed() {
        Ok(time_passed) => {
            if time_passed > time_per_frame {
                func();
                now = SystemTime::now();
            }
        }
        Err(e) => eprintln!("Noooo"),
    }
    now
}

fn poll_events(events_loop: &mut glutin::EventsLoop, gl_window: &GlWindow) -> bool {
    let mut should_close = false;
    events_loop.poll_events(|event| {
        if let glutin::Event::WindowEvent { event, .. } = event {
            match event {
                glutin::WindowEvent::CloseRequested => should_close = true,
                glutin::WindowEvent::Resized(size) => {
                    gl_window.resize(glutin::dpi::PhysicalSize::new(size.width, size.height))
                }
                _ => (),
            }
        }
    });
    should_close
}

fn new_position() -> v3<f32> {
    random_v3().normalize()
}

fn modify_buffer(offset: usize, data: Vec<f32>, vbo: GLuint) {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            (offset * mem::size_of::<GLfloat>()) as GLsizeiptr,
            (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &data[0] as *const f32 as *const c_void,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}

fn bind_buffer(buffer: &Vec<f32>) -> (GLuint, GLuint) {
    unsafe {
        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // println!("vbo val is: {}\nvao val is: {}", vbo, vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &buffer[0] as *const f32 as *const c_void,
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        (vao, vbo)
    }
}

fn bind_buffers() -> GLuint {
    unsafe {
        // Set up vao and vbos
        /*         let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ]; */
        let vertices = get_vertices();

        let (mut vbo, mut vao) = (0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        /*
        with vbo bound to the gpu memory, we can fill the current buffer object with our vertex information by calling
        gl::BufferData. We're putting the entire thing into gpu memory. We have now BUFFERED our vertex data. I will
        start calling this our "buffered data".
        */
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        /*
        now, we have buffered our data, but we haven't told OpenGL yet how to relate this data to the input parameters
        of the vertex shader. Specifically, we want to tell OpenGl that each pair of 3 floats corresponds to the input
        parameter in the vertex shader declared as "layout (location = 0) in vec3 aPos". Notice the "location 0" part
        here, I'm going to refer to that in a moment.

        Now we have to specify the vertex attributes which our vertex shader takes in. Calling gl::VertexAttribPointer,
        we tell OpenGl that our data
        (arg1:) corresponds to the 0th vertex attribute,
        (arg2:) the vertex attribute has 3 elements,
        (arg3:) each element is of type float,
        (arg4:) we don't want our vertices to renormalized to the interval -1<x,y,z<1,
        (arg5:) this vertex attribute starts at position "null" in the buffered data
        */
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        /*
        We have declared a vertex attribute, but so what? The point is that we are trying to make this attribute available
        in our vertex shader (remember the location 0 part?). To make each pair of three floats available as a vec3 in our
        shader, we need to ask OpenGL for this. We use (arg1:) 0 to indicate that this vertex attribute pointer refers to
        whatever input value was specified as "location = 0" in our vertex shader.
        */
        gl::EnableVertexAttribArray(0);
        /*
        Now that we have bound every second pair of three vertices starting from the FIRST..
        [1.0, 1.0, 0.5, 0.3, 0.2, 0.5, 0.1, 0.5, 0.9, 0.1, 0.2, 0.4, 0.5, and so on] like this
         ----this------       nope     ----this------      nope      ---- and so on
        ..to the aPos vec3 variable in the vertex shader, we also
        similarly want to bind every second pair of three vertices starting from the SECOND..
        [1.0, 1.0, 0.5, 0.3, 0.2, 0.5, 0.1, 0.5, 0.9, 0.1, 0.2, 0.4, 0.5, and so on] like this
              nope      -----this----       nope      -----this-----      and so on
        ..to the aCol vec3 variable  in the vertex shader.
        */
        gl::VertexAttribPointer(
            1,         // the attribute is the one for (location = 1) in the vertex shader
            3,         // the attribute has length 3
            gl::FLOAT, // type is float
            gl::FALSE, // no normalization
            6 * mem::size_of::<GLfloat>() as GLsizei, // distance between the same attributes
            (3 * mem::size_of::<GLfloat>()) as *const c_void, // starts at index 3 in the buffered data
        );
        /*then enable it*/
        gl::EnableVertexAttribArray(1);

        /* unbind the buffer and unbind the vertex array*/
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Wireframe
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        vao
    }
}

fn get_shader_program() -> GLuint {
    unsafe {
        // Setup shader compilation checks
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // -1 to skip trialing null character

        // Vertex shader
        let vertex_shader: GLuint = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        // Check for shader compilation errors
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }

        // Fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        // Check for shader compilation errors
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }

        // Link Shaders
        let shader_program: GLuint = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                str::from_utf8(&info_log).unwrap()
            );
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        shader_program
    }
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
//layout (location = 1) in vec3 aColor; // Specify a vertex attribute for color
//out vec3 color;
void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    //color = aColor; // pass the color along to the fragment shader
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;
//in vec3 color;
void main()
{
   // Set the fragment color to the color passed from the vertex shader
   FragColor = vec4(0.0, 0.0, 0.0, 1.0f);
}
"#;

fn get_vertices() -> Vec<f32> {
    let mut verticesv3: Vec<v3<f32>> = vec![];
    let general_direction = v3::new(1.0, 0.0, 0.0);
    verticesv3.push(v3::new(-0.5, 0.0, 0.0));
    verticesv3.push(v3::new(-0.25, -0.25, 0.0));
    verticesv3.push(v3::new(-0.0, -0.30, 0.0));
    verticesv3.push(v3::new(0.25, -0.25, 0.0));
    verticesv3.push(v3::new(0.5, 0.0, 0.0));
    /*     for i in 1..5 {
        let distortion = random_v3().normalize();
        let difference = 0.2 * (general_direction + distortion).normalize();
        let next = verticesv3[i - 1] + difference;
        verticesv3.push(next);
    } */
    let points = vec![
        verticesv3[0],
        verticesv3[1],
        verticesv3[2],
        verticesv3[1],
        verticesv3[2],
        verticesv3[3],
        verticesv3[2],
        verticesv3[3],
        verticesv3[4],
    ];

    points
        .iter()
        .flat_map(|v| {
            let c = v.normalize();
            vec![v[0], v[1], v[2], c[0], c[1], c[2]].into_iter()
        })
        .collect()
}
