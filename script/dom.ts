export function getCanvasElement(id: string): HTMLCanvasElement {
  const canvas: HTMLElement | null = document.getElementById(id);
  if (null === canvas) {
    throw new Error(`failed to find a canvas element: ${id}`);
  }
  return canvas as HTMLCanvasElement;
}

export function getDivElement(id: string): HTMLDivElement {
  const element: HTMLElement | null = document.getElementById(id);
  if (null === element) {
    throw new Error(`failed to find a div element: ${id}`);
  }
  return element as HTMLDivElement;
}

export function syncCanvasSize(
  canvasAspectRatio: number,
  container: HTMLDivElement,
  canvas: HTMLCanvasElement,
) {
  const rect: DOMRect = container.getBoundingClientRect();
  const containerWidth: number = rect.width;
  const containerHeight: number = rect.height;
  const containerAspect: number = containerWidth / containerHeight;
  const { canvasWidth, canvasHeight } = (function () {
    if (containerAspect > canvasAspectRatio) {
      // container is wider than canvas aspect ratio
      // fit canvas height to container
      return {
        canvasWidth: containerHeight * canvasAspectRatio,
        canvasHeight: containerHeight,
      };
    } else {
      // container is taller than canvas aspect ratio
      // fit canvas width to container
      return {
        canvasWidth: containerWidth,
        canvasHeight: containerWidth / canvasAspectRatio,
      };
    }
  })();
  // Update canvas size
  canvas.style.width = `${canvasWidth.toString()}px`;
  canvas.style.height = `${canvasHeight.toString()}px`;
  canvas.width = canvasWidth;
  canvas.height = canvasHeight;
}
