import { Collision, InitOutput, radius as getParticleRadius } from "../pkg";
import { Timer } from "./timer";
import { syncCanvasSize } from "./dom";

export function canvas2dDrawer({
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
    const amplificationFactor = canvas.width / domainWidth;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.imageSmoothingEnabled = false;
    ctx.fillStyle = "#ffffff";
    ctx.beginPath();
    for (let index = 0; index < nitems; index++) {
      const x: number = amplificationFactor * positions[2 * index + 0];
      const y: number = amplificationFactor * positions[2 * index + 1];
      const r: number = amplificationFactor * radius;
      ctx.moveTo(x, y);
      ctx.arc(x, y, r, 0, 2 * Math.PI);
    }
    ctx.fill();
    timer.update();
    requestAnimationFrame(updateAndDraw);
  }
  window.addEventListener("resize", () => {
    syncCanvasSize(canvasAspectRatio, container, canvas);
  });
  syncCanvasSize(canvasAspectRatio, container, canvas);
  timer.start();
  updateAndDraw();
}
