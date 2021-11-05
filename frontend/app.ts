import { GlProgram, Square, BufferInfo, hey } from "rust-gl";
import { getGlContext } from "./utils";

const CANVAS_ID = "canvas";

const main = () => {
  const gl = getGlContext(CANVAS_ID);
  if (gl === null) {
    console.error("Failed to acquire Gl Context");
    return;
  }

  hey();

  const prog = GlProgram.new(gl, gl.canvas.width, gl.canvas.height);

  const sq = Square.new(0.5);

  const squareInfo = BufferInfo.create_buffer_info(gl, sq);

  prog.draw(sq, squareInfo);

  // start(context);
};

main();
