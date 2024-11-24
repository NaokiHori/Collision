export type WebGLContext = WebGLRenderingContext | WebGL2RenderingContext;

export function getContext(canvas: HTMLCanvasElement): WebGLContext {
  const gl2: WebGL2RenderingContext | null = canvas.getContext("webgl2");
  if (null !== gl2) {
    console.log("Use WebGL2RenderingContext");
    return gl2;
  }
  const gl: WebGLContext | null = canvas.getContext("webgl");
  if (null !== gl) {
    console.log("Use WebGLRenderingContext");
    return gl;
  }
  throw new Error("failed to fetch WebGL context");
}
