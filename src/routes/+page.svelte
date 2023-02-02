<script>
  import Greet from '$lib/Greet.svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let flag = true;
  let dflag = false;
  async function toggleClickThrough() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke('set_click_through', { flag });
    flag = !flag;
  }
  async function toggleDecorations() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke('set_decorations', { dflag });
    dflag = !dflag;
  }
</script>

<h1>Welcome to Tauri!</h1>

<div class="row">
  <p on:click|preventDefault={toggleDecorations}>decorations</p>
  <p on:click|preventDefault={toggleClickThrough}>click through</p>
  <a href="https://vitejs.dev" target="_blank">
    <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
  </a>
  <a href="https://tauri.app" target="_blank" >
    <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
  </a>
  <a href="https://kit.svelte.dev" target="_blank">
    <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
  </a>
</div>

<p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

<div class="row">
  <Greet />
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
