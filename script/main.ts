import wbgInit, { InitOutput } from "../pkg";
import { checkWebGLAvailability, webGLDrawer } from "./webgl";
import { canvas2dDrawer } from "./canvas2d";
import { getNumber } from "./urlSearchParams";
import { getCanvasElement, getDivElement } from "./dom";

function decideDomainLengths(container: HTMLDivElement): {
  domainWidth: number;
  domainHeight: number;
} {
  const rect: DOMRect = container.getBoundingClientRect();
  const canvasAspectRatio: number = rect.width / rect.height;
  const length: number = getNumber("length", 256, 16, 1024);
  if (canvasAspectRatio < 1) {
    const domainWidth: number = length * canvasAspectRatio;
    const domainHeight: number = length;
    return { domainWidth, domainHeight };
  } else {
    const domainWidth: number = length;
    const domainHeight: number = length / canvasAspectRatio;
    return { domainWidth, domainHeight };
  }
}

async function main() {
  const container: HTMLDivElement = getDivElement("canvas-container");
  const canvas: HTMLCanvasElement = getCanvasElement("my-canvas");
  const { domainWidth, domainHeight } = decideDomainLengths(container);
  const canvasAspectRatio: number = domainWidth / domainHeight;
  const wasm: InitOutput = await wbgInit();
  const nitems: number = getNumber(
    "nitems",
    Math.round((domainWidth * domainHeight) / 6),
    2,
    32768,
  );
  const rate: number = getNumber("rate", 5e-2, 1e-4, 1e3);
  console.log(`domain width: ${domainWidth.toString()}`);
  console.log(`domain height: ${domainHeight.toString()}`);
  console.log(`number of particles: ${nitems.toString()}`);
  console.log(`draw rate: ${rate.toString()}`);
  const isWebGLAvailable: boolean = checkWebGLAvailability();
  if (isWebGLAvailable) {
    console.log("Use WebGL Drawer");
    webGLDrawer({
      wasm,
      canvasAspectRatio,
      container,
      canvas,
      domainWidth,
      domainHeight,
      nitems,
      rate,
    });
  } else {
    console.log("Use Canvas2D Drawer");
    canvas2dDrawer({
      wasm,
      canvasAspectRatio,
      container,
      canvas,
      domainWidth,
      domainHeight,
      nitems,
      rate,
    });
  }
}

window.addEventListener("load", () => {
  main().catch((error: unknown) => {
    console.error(error);
  });
});
