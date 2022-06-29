import init, { set_theme1, set_theme2 } from "./abridge_wasm.js";
await init();

const btn_theme1 = document.getElementById("btn_theme1");
const btn_theme2 = document.getElementById("btn_theme2");
btn_theme1.addEventListener("click", event => set_theme1());
btn_theme2.addEventListener("click", event => set_theme2());