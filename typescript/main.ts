import collision_start, { Collision } from "./collision.js";

let collision_obj: Collision;

function get_and_set(keyword: string, defval: number, minval: number, maxval: number): number {
  // check if URL param is given
  const url_params = new URLSearchParams(window.location.search);
  // if not given, use default value
  let val: number = defval;
  if (url_params.has(keyword)) {
    // if given, use after sanitised
    let tmp: number | null = Number(url_params.get(keyword));
    if (tmp) {
      tmp = tmp < minval ? minval : tmp;
      tmp = maxval < tmp ? maxval : tmp;
      val = tmp;
    }
  }
  return val;
}

function update_and_draw(): void {
  // integrate in time and draw a field
  collision_obj.update();
  // set myself as the callback
  requestAnimationFrame(update_and_draw);
}

window.addEventListener(`load`, () => {
  collision_start().then(() => {
    // all things which should be done before iterating
    const length: number = get_and_set(`length`, 192., 8., 1024.);
    const nitems: number = get_and_set(`nitems`, 8192, 2, 32768);
    const rate: number = get_and_set(`rate`, 1e-1, 1e-4, 1e3);
    console.log(`length: ${length.toExponential()}`);
    console.log(`number of particles: ${nitems}`);
    console.log(`draw rate: ${rate}`);
    // initialise simulator and drawer
    collision_obj = Collision.new(length, nitems, rate, Math.random());
    // initialise window / canvas size
    collision_obj.update_canvas_size();
    // register it to an event handler
    window.addEventListener(`resize`, () => {
      collision_obj.update_canvas_size();
    });
    // trigger first animation flow
    update_and_draw();
  });
});

