import { ObjectType } from "./consts.js";
import { gridData } from "./grid.js";

function u64bytes(int) {
  // we want to represent the input as a 8-bytes array
  var byteArray = [0, 0, 0, 0, 0, 0, 0, 0];

  for (var index = 0; index < byteArray.length; index++) {
    var byte = int & 0xff;
    byteArray[index] = byte;
    int = (int - byte) / 256;
  }

  // Use big endian
  return byteArray.reverse();
}

document.querySelector("#btn-save").addEventListener("click", () => {
  let bytes = new Uint8Array(
    gridData.flatMap((t) => {
      switch (t.type) {
        case ObjectType.EMPTY:
          return [0];
        case ObjectType.IS:
          return [2, 0, 0, 0, 0, 0, 0, 0, 0];
        default:
          return [t.type, ...u64bytes(t.id)];
      }
    }),
  );

  let blob = new Blob([bytes], { type: "application/octet-stream" });

  let anchor = document.createElement("a");
  anchor.href = URL.createObjectURL(blob);
  anchor.download = "my_level.dat";
  anchor.click();
});
