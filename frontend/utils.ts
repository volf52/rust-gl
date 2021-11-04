export const getGlContext = (canvasId: string) => {
  const canvas = document.getElementById(canvasId);

  if (canvas === null) return null;

  const ctx = (canvas as HTMLCanvasElement).getContext("webgl2");

  return ctx;
};
