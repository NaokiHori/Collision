precision mediump float;

attribute vec2 a_position;
attribute float a_temperature;
uniform vec2 u_resolution;
uniform float u_length;
uniform float u_diameter;
varying float v_color;

void main (void) {
  v_color = a_temperature;
  gl_PointSize = u_diameter / u_length * u_resolution.x;
  // convert pixel coordinates to clip space [-1 : +1]
  vec2 clip_space = 2. * (a_position / u_length) - 1.;
  gl_Position = vec4(clip_space, 0., 1.);
}
