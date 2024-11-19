import wbgInit, { Collision } from "../pkg";
import { getNumber } from "./urlSearchParams";

window.addEventListener("load", () => {
  wbgInit()
    .then(() => {
      // all things which should be done before iterating
      const length: number = getNumber("length", 192, 8, 1024);
      const nitems: number = getNumber("nitems", 8192, 2, 32768);
      const rate: number = getNumber("rate", 1e-1, 1e-4, 1e3);
      console.log(`length: ${length.toExponential()}`);
      console.log(`number of particles: ${nitems.toString()}`);
      console.log(`draw rate: ${rate.toString()}`);
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
    })
    .catch((error: unknown) => {
      console.error(error);
    });
});
