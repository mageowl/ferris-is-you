import { OBJECTS, PROPERTIES, ObjectType } from "./consts.js";

const palette = document.querySelector("#palette");
export let selected = { type: ObjectType.GENERIC, id: 1 };

OBJECTS.forEach((name, id) => {
  if (name === "text") return;

  let el = document.createElement("span");

  el.innerText = name;
  el.addEventListener("click", () => {
    selected = { type: ObjectType.GENERIC, id };
  });

  palette.insertBefore(el, palette.querySelector("h3.text"));
});

OBJECTS.forEach((name, id) => {
  let el = document.createElement("span");

  el.innerText = `[noun] ${name}`;
  el.addEventListener("click", () => {
    selected = { type: ObjectType.NOUN, id };
  });

  palette.append(el);
});

PROPERTIES.forEach((name, id) => {
  let el = document.createElement("span");

  el.innerText = `[property] ${name}`;
  el.addEventListener("click", () => {
    selected = { type: ObjectType.PROPERTY, id };
  });

  palette.append(el);
});

let isEl = document.createElement("span");
isEl.innerText = `is`;
isEl.addEventListener("click", () => {
  selected = { type: ObjectType.IS, id: 0 };
});
palette.append(isEl);
