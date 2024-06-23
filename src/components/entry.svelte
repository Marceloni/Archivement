<script lang="ts">
  import { onMount } from "svelte";
  import type { EntrySettings, ContentPiece } from "../entrySettings"
  export let settings: EntrySettings
  import { appDataDir, join } from "@tauri-apps/api/path"
  import { convertFileSrc } from "@tauri-apps/api/core"
</script>

<div class="entry">
  <h2 class="entry-title">{settings.title}</h2>
  <div class="content-preview">
    {#each settings.content as contentPiece}
      {#if contentPiece.type === "image"}<img src={contentPiece.path} alt="Image">{/if}
      {#if contentPiece.type === "text"}<p>{contentPiece.text}</p>{/if}
      {#if contentPiece.type === "video"}
        <video controls>
          <source src={contentPiece.path} type="video/mp4">
        </video>
      {/if}
      {#if contentPiece.type === "audio"}
        <audio controls>
          <source src={contentPiece.path} type="audio/mpeg">
        </audio>
      {/if}
    {/each}
  </div>
</div>

<style>

  .entry {
    background-color: #7e7e7e;
    padding: 0.5rem;
    margin: 0.5rem;
    border: 3px solid rgb(51, 51, 51);
    text-align: center;
    width: auto;
    box-sizing: border-box;
  }
  .entry-title {
    margin: 0;
    padding-bottom: 0.25rem;
    margin-bottom: 0.5rem;
    border-bottom: 3px solid rgb(51, 51, 51);
  }
  .content-preview {
    margin: 0.25rem;
    width: auto;
    display: grid;
    justify-content: center;
    align-items: center;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(2, 1fr);
    gap: 0.25rem; /* Optional: Add space between the grid items */
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
  .content-preview img:nth-child(1) {
    grid-column: 1 / 3; /* Span 2 columns */
    grid-row: 1 / 3;    /* Span 2 rows */
  }
  .content-preview img:nth-child(2) {
    grid-column: 3 / 4;
    grid-row: 1 / 2;
  }
  .content-preview img:nth-child(3) {
    grid-column: 4 / 4;
    grid-row: 1 / 2;
  }
  .content-preview img:nth-child(4) {
    grid-column: 3 / 4;
    grid-row: 2 / 2;
  }
  .content-preview img:nth-child(5) {
    grid-column: 4 / 4;
    grid-row: 2 / 2;
  }
</style>
