import { WebGLContext } from "./context";

function initShader(
  gl: WebGLContext,
  type: WebGLContext["FRAGMENT_SHADER"] | WebGLContext["VERTEX_SHADER"],
  source: string,
): WebGLShader {
  // creates a shader of the given type
  const shader: WebGLShader | null = gl.createShader(type);
  if (null === shader) {
    throw new Error("gl.createShader failed");
  }
  // compile source
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  // check if shader being successfully compiled
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    const info: string = gl.getShaderInfoLog(shader) ?? "unknown message";
    gl.deleteShader(shader);
    throw new Error(`gl.compileShader failed: ${info}`);
  }
  return shader;
}

export function initProgram(
  gl: WebGLContext,
  vertexShaderSource: string,
  fragmentShaderSource: string,
): WebGLProgram {
  const vertexShader: WebGLShader = initShader(
    gl,
    gl.VERTEX_SHADER,
    vertexShaderSource,
  );
  const fragmentShader: WebGLShader = initShader(
    gl,
    gl.FRAGMENT_SHADER,
    fragmentShaderSource,
  );
  const program: WebGLProgram | null = gl.createProgram();
  if (null === program) {
    throw new Error("gl.createProgram failed");
  }
  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    const info: string = gl.getProgramInfoLog(program) ?? "unknown message";
    gl.deleteProgram(program);
    throw new Error(`gl.attachShader / gl.linkProgram failed: ${info}`);
  }
  gl.useProgram(program);
  return program;
}
