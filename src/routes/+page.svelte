<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core"
  import Entry from "../components/entry.svelte"
  import type { EntrySettings } from "../entrySettings"
  import {convertFileSrc} from "@tauri-apps/api/core"
  import { appDataDir, join } from "@tauri-apps/api/path";
  function addEntry() {
    goto("/create")
  }

  onMount(async ()=>{
    let settings_files: EntrySettings[] = await invoke("read_entries")
    settings_files.forEach(async (settings) => {
      settings.content = await Promise.all(settings.content.map(async (contentPiece) => {
        if (contentPiece.type === "image" || contentPiece.type === "video" || contentPiece.type === "audio") {
          contentPiece.path = await convertFileSrc(await join(await appDataDir(), "entries", settings.uuid, "content", contentPiece.path as string))
        }
        return contentPiece
      }))
      
      new Entry({target: document.getElementById("entry-list") as HTMLElement, props: {settings}})
    })
  })
  
</script>

<div id="home-page">
  <h1>Welcome to Archivement</h1>

  <p>Archive your Achievements</p>

  <div id="entry-list">
    
  </div>

  <div id="footer">
    <img alt="Create Entry" on:click={addEntry} id="create-entry-button" src="src/assets/icons/plus.svg">
  </div>
</div>

<style>
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
    background-color: var(--primary);
    align-items: center;
    justify-content: center;
    display: flex;
    padding-bottom: 0.5rem;
    padding-top: 0.5rem;
  }

  #create-entry-button {
    cursor: pointer;
    height: 3rem;
    width: 3rem;
    border-radius: 50%;
    border-color: black;
    border-width: 4px;
    border-style: solid;
    box-sizing: border-box;
    user-select: none;
    -webkit-user-select: none;
    -webkit-user-drag: none;
    fill: var(--text);
  }
  #create-entry-button:hover {
    filter: invert(1);
  }
</style>
