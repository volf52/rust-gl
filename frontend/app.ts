import('rust-gl');
import { getGlContext } from "./utils";

const CANVAS_ID = "canvas";

const main = () => {
  const gl = getGlContext(CANVAS_ID);
  if (gl === null) {
    console.error("Failed to acquire Gl Context");
    return;
  }

  // start(context);
};

main();
