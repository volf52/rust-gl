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

function sendImage(url: string): HTMLImageElement {
  const image = new Image();
  image.src = url;
  return image;
}

export function load_texture(
  gl: WebGL2RenderingContext,
  url: string,
): WebGLTexture {
  const texture = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, texture);

  // Because images have to be download over the internet
  // they might take a moment until they are ready.
  // Until then put a single pixel in the texture so we can
  // use it immediately. When the image has finished downloading
  // we'll update the texture with the contents of the image.
  const level = 0;
  const internalFormat = gl.RGBA;
  const width = 1;
  const height = 1;
  const border = 0;
  const srcFormat = gl.RGBA;
  const srcType = gl.UNSIGNED_BYTE;
  const pixel = new Uint8Array([0, 0, 255, 255]); // opaque blue
  gl.texImage2D(
    gl.TEXTURE_2D,
    level,
    internalFormat,
    width,
    height,
    border,
    srcFormat,
    srcType,
    pixel,
  );

  const image = new Image();
  image.onload = function () {
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(
      gl.TEXTURE_2D,
      level,
      internalFormat,
      srcFormat,
      srcType,
      image,
    );

    // WebGL1 has different requirements for power of 2 images
    // vs non power of 2 images so check if the image is a
    // power of 2 in both dimensions.

    gl.generateMipmap(gl.TEXTURE_2D);
  };
  image.src = url;

  return texture!!;
}
