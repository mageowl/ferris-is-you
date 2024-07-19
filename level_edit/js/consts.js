export const LEVEL_WIDTH = 30;
export const LEVEL_HEIGHT = 20;
export const OBJECTS = ["text", "ferris", "wall", "flag", "tile", "rock"];
export const PROPERTIES = ["you", "push", "stop", "win"];

export const ObjectType = {
  EMPTY: 0,
  NOUN: 1,
  IS: 2,
  PROPERTY: 3,
  GENERIC: 4,
};

const bg = (url) => `url(../assets/sprites/${url}.png)`;
export const sprite = ({ type, id }) => {
  switch (type) {
    case ObjectType.GENERIC: {
      return bg("obj/" + OBJECTS[id]);
    }
    case ObjectType.NOUN: {
      return bg("text/noun/" + OBJECTS[id] + "_on");
    }
    case ObjectType.IS: {
      return bg("text/is_on");
    }
    case ObjectType.PROPERTY: {
      return bg("text/prop/" + PROPERTIES[id] + "_on");
    }

    case ObjectType.EMPTY: {
      return "none";
    }
  }
};
