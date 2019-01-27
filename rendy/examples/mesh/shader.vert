#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 pos;
layout(location = 1) in vec4 color;
layout(location = 2) in vec3 norm;
layout(location = 3) in mat4 model; // per-instance.

layout(set = 0, binding = 0) uniform Args {
    mat4 proj;
    mat4 view;
};

layout(location = 0) out vec4 frag_pos;
layout(location = 1) out vec3 frag_norm;
layout(location = 2) out vec4 frag_color;

void main() {
    frag_color = color;
    frag_norm = normalize((vec4(norm, 1.0) * model).xyz);
    frag_pos = model * vec4(pos, 1.0);
    gl_Position = proj * view * frag_pos;
}
