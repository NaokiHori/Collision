export class Timer {
  _counter: number;
  _increment: number;
  _start_at: number;

  constructor(increment: number) {
    this._counter = 0;
    this._increment = increment;
    this._start_at = 0;
  }

  public start() {
    this._start_at = performance.now();
  }

  public update() {
    this._counter += 1;
    if (this._increment < performance.now() - this._start_at) {
      console.log(
        `${this._counter.toString()} animation loops per ${this._increment.toString()} ms`,
      );
      this._counter = 0;
      this._start_at = performance.now();
    }
  }
}
