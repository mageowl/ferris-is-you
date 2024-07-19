let cb = null;

export function dialog() {
  if (cb != null) return;
  document
    .querySelectorAll("#dialog, #scrim")
    .forEach((e) => e.classList.add("show"));
  return new Promise((res) => (cb = res));
}

const dialogEl = document.querySelector("#dialog");
dialogEl.addEventListener("submit", (e) => {
  e.preventDefault();
  document
    .querySelectorAll("#dialog, #scrim")
    .forEach((e) => e.classList.remove("show"));

  cb(dialogEl.querySelector("input").value);
  dialogEl.querySelector("input").value = "";
  cb = null;
});
dialogEl.addEventListener("keydown", (e) => {
  e.stopPropagation();
});
