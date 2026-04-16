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
    constructor(animation_cb, start_cb = null, delay = 0) {
        this.animation_frame = null;
        this.pending_timer = null;
        this.start_cb = null;
        this.cancel_cb = null;
        this.start_time_ms = 0;
        this.last_time_ms = 0;
        this.animation_cb = animation_cb;
        this.start_cb = start_cb;
    }
    restart(delay_ms = 0, start_cb) {
        this.start_cb = start_cb;
        this.schedule(delay_ms);
    }
    schedule_at(when_ms, cb) {
        var delay = performance.now() - when_ms;
        if (delay < 0) {
            delay = 0;
        }
        this.schedule(delay, cb);
    }
    schedule(delay_ms = 0, cb) {
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
            this.pending_timer = window.setTimeout(() => this.animate(performance.now()), delay_ms);
        }
        else {
            this.animation_frame = requestAnimationFrame((time) => this.animate(time));
        }
    }
    animate(time) {
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
        }
        else {
            this.animation_cb(time);
        }
    }
}
