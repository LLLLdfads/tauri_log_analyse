<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
    import { onMount } from "svelte";

  let name = $state("");
  let greetMsg = $state("");
  let filePath = $state(""); // 修改为允许 null
  let fileContent = $state("");
  let contentLines = $state(["ddd",1,3,"ddd"]);

  async function greet(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
  async function f1() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    filePath = file ?? "";
    console.log(filePath);
  }
  async function parseFile(event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // fileContent = await invoke("parse_file", { filePath });    
    fileContent = await invoke("parse_file", { filePath });
  }
</script>

<main class="container">
  <button onclick={f1}>选择一个文件</button>
    {#if filePath===""}
    <span>请选择一个日志文件</span>
    {:else}
    <span>{filePath}</span>
    {/if}
  <button onclick="{parseFile}">开始解析</button>
  <div style="white-space: nowrap">{fileContent}</div>
  {#each contentLines as line,index}
  <div>{line}</div>
  {/each}

</main>

<style>
</style>
