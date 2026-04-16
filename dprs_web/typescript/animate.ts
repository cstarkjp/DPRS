/*

Example requestAnimationFrame

this.anim = new Animate( (time) => this.anim_tick(time) );
this.tick = 0;
this.anim.schedule();

anim_tick(_time:number): void {
  if (this.tick < this.fred.length) {
    this.redraw(this.tick);
    this.tick = this.tick + 1;
    this.anim.schedule(100);
  }
}
anim_restart(): void {
  this.tick = 0;
  this.anim_tick(0);
}
*/

export class Animate {
  private animation_frame: number | null = null;
  private pending_timer: number | null = null;
  private start_cb: null | ((time: number) => void) = null;
  private animation_cb: (time: number) => void;
  private cancel_cb: null | ((time: number) => void) = null;
  start_time_ms: number = 0;
  last_time_ms: number = 0;

  constructor(
    animation_cb: (time: number) => void,
    start_cb: null | ((time: number) => void) = null,
    delay: number = 0,
  ) {
    this.animation_cb = animation_cb;
    this.start_cb = start_cb;
  }

  duration(): number {
    return this.last_time_ms - this.start_time_ms;
  }

  restart(delay_ms: number = 0, start_cb: (time: number) => void) {
    this.start_cb = start_cb;
    this.schedule(delay_ms);
  }

  schedule_at(when_ms: number, cb?: (time: number) => void) {
    var delay = performance.now() - when_ms;
    if (delay < 0) {
      delay = 0;
    }
    this.schedule(delay, cb);
  }

  schedule(delay_ms: number = 0, cb?: (time: number) => void) {
    if (cb !== undefined) {
      this.animation_cb = cb;
    }
    if (this.pending_timer !== null) {
      window.clearTimeout(this.pending_timer);
      this.pending_timer = null;
    }
    if (this.animation_frame !== null) {
      window.cancelAnimationFrame(this.animation_frame);
      this.animation_frame = null;
    }
    if (delay_ms > 0) {
      this.pending_timer = window.setTimeout(
        () => this.animate(performance.now()),
        delay_ms,
      );
    } else {
      this.animation_frame = requestAnimationFrame((time) =>
        this.animate(time),
      );
    }
  }

  private animate(time: number) {
    this.animation_frame = null;
    this.pending_timer = null;

    this.last_time_ms = time;
    if (this.start_cb !== null) {
      this.start_time_ms = time;
    }

    if (this.cancel_cb !== null) {
      this.cancel_cb(time);
      return;
    }
    if (this.start_cb !== null) {
      const cb = this.start_cb;
      this.start_cb = null;
      cb(time);
    } else {
      this.animation_cb(time);
    }
  }
}
