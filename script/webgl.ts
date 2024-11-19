import { getContext, WebGLContext } from "./webgl/context";
import { initProgram } from "./webgl/program";
import { initVBO } from "./webgl/vbo";
import { initResizeEvent } from "./webgl/resizeEvent";
import vertexShaderSource from "../shader/vertexShader.glsl?raw";
import fragmentShaderSource from "../shader/fragmentShader.glsl?raw";

export class WebGLObjects {
  gl: WebGLContext;
  program: WebGLProgram;
  handleResizeEvent: (canvas: HTMLCanvasElement) => void;
  positionsVBO: WebGLBuffer;
  temperaturesVBO: WebGLBuffer;

  constructor(
    canvas: HTMLCanvasElement,
    length: number,
    nitems: number,
    radius: number,
  ) {
    const gl: WebGLContext = getContext(canvas);
    const program = initProgram(gl, vertexShaderSource, fragmentShaderSource);
    gl.uniform1f(gl.getUniformLocation(program, "u_length"), length);
    gl.uniform1f(gl.getUniformLocation(program, "u_diameter"), 2 * radius);
    const handleResizeEvent = initResizeEvent(gl, program);
    const positionsVBO = initVBO({
      nitems,
      gl,
      program,
      attributeName: "a_position",
      stride: "xy".length,
      usage: gl.DYNAMIC_DRAW,
    });
    const temperaturesVBO = initVBO({
      nitems,
      gl,
      program,
      attributeName: "a_temperature",
      stride: 1,
      usage: gl.DYNAMIC_DRAW,
    });
    handleResizeEvent(canvas);
    this.gl = gl;
    this.program = program;
    this.handleResizeEvent = handleResizeEvent;
    this.positionsVBO = positionsVBO;
    this.temperaturesVBO = temperaturesVBO;
  }

  public draw(
    nitems: number,
    positions: Float32Array,
    temperatures: Float32Array,
  ) {
    const gl: WebGLContext = this.gl;
    const positionsVBO: WebGLBuffer = this.positionsVBO;
    const temperaturesVBO: WebGLBuffer = this.temperaturesVBO;
    gl.bindBuffer(gl.ARRAY_BUFFER, positionsVBO);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, positions);
    gl.bindBuffer(gl.ARRAY_BUFFER, null);
    gl.bindBuffer(gl.ARRAY_BUFFER, temperaturesVBO);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, temperatures);
    gl.bindBuffer(gl.ARRAY_BUFFER, null);
    gl.drawArrays(gl.POINTS, 0, nitems);
  }
}

export function checkWebGLAvailability(): boolean {
  const canvas: HTMLCanvasElement = document.createElement("canvas");
  const gl: WebGLRenderingContext | null = canvas.getContext("webgl");
  canvas.remove();
  return null !== gl;
}
