import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import p5 from "p5";

console.log("p5.js loaded");

let settings = {};
async function fetchSettings() {
  settings = await invoke("get_settings");
  console.log("settings initialized");
}
fetchSettings();

listen("settings-updated", (event) => {
  settings = event.payload;
  console.log("settings updated");
});

let particles = [];
const sketch = (p) => {
  let gravityCenter;

  p.setup = () => {
    p.createCanvas(p.windowWidth, p.windowHeight);
    console.log(p.windowWidth);
    p.clear();
    gravityCenter = p.createVector(p.width / 2, p.height / 2);
    p.textFont('monospace');
    p.colorMode(p.HSB, 360, 100, 100, 100);
    console.log("setup finished");
    invoke("show_window");
  };

  p.windowResized = () => {
    p.resizeCanvas(p.windowWidth, p.windowHeight);
    console.log(p.windowWidth);
  };

  p.draw = () => {
    p.clear();

    for (let i = particles.length - 1; i >= 0; i--) {
      let p = particles[i];
      p.applyGravity(gravityCenter);
      p.update();
      p.show();
      if (p.lifespan <= 0) particles.splice(i, 1);
    }
  };

  p.mouseMoved = () => {
    gravityCenter.set(p.mouseX, p.mouseY);
  };

  p.keyPressed = () => {
    // console.log(p.key);
    let particle = new Particle(p, p.random(p.width), p.random(p.height), p.key);
    particles.push(particle);
  };

  listen("global-key", (event) => {
    // console.log(event.payload);
    let _key = event.payload;
    if (_key.startsWith("Key") || _key.startsWith("Num")) {
      _key = _key.slice(3);
    }
    particles.push(new Particle(p, p.random(p.width), p.random(p.height), _key));
  });

  listen("mouse-move", (event) => {
    const [x, y] = event.payload;
    // p.mousePos.x = x;
    // p.mousePos.y = y;
    gravityCenter.set(x / window.devicePixelRatio, y / window.devicePixelRatio);
  });

};

new p5(sketch);

class Particle {
  constructor(p, x, y, k) {
    this.p = p;

    this.pos = p.createVector(x, y);
    this.vel = p5.Vector.random2D().mult(p.random(1, 3));
    this.acc = p.createVector(0, 0);
    this.lifespan = 255; // alpha
    this.char = k;
    this.size = p.random(settings.particle_size, settings.particle_size * 2 / this.char.length);
    this.rotate = p.random(360);

    let codeValue = this.char.charCodeAt(this.char.length - 1);
    this.hueValue = (p.map(codeValue, 32, 126, 0, 360) + settings.hue_offset) % 360;
  }

  applyGravity(center) {
    let dir = p5.Vector.sub(center, this.pos);
    let distanceValue = this.p.constrain(dir.mag(), 50, 300);
    dir.normalize();
    let force = settings.gravity_strength * 200 / (distanceValue * distanceValue);
    dir.mult(force);
    this.acc.add(dir);
  }

  update() {
    this.vel.add(this.acc);
    this.pos.add(this.vel);
    this.acc.mult(0);

    this.lifespan -= settings.life_decay;
    this.vel.mult(0.98);
    this.rotate += (0.0001 * settings.rotate_speed * this.lifespan);

    if (this.pos.x < 0 || this.pos.x > this.p.width)
      this.vel.x *= -1;

    if (this.pos.y < 0 || this.pos.y > this.p.height)
      this.vel.y *= -1;
  }

  show() {
    const p = this.p;

    p.fill(this.hueValue, 80, 80, this.lifespan);
    p.noStroke();
    p.textSize(this.size);
    p.textAlign(p.CENTER, p.CENTER);

    p.push();
    p.translate(this.pos.x, this.pos.y);
    p.rotate(this.rotate);
    p.text(this.char, 0, 0);
    p.pop();

    for (let other of particles) {
      if (other !== this) {
        let d = p.dist(
          this.pos.x,
          this.pos.y,
          other.pos.x,
          other.pos.y
        );

        if (d < (this.size + other.size) / 2) {
          p.stroke(255, 50);
          p.line(
            this.pos.x,
            this.pos.y,
            other.pos.x,
            other.pos.y
          );
        }
      }
    }
  }
}