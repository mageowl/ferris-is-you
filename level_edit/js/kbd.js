import { back, forward } from "./grid.js";
import { Tool, setTool } from "./tools.js";

window.addEventListener(
  "keydown",
  ({ key, ctrlKey, metaKey, shiftKey, altKey }) => {
    let modKey = ctrlKey || metaKey;
    let anyMod = modKey || altKey;

    if (key === "b" && !anyMod) {
      setTool(Tool.Draw, document.querySelector("#tool-draw"));
    } else if (key === "e" && !anyMod) {
      setTool(Tool.Erase, document.querySelector("#tool-erase"));
    } else if (key.toLowerCase() === "t" && !anyMod) {
      setTool(
        shiftKey ? Tool.VText : Tool.HText,
        document.querySelector("#tool-text"),
      );
    } else if (key === "s" && modKey) {
      document.querySelector("#btn-save").click();
    } else if (key === "z" && modKey) {
      console.log("ctrl+z");
      if (shiftKey) forward();
      else back();
    }
  },
);
