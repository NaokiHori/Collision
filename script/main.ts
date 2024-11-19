import wbgInit, { Collision } from "../pkg";

function getAndSet(keyword: string, defval: number, minval: number, maxval: number): number {
  // check if URL param is given
  const urlParams = new URLSearchParams(window.location.search);
  // if not given, use default value
  let val: number = defval;
  if (urlParams.has(keyword)) {
    // if given, use after sanitised
    let tmp: number | null = Number(urlParams.get(keyword));
    if (tmp) {
      tmp = tmp < minval ? minval : tmp;
      tmp = maxval < tmp ? maxval : tmp;
      val = tmp;
    }
  }
  return val;
}

window.addEventListener("load", () => {
  wbgInit().then(() => {
    // all things which should be done before iterating
    const length: number = getAndSet("length", 192., 8., 1024.);
    const nitems: number = getAndSet("nitems", 8192, 2, 32768);
    const rate: number = getAndSet("rate", 1e-1, 1e-4, 1e3);
    console.log(`length: ${length.toExponential()}`);
    console.log(`number of particles: ${nitems}`);
    console.log(`draw rate: ${rate}`);
    // initialise simulator and drawer
    const collision = Collision.new(length, nitems, rate, Math.random());
    // initialise window / canvas size
    collision.update_canvas_size();
    // register it to an event handler
    window.addEventListener("resize", () => {
      collision.update_canvas_size();
    });
    // main loop
    function updateAndDraw(): void {
      // integrate in time and draw a field
      collision.update();
      // set myself as the callback
      requestAnimationFrame(updateAndDraw);
    }
    updateAndDraw();
  });
});

