import config from "../data/config";
import { invoke } from "@tauri-apps/api/tauri";

export function run() {
  const src = config.codeGetter();
  const lang = config.activeLang;

  if (config.direction === "assembly") {
    invoke("cmd_assemble", { msg: `{"arch": "${lang}", "data": "${src}"}` })
      .then((m) => {
        console.log("assembly ok: ", m);
        config.resultSetter(m);
      })
      .catch((e) => {
        console.error("assembly error.", e);
      });
  } else if (config.direction === "disassembly") {
    invoke("cmd_disassemble", { msg: `{"arch": "${lang}", "data": "${src}"}` })
      .then((m) => {
        console.log("disassembly ok: ", m);
        config.resultSetter(m);
      })
      .catch((e) => {
        console.error("disassembly error.", e);
      });
  } else {
    console.error("unknwon direction ", config.direction);
  }
}
