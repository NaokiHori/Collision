import { WebGLContext } from "./context";

export function initResizeEvent(
  gl: WebGLContext,
  program: WebGLProgram,
): (canvas: HTMLCanvasElement) => void {
  // tasks to be done on window resize
  return (canvas: HTMLCanvasElement): void => {
    const w: number = canvas.width;
    const h: number = canvas.height;
    gl.viewport(0, 0, w, h);
    gl.uniform2f(gl.getUniformLocation(program, "u_resolution"), w, h);
  };
}
