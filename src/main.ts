import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.greet = greet;


async function openDialog () {
  let filepath  = document.querySelector(".js-filepath") as HTMLInputElement;
  open().then((fpath) => {
    // console.log(fpath);
    // console.log(filepath);
    if(fpath){
      filepath.value =  fpath.toString();
      invoke("simple_command", {
        filepath: fpath,
      });
    }
  });
}
window.openDialog = openDialog;