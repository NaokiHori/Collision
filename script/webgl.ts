import {
  InitOutput,
  Collision,
  radius as getParticleRadius,
} from "../pkg/collision";
import { Timer } from "./timer";
import { syncCanvasSize } from "./dom";
import { getContext, WebGLContext } from "./webgl/context";
import { initProgram } from "./webgl/program";
import { initVBO } from "./webgl/vbo";
import { initResizeEvent } from "./webgl/resizeEvent";
import vertexShaderSourceES2 from "../shader/vertexShader.es2.glsl?raw";
import fragmentShaderSourceES2 from "../shader/fragmentShader.es2.glsl?raw";
import vertexShaderSourceES3 from "../shader/vertexShader.es3.glsl?raw";
import fragmentShaderSourceES3 from "../shader/fragmentShader.es3.glsl?raw";

export class WebGLObjects {
  gl: WebGLContext;
  program: WebGLProgram;
  handleResizeEvent: (canvas: HTMLCanvasElement) => void;
  positionsVBO: WebGLBuffer;
  temperaturesVBO: WebGLBuffer;

  constructor(
    canvas: HTMLCanvasElement,
    domainWidth: number,
    domainHeight: number,
    nitems: number,
    radius: number,
  ) {
    const gl: WebGLContext = getContext(canvas);
    const isGL2: boolean = gl instanceof WebGL2RenderingContext;
    const program = initProgram(
      gl,
      isGL2 ? vertexShaderSourceES3 : vertexShaderSourceES2,
      isGL2 ? fragmentShaderSourceES3 : fragmentShaderSourceES2,
    );
    gl.uniform2f(
      gl.getUniformLocation(program, "u_domain"),
      domainWidth,
      domainHeight,
    );
    gl.uniform1f(gl.getUniformLocation(program, "u_diameter"), 2 * radius);
    (function setPointSizeRange() {
      const pointSizeRange: Float32Array = gl.getParameter(
        gl.ALIASED_POINT_SIZE_RANGE,
      ) as Float32Array;
      gl.uniform2f(
        gl.getUniformLocation(program, "u_point_size_range"),
        pointSizeRange[0],
        pointSizeRange[1],
      );
    })();
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

export function webGLDrawer({
  wasm,
  canvasAspectRatio,
  container,
  canvas,
  domainWidth,
  domainHeight,
  nitems,
  rate,
}: {
  wasm: InitOutput;
  canvasAspectRatio: number;
  container: HTMLDivElement;
  canvas: HTMLCanvasElement;
  domainWidth: number;
  domainHeight: number;
  nitems: number;
  rate: number;
}) {
  const collision = new Collision(
    domainWidth,
    domainHeight,
    nitems,
    rate,
    Math.random(),
  );
  const radius = getParticleRadius();
  const webGLObjects = new WebGLObjects(
    canvas,
    domainWidth,
    domainHeight,
    nitems,
    radius,
  );
  const timer = new Timer(1000);
  function updateAndDraw() {
    collision.update();
    const positions = new Float32Array(
      wasm.memory.buffer,
      collision.positions(),
      nitems * "xy".length,
    );
    const temperatures = new Float32Array(
      wasm.memory.buffer,
      collision.temperatures(),
      nitems,
    );
    webGLObjects.draw(nitems, positions, temperatures);
    timer.update();
    requestAnimationFrame(updateAndDraw);
  }
  window.addEventListener("resize", () => {
    syncCanvasSize(canvasAspectRatio, container, canvas);
    webGLObjects.handleResizeEvent(canvas);
  });
  syncCanvasSize(canvasAspectRatio, container, canvas);
  webGLObjects.handleResizeEvent(canvas);
  timer.start();
  updateAndDraw();
}
