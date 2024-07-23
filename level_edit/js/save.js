import { LEVEL_HEIGHT, LEVEL_WIDTH, ObjectType } from "./consts.js";
import { gridData, loadData } from "./grid.js";

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

document.querySelector("#btn-open").addEventListener("click", () => {
  document.querySelector("#file-dialog").click();
});

document.querySelector("#file-dialog").addEventListener("change", async (e) => {
  const file = e.target.files[0];
  const bytes = await file.bytes();
  let i = 0;
  let newData = [];

  for (let _ = 0; _ < LEVEL_WIDTH * LEVEL_HEIGHT; _++) {
    let objClass = bytes[i++];

    if (objClass === 0) {
      newData.push({ type: ObjectType.EMPTY });
      continue;
    }

    let idBytes = bytes.slice(i, i + 8);
    i += 8;

    let idBig = 0n;
    for (const int of idBytes.values()) {
      const bint = BigInt(int);
      idBig = (idBig << 8n) + bint;
    }
    let id = Number(idBig);

    switch (objClass) {
      case 1: {
        newData.push({ type: ObjectType.NOUN, id });
        break;
      }
      case 2: {
        newData.push({ type: ObjectType.IS });
        break;
      }
      case 3: {
        newData.push({ type: ObjectType.PROPERTY, id });
        break;
      }
      case 4: {
        newData.push({ type: ObjectType.GENERIC, id });
        break;
      }
      default: {
        console.error(objClass);
      }
    }
  }

  loadData(newData);
});
