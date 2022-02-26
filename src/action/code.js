import config from "../data/config";

export function run() {
  const c = config.codeGetter();
  console.log("run: ", c);
  config.resultSetter(c);
}
