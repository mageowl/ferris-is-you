import { LEVEL_HEIGHT, LEVEL_WIDTH, ObjectType, sprite } from "./consts.js";
import { currentTool } from "./tools.js";

const grid = document.querySelector("#grid-container .grid");

export let gridData = new Array(LEVEL_WIDTH * LEVEL_HEIGHT).fill({
  type: ObjectType.EMPTY,
});
let history = [[...gridData]];
let historyIdx = 0;
let mouseDown = false;

for (let i = 0; i < LEVEL_WIDTH * LEVEL_HEIGHT; i++) {
  let el = document.createElement("div");

  el.addEventListener("mouseenter", () => {
    el.style.backgroundImage = currentTool.preview();
    if (mouseDown) {
      currentTool.action(gridData, i);
    }
  });

  el.addEventListener("mouseleave", () => {
    el.style.backgroundImage = sprite(gridData[i]);
  });

  el.addEventListener("mousedown", () => {
    currentTool.action(gridData, i);
  });

  grid.append(el);
}

window.addEventListener("mousedown", () => {
  mouseDown = true;
});
window.addEventListener("mouseup", () => {
  mouseDown = false;

  if (JSON.stringify(history.at(-1)) != JSON.stringify(gridData)) {
    history = history.slice(0, historyIdx + 1).concat([[...gridData]]);
    historyIdx += 1;
  }
});

export function back() {
  historyIdx -= 1;
  if (historyIdx < 0) historyIdx = 0;

  gridData = history[historyIdx];
  Array.from(grid.children).forEach((el, i) => {
    el.style.backgroundImage = sprite(gridData[i]);
  });
}

export function forward() {
  historyIdx += 1;
  if (historyIdx >= history.length) historyIdx = history.length - 1;

  gridData = history[historyIdx];
}
