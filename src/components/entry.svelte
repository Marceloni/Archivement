<script lang="ts">
  import type { EntrySettings } from "../entrySettings"
  export let settings: EntrySettings
  import { invoke } from "@tauri-apps/api/core"
  import { goto } from "$app/navigation";

  async function openEntry() {
    goto("/entry_content/" + encodeURIComponent(JSON.stringify(settings)))
  }
</script>

<div class="entry" on:click={openEntry}>
  <h2 class="entry-title">{settings.title}</h2>
  <div class="content-preview">
    {#each settings.content.slice(0, 5) as contentPiece}
      {#if contentPiece.type === "image"}<img src={contentPiece.path} alt="Image" class="content-preview-element">{/if}
      {#if contentPiece.type === "text"}<p class="content-preview-element">{contentPiece.text}</p>{/if}
      {#if contentPiece.type === "video"}
        <video controls class="content-preview-element">
          <source src={contentPiece.path}>
        </video>
      {/if}
      {#if contentPiece.type === "audio"}
        <audio controls class="content-preview-element">
          <source src={contentPiece.path}>
        </audio>
      {/if}
    {/each}
  </div>
</div>

<style>
  .entry {
    background-color: var(--secondary);
    padding: 0.5rem;
    margin: 0.5rem;
    border: 0.25rem solid var(--primary);
    border-radius: 1rem;
    text-align: center;
    width: auto;
    box-sizing: border-box;
  }
  .entry-title {
    margin: 0;
    padding-bottom: 0.25rem;
    margin-bottom: 0.5rem;
    border-bottom: 3px solid var(--text);
  }
  .content-preview {
    margin: 0.25rem;
    width: auto;
    display: grid;
    justify-content: center;
    align-items: center;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(2, 1fr);
    gap: 0.5rem;
  }
  .content-preview p {
    user-select: none;
    -webkit-user-select: none;
    -webkit-user-drag: none;
    width: 100%;
    height: auto;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .content-preview img, .content-preview video {
    user-select: none;
    -webkit-user-select: none;
    -webkit-user-drag: none;
    width: 100%;
    height: auto;
  }
  .content-preview audio{
    user-select: none;
    -webkit-user-select: none;
    -webkit-user-drag: none;
    width: 100%;
  }
  .content-preview-element:nth-child(1) {
    grid-column: 1 / 3;
    grid-row: 1 / 3;
  }
  .content-preview-element:nth-child(2) {
    grid-column: 3 / 4;
    grid-row: 1 / 2;
  }
  .content-preview-element:nth-child(3) {
    grid-column: 4 / 4;
    grid-row: 1 / 2;
  }
  .content-preview-element:nth-child(4) {
    grid-column: 3 / 4;
    grid-row: 2 / 2;
  }
  .content-preview-element:nth-child(5) {
    grid-column: 4 / 4;
    grid-row: 2 / 2;
  }
</style>
