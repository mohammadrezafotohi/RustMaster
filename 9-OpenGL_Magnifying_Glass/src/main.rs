use macroquad::prelude::*;

const MAGNIFIYING_GLASS_VERTEX_SHADER: &'static str = 
r#"
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 texCoords;

out vec2 center;
out vec2 uv;
out vec2 uv_screen;

uniform vec2 centerSC;
uniform mat4 modelMatrix;
uniform mat4 projectionMatrix;

void main() {

    // Calculations
    vec4 res = projectionMatrix * modelMatrix * vec4(aPos, 1);
    vec4 c = projectionMatrix * modelMatrix * vec4(centerSC, 0, 1);

    // out's
    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    center = c.xy / 2.0 + vec2(0.5, 0.5);
    uv = texCoords;

    gl_Position = res;
}
"#;

const MAGNIFIYING_GLASS_FRAGMENT_SHADER: &'static str =
r#"
#version 330 core

out vec4 FragColor;

in vec2 center;
in vec2 uv;
in vec2 uv_screen;

uniform sampler2D textureData;

void main() {
    float gradient = length(uv);
    vec2 uv_zoom = (uv_screen - center) * gradient + center;

    FragColor = texture(textureData, uv_zoom);
}
"#;

#[macroquad::main("9-OpenGL_Magnifying_Glass")]
async fn main() {
    let texture: Texture2D = load_texture(r"image.png").await.unwrap();

    let magnify_program = load_material(
        ShaderSource::Glsl {
            vertex: MAGNIFIYING_GLASS_VERTEX_SHADER,
            fragment: MAGNIFIYING_GLASS_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![UniformDesc::new("centerSC", UniformType::Float2)],
..Default::default()
        },).unwrap();

    loop {
        clear_background(WHITE);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        
        let lens_center = mouse_position();
        magnify_program.set_uniform("centerSC", lens_center);

        draw_circle(lens_center.0, lens_center.1, 250.0, RED);

        gl_use_material(&magnify_program);
        gl_use_default_material();

        next_frame().await
    }
}