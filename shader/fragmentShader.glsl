precision mediump float;

uniform vec2 u_resolution;
varying float v_color;

void main (void) {
  vec2 circle_coord = 2. * gl_PointCoord - 1.;
  float distance_from_center = dot(circle_coord, circle_coord);
  if (1. < distance_from_center) {
    // external
    discard;
  } else {
    // internal
    float r = v_color < 0.5 ? 1. : 2. - 2. * v_color;
    float g = v_color < 0.5 ? 2. * v_color : 1.;
    float b = 1.;
    gl_FragColor = vec4(r, g, b, 1.);
  }
}
