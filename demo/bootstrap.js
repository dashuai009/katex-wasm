import("./index.js").catch((error) => {
  const message = document.createElement("p");
  message.setAttribute("role", "alert");
  message.textContent = `Failed to load renderers: ${error.message}`;
  document.body.prepend(message);
  window.__PERF_RESULT__ = { error: String(error) };
  window.__PERF_DONE__ = true;
});
