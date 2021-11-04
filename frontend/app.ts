import { GlProgram, start, ShaderConstant, Square, BufferInfo } from "rust-gl";
import { getGlContext } from "./utils";

const CANVAS_ID = "canvas";

const main = () => {
  const context = getGlContext(CANVAS_ID);
  if (context === null) {
    console.error("Failed to acquire Gl Context");
    return;
  }

  const prog = GlProgram.new(context);
  console.log(
    prog.get_attrib_loc(ShaderConstant[ShaderConstant.ATextureCoord]),
  );

  console.log(
    prog.get_attrib_loc(ShaderConstant[ShaderConstant.UProjectionMatrix]),
  );

  const a = prog.get_uniform_loc(
    ShaderConstant[ShaderConstant.UProjectionMatrix],
  );
  const b = prog.get_uniform_loc(
    ShaderConstant[ShaderConstant.UProjectionMatrix],
  );
  const c = prog.get_uniform_loc(ShaderConstant[ShaderConstant.USampler]);

  console.log(a === b);
  console.log(a === c);

  const sq = Square.new(3);
  console.log(sq.color());

  const squareInfo = BufferInfo.create_buffer_info(prog.context(), sq);

  // start(context);
};

main();
