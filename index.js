import init, { create_points } from "./pkg/l_systems_rs.js";

import predefined from "./figures.js";

var wrapper = document.getElementById("canvas-wrapper");
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
const systemInput = document.getElementById("system");
const angleInput = document.getElementById("angle");
const lengthInput = document.getElementById("length");
const depthInput = document.getElementById("depth");
const predefinedSelect = document.getElementById("predefined");
const drawBtn = document.getElementById("drawBtn");

ctx.canvas.width = wrapper.clientWidth;
ctx.canvas.height = wrapper.clientHeight;

var draw_point = { x: canvas.width / 2, y: canvas.height / 2, z: 0 };

function assign_parameters(selected) {
  systemInput.value = selected.value;
  angleInput.value = selected.angle;
  lengthInput.value = selected.length;
  depthInput.value = selected.depth;

  if (selected.center) {
    draw_point = selected.center;
  } else {
    draw_point = { x: canvas.width / 2, y: canvas.height / 2, z: 0 };
  }
}

function draw_points(points) {
  ctx.beginPath();
  for (let i = 0; i < points.length; i++) {
    const start = points[i][0];
    const end = points[i][1];
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
  }
  ctx.stroke();
}

function block_input() {
  predefinedSelect.disabled = true;
  drawBtn.disabled = true;
  drawBtn.innerHTML = "Loading...";
}

function allow_input() {
  predefinedSelect.disabled = false;
  drawBtn.disabled = false;
  drawBtn.innerHTML = "Draw";
}

function fetch_points() {
  return new Promise((resolve) => {
    setTimeout(() => {
      let angle = angleInput.value.trim();
      let depth = depthInput.value.trim();
      let length = lengthInput.value.trim();
      let system = systemInput.value.trim();

      let arg = {
        line_length: +length,
        recursion: +depth,
        angle: +angle,
        system,
        center_point: draw_point,
      };

      let output = create_points(JSON.stringify(arg));

      resolve(output);
    });
  });
}

async function clear_and_draw() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  block_input();
  let points = await fetch_points();
  draw_points(points);
  allow_input();
}

angleInput.addEventListener("change", clear_and_draw);
depthInput.addEventListener("change", clear_and_draw);
lengthInput.addEventListener("change", clear_and_draw);
systemInput.addEventListener("change", clear_and_draw);
drawBtn.addEventListener("click", async () => clear_and_draw);

predefinedSelect.addEventListener("change", async (e) => {
  let matching = predefined.find((x) => x.key == e.target.value);
  assign_parameters(matching);
  clear_and_draw();
});

async function run() {
  await init();

  for (let option of predefined) {
    let newOption = document.createElement("option");
    newOption.value = option.key;
    newOption.text = option.key;
    newOption.key = option.key;
    predefinedSelect.appendChild(newOption);
  }

  assign_parameters(predefined[0]);
  clear_and_draw();
}

run();
