in vec2 pos;
in vec2 tex;

out vec2 v_tex;

void main()
{
    v_tex = tex;
    gl_Position = vec4(pos, 0.0, 1.0);
}
