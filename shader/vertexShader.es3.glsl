#version 300 es

precision mediump float;

in vec2 a_position;
in float a_temperature;
uniform vec2 u_resolution;
uniform vec2 u_domain;
uniform vec2 u_point_size_range;
uniform float u_diameter;
out float v_color;

void main (void) {
  v_color = a_temperature;
  gl_PointSize = min(max(u_diameter / u_domain.y * u_resolution.y, u_point_size_range[0]), u_point_size_range[1]);
  vec2 clip_space = 2. * (a_position / u_domain) - 1.;
  gl_Position = vec4(clip_space, 0., 1.);
}
