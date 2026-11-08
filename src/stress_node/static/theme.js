// Theme: stored choice, else the OS preference. Load in <head> (no defer)
// so the theme is set before the first paint. The click handler is delegated
// on document, so it keeps working after hx-boost swaps the page body.
(function () {
  var root = document.documentElement;

  try {
    var stored = localStorage.getItem("theme");
    var prefersDark = window.matchMedia && matchMedia("(prefers-color-scheme: dark)").matches;
    if (stored === "dark" || (!stored && prefersDark)) {
      root.setAttribute("data-theme", "dark");
    }
  } catch (e) {}

  document.addEventListener("click", function (event) {
    if (!event.target.closest(".theme-toggle")) return;
    var isDark = root.getAttribute("data-theme") === "dark";
    if (isDark) {
      root.removeAttribute("data-theme");
    } else {
      root.setAttribute("data-theme", "dark");
    }
    try {
      localStorage.setItem("theme", isDark ? "light" : "dark");
    } catch (e) {}
  });
})();
