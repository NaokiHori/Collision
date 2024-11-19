import wbgInit, {
  Collision,
  InitOutput,
  radius as getParticleRadius,
} from "../pkg";
import { WebGLObjects, checkWebGLAvailability } from "./webgl";
import { getNumber } from "./urlSearchParams";
import { getCanvasElement, syncCanvasSize } from "./dom";
import { Timer } from "./timer";

function canvas2dDrawer({
  wasm,
  canvas,
  length,
  nitems,
  rate,
}: {
  wasm: InitOutput;
  canvas: HTMLCanvasElement;
  length: number;
  nitems: number;
  rate: number;
}) {
  const collision = new Collision(length, nitems, rate, Math.random());
  const radius = getParticleRadius();
  const ctx: CanvasRenderingContext2D = (function () {
    const ctx: CanvasRenderingContext2D | null = canvas.getContext("2d");
    if (null === ctx) {
      throw new Error("failed to get context");
    }
    return ctx;
  })();
  const timer = new Timer(1000);
  function updateAndDraw() {
    collision.update();
    const positions = new Float32Array(
      wasm.memory.buffer,
      collision.positions(),
      nitems * "xy".length,
    );
    const ratio = canvas.width / length;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.imageSmoothingEnabled = false;
    ctx.fillStyle = "#ffffff";
    ctx.beginPath();
    for (let index = 0; index < nitems; index++) {
      const x: number = ratio * positions[2 * index + 0];
      const y: number = ratio * positions[2 * index + 1];
      const r: number = ratio * radius;
      ctx.moveTo(x, y);
      ctx.arc(x, y, r, 0, 2 * Math.PI);
    }
    ctx.fill();
    timer.update();
    requestAnimationFrame(updateAndDraw);
  }
  window.addEventListener("resize", () => {
    syncCanvasSize(canvas);
  });
  syncCanvasSize(canvas);
  timer.start();
  updateAndDraw();
}

function webGLDrawer({
  wasm,
  canvas,
  length,
  nitems,
  rate,
}: {
  wasm: InitOutput;
  canvas: HTMLCanvasElement;
  length: number;
  nitems: number;
  rate: number;
}) {
  const collision = new Collision(length, nitems, rate, Math.random());
  const radius = getParticleRadius();
  const webGLObjects = new WebGLObjects(canvas, length, nitems, radius);
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
    syncCanvasSize(canvas);
    webGLObjects.handleResizeEvent(canvas);
  });
  syncCanvasSize(canvas);
  webGLObjects.handleResizeEvent(canvas);
  timer.start();
  updateAndDraw();
}

async function main() {
  const wasm: InitOutput = await wbgInit();
  const length: number = getNumber("length", 256, 16, 1024);
  const nitems: number = getNumber("nitems", 8192, 2, 32768);
  const rate: number = getNumber("rate", 1e-1, 1e-4, 1e3);
  console.log(`length: ${length.toExponential()}`);
  console.log(`number of particles: ${nitems.toString()}`);
  console.log(`draw rate: ${rate.toString()}`);
  const canvas: HTMLCanvasElement = getCanvasElement();
  const isWebGLAvailable: boolean = checkWebGLAvailability();
  if (isWebGLAvailable) {
    console.log("Use WebGL Drawer");
    webGLDrawer({ wasm, canvas, length, nitems, rate });
  } else {
    console.log("Use Canvas2D Drawer");
    canvas2dDrawer({ wasm, canvas, length, nitems, rate });
  }
}

window.addEventListener("load", () => {
  main().catch((error: unknown) => {
    console.error(error);
  });
});
