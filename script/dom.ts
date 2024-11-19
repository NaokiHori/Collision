export function getCanvasElement(): HTMLCanvasElement {
  const id = "my-canvas";
  const canvas: HTMLElement | null = document.getElementById(id);
  if (null === canvas) {
    throw new Error(`failed to find a canvas element: ${id}`);
  }
  return canvas as HTMLCanvasElement;
}

export function syncCanvasSize(canvas: HTMLCanvasElement) {
  const rect: DOMRect = canvas.getBoundingClientRect();
  const width: number = rect.width;
  const height: number = rect.height;
  canvas.width = width;
  canvas.height = height;
}
