import {
  LEVEL_WIDTH,
  OBJECTS,
  ObjectType,
  PROPERTIES,
  sprite,
} from "./consts.js";
import { dialog } from "./dialog.js";
import { selected } from "./palette.js";

export class Tool {
  static Draw = new Tool()
    .withPreview(() => sprite(selected))
    .withAction((gridData, i) => (gridData[i] = selected));

  static Erase = new Tool()
    .withPreview(() => "none")
    .withAction((gridData, i) => (gridData[i] = { type: ObjectType.EMPTY }));

  static HText = new Tool()
    .withPreview(() => "url(./arrow-right.svg)")
    .withAction(async (gridData, i) => {
      let text = await dialog();

      text.split(" ").forEach((w, j) => {
        let isNoun = OBJECTS.includes(w);
        let isProperty = PROPERTIES.includes(w);
        let isIs = w === "is";

        gridData[i + j] = {
          type: isNoun
            ? ObjectType.NOUN
            : isProperty
              ? ObjectType.PROPERTY
              : isIs
                ? ObjectType.IS
                : ObjectType.EMPTY,
          id: isNoun
            ? OBJECTS.indexOf(w)
            : isProperty
              ? PROPERTIES.indexOf(w)
              : 0,
        };
      });

      Array.from(document.querySelector(".grid").children).forEach((el, i) => {
        el.style.backgroundImage = sprite(gridData[i]);
      });
    });

  static VText = new Tool()
    .withPreview(() => "url(./arrow-down.svg)")
    .withAction(async (gridData, i) => {
      let text = await dialog();

      text.split(" ").forEach((w, j) => {
        let isNoun = OBJECTS.includes(w);
        let isProperty = PROPERTIES.includes(w);
        let isIs = w === "is";

        gridData[i + j * LEVEL_WIDTH] = {
          type: isNoun
            ? ObjectType.NOUN
            : isProperty
              ? ObjectType.PROPERTY
              : isIs
                ? ObjectType.IS
                : ObjectType.EMPTY,
          id: isNoun
            ? OBJECTS.indexOf(w)
            : isProperty
              ? PROPERTIES.indexOf(w)
              : -1,
        };
      });

      Array.from(document.querySelector(".grid").children).forEach((el, i) => {
        el.style.backgroundImage = sprite(gridData[i]);
      });
    });

  preview = () => "";
  action = () => undefined;

  withPreview(fn) {
    this.preview = fn;
    return this;
  }

  withAction(fn) {
    this.action = fn;
    return this;
  }
}

export let currentTool = Tool.Draw;

const select =
  (tool, shiftTool = tool) =>
  (e) => {
    currentTool = e.shiftKey ? shiftTool : tool;
    document.querySelector(".selected")?.classList.remove("selected");
    e.target.classList.add("selected");
  };

document
  .querySelector("#tool-draw")
  .addEventListener("click", select(Tool.Draw));
document
  .querySelector("#tool-erase")
  .addEventListener("click", select(Tool.Erase));
document
  .querySelector("#tool-text")
  .addEventListener("click", select(Tool.HText, Tool.VText));

export const setTool = (tool, el) => {
  currentTool = tool;
  document.querySelector(".selected")?.classList.remove("selected");
  el.classList.add("selected");

  document.querySelector(".grid div:hover").style.backgroundImage =
    currentTool.preview();
};
