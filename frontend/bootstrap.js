import "./app.css";

import("./app.ts").catch((e) =>
  console.error("Error importing **app.ts**:", e),
);
