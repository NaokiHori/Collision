export type WebGLContext = WebGLRenderingContext | WebGL2RenderingContext;

export function getContext(canvas: HTMLCanvasElement): WebGLContext {
  const gl2: WebGL2RenderingContext | null = canvas.getContext("webgl2");
  if (null !== gl2) {
    console.log("use WebGL2RenderingContext");
    return gl2;
  }
  const gl: WebGLContext | null = canvas.getContext("webgl");
  if (null !== gl) {
    console.log("use WebGLRenderingContext");
    return gl;
  }
  throw new Error("failed to fetch WebGL context");
}
