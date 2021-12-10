import { test_error, main as RGL } from "rust-gl";

const CANVAS_ID = "canvas";

// const main = () => {
// const gl = getGlContext(CANVAS_ID);
// if (gl === null) {
//   console.error("Failed to acquire Gl Context");
//   return;
// }

// start(context);
// };
// test_error();

RGL();
