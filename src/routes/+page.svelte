<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core"
  import Entry from "../components/entry.svelte"
  import type { EntrySettings } from "../entrySettings"
  import {convertFileSrc} from "@tauri-apps/api/core"
  import { appDataDir, join } from "@tauri-apps/api/path";
  import { event } from "@tauri-apps/api";
  function addEntry() {
    goto("/create")
  }

  onMount(async ()=>{loadEntries()})

  event.listen("reload_entries", async () => {loadEntries()})

  let entryList: HTMLElement

  async function loadEntries() {
    entryList.innerHTML = ""

    let settings_files: EntrySettings[] = await invoke("read_entries")
    settings_files.forEach(async (settings) => {
      settings.content = await Promise.all(settings.content.map(async (contentPiece) => {
        if (contentPiece.type === "image" || contentPiece.type === "video" || contentPiece.type === "audio") {
          contentPiece.path = await convertFileSrc(await join(await appDataDir(), "entries", settings.uuid, "content", contentPiece.path as string))
        }
        return contentPiece
      }))
      
      new Entry({target: entryList, props: {settings}})
    })
  }

  let menuDropdownShown = false
  window.onclick = function(event) {
    if (!(event.target as HTMLElement).matches("#menu-button")) {
      menuDropdownShown = false
    }
  } 
  function openMenuDropdown() {
    menuDropdownShown =! menuDropdownShown
  }
  function menuAction(event: MouseEvent) {
    switch ((event.target as HTMLElement).dataset.action) {
      case "import":
        invoke("import_entries")
        break;
      case "export":
        invoke("export_entries")
        break;
    }
  }
</script>

<div id="home-page">
  <div id="top-div">
    <h1>Welcome to Archivement</h1>
    <div on:click={openMenuDropdown} class="icon" id="menu-button" style="mask-image: url('src/assets/icons/menu.svg'); -webkit-mask-image: url('src/assets/icons/menu.svg');"/>
    <div id="menu-dropdown" class={menuDropdownShown?"show":""} on:click={menuAction}>
      <p data-action="import">Import</p>
      <p data-action="export">Export</p>
  </div>
  </div>

  <p>Archive your Achievements</p>

  <div id="entry-list" bind:this={entryList}>
    
  </div>

  <div id="footer">
    <div id="create-entry-button" on:click={addEntry}>
      <div class="icon" style="mask-image: url('src/assets/icons/plus.svg'); -webkit-mask-image: url('src/assets/icons/plus.svg'); height: 100%"/>
    </div>
  </div>
</div>

<style>
  #menu-dropdown {
    display: none;
    background-color: var(--primary);
    box-shadow: 0rem 0rem 1rem var(--secondary);
    position: absolute;
    width: 5rem;
    z-index: 1;
    top: 4rem;
    right: 1rem;
    align-self: start;
  }
  #menu-dropdown p {
    cursor: pointer;
    color: inherit;
    padding: 0.5rem 0.5rem;
    text-decoration: none;
    margin: 0;
  }
  #menu-dropdown p:hover {
    background-color: var(--accent);
  }
  .show {
    display: block !important;
  }
  #top-div {
    display: flex;
    flex-direction: row;
    align-items: center;
  }
  #menu-button {
    width: 3rem;
    height: 3rem;
    margin-left: auto;
  }
  #menu-button:hover {
    cursor: pointer;
    background-color: var(--accent);
  }
  #top-div h1 {
    width: 100%;
    padding-left: 3rem;
  }
  #home-page {
    margin-bottom: 5rem;
  }

  h1, p {
    text-align: center;
  }

  #entry-list {
    display: grid;
    grid-template-columns: 1fr 1fr;
    justify-content: center;
    align-self: center;
    width: 100%;
  }

  @media only screen and (max-width: 700px) {
    #entry-list {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-self: center;
      width: 100%;
    }
  }
  
  #footer {
    position: fixed;
    left: 0;
    bottom: 0;
    width: 100%;
    height: 3rem;
    background-color: var(--secondary);
    align-items: center;
    justify-content: center;
    display: flex;
    padding-bottom: 0.5rem;
    padding-top: 0.5rem;
  }

  #create-entry-button {
    box-sizing: border-box;
    cursor: pointer;
    height: 3rem;
    width: 3rem;
    border-radius: 50%;
    border-color: var(--text);
    border-width: 4px;
    border-style: solid;
  }
  #create-entry-button:hover {
    background-color: var(--accent);
  }
</style>
