mod lib;
use lib::*;

const DIMENSION: u32 = 1500;

fn main() {

    let vertex_buffer : &[Vertex] = &[];

    let index_buffer : &[u32] = &[0,1,2];

    const PROGRAM_VERT : &str = "
                #version 330

                uniform vec2 cent;
                uniform float radiusScale;

                out vec2 v_pos;
                out vec2 center;
                out float radius;

                void main()
                {
                    vec2 pts[3] = vec2[3](
                        vec2(0.2*radiusScale, -0.1*radiusScale),
                        vec2(-0.2*radiusScale, -0.1*radiusScale),
                        vec2(0.0*radiusScale, 0.168*radiusScale));

                    v_pos = pts[gl_VertexID] + cent;  

                    float a = distance(pts[1], pts[2]); 
                    float b = distance(pts[2], pts[0]); 
                    float c = distance(pts[0], pts[1]); 
                
                    center = (pts[0] * a + pts[1] * b + pts[2] * c) / (a+b+c) + cent;
                    float s = (a + b + c) / 2.0;
                    radius = sqrt((s - a) * (s - b) * (s - c) / s);
                
                    gl_Position = vec4(v_pos, 0.0, 1.0);
                 
                }
            ";

    const PROGRAM_FRAGMENT: &str = "    
                #version 330

                out vec4 FragColor;
                in vec2 v_pos;
                in vec2 center;
                in float radius;
                
                void main()
                {
                    if (distance(v_pos, center) > radius)
                    {
                        discard;

                        // debug
                        //out_color = vec4(0.5, 0.5, 0.5, 1.0);
                        //return;
                    }
                    FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }";
    
    
    
    
    
    
    let b = DrawBuffer {
        vertex_buffer: vertex_buffer, 
        index_buffer: index_buffer,
        program_vert: PROGRAM_VERT,
        program_frag: PROGRAM_FRAGMENT,
    };

    let c = ShapeBuffer {
        shapes: Box::new(vec![
            Shape::Circ(Circle {
                center: [0.5 as f32, 0.5 as f32],
                radius: 1.0 as f32,
            }), 
            Shape::Circ(Circle {
                center: [0.0 as f32, 0.0 as f32],
                radius: 1.0 as f32,
            }), 
            Shape::Circ(Circle {
                center: [-0.5 as f32, -0.5 as f32],
                radius: 1.0 as f32,
            })]),
    };



    Renderer::render(&b, c, DIMENSION, true);
}