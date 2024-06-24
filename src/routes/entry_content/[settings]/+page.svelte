<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import type { EntrySettings } from "../../../entrySettings";
    import {open} from "@tauri-apps/plugin-dialog"
    import { event } from "@tauri-apps/api";
    import { appDataDir, join } from "@tauri-apps/api/path";
    /** @type {import('./$types').PageData} */
    export let data;
    let settings: EntrySettings = JSON.parse(data.settings)
    function revertContentPiece(e: MouseEvent) {
        let target = e.currentTarget as HTMLElement
        let contentPieceElement = target.parentElement?.querySelector(".content-piece-element") as HTMLInputElement
        let parentElement = target.parentElement as HTMLDivElement
        let index = parseInt(parentElement.dataset.index as string);
        let type = parentElement.dataset.type as string

        if (type === "text") {
            contentPieceElement.value = settings.content[index].text as string
        }else {
            contentPieceElement.setAttribute("src", settings.content[index].path as string)
        }
    }

    async function reloadContentPieces(uuid: String) {
        let new_settings: EntrySettings = await invoke("get_settings", {uuid})
        new_settings.content = await Promise.all(new_settings.content.map(async (contentPiece) => {
            if (contentPiece.type === "image" || contentPiece.type === "video" || contentPiece.type === "audio") {
                console.log(await convertFileSrc(await join(await appDataDir(), "entries", new_settings.uuid, "content", contentPiece.path as string)))
                contentPiece.path = await convertFileSrc(await join(await appDataDir(), "entries", new_settings.uuid, "content", contentPiece.path as string))
            }
            return contentPiece
        }))
        settings = new_settings
    }

    async function changeContentButton(e: MouseEvent) {
        let target = e.currentTarget as HTMLElement
        let contentPieceDiv = target.parentElement?.parentElement as HTMLDivElement;
        await invoke("change_content_piece", {uuid: settings.uuid, index: parseInt(contentPieceDiv.dataset.index as string), text: contentPieceDiv.dataset.type=="text" ? (contentPieceDiv.querySelector(".content-piece-element input") as HTMLInputElement).value : undefined})
    }

    event.listen("reload_entry", (event) => {
        reloadContentPieces(event.payload as string)
    })
  </script>
  <div id="content-edit-page">
      <h1 id="entry-title">{settings.title}</h1>
      <p id="entry-description">{settings.description}</p>
      <div id="content-pieces">
          {#each settings.content as contentPiece, index}
              <div class="content-piece-div" data-index={index} data-type={contentPiece.type}>
                  {#if contentPiece.type === "image"}
                      <img src={contentPiece.path} class="content-piece-element">
                  {/if}
                  {#if contentPiece.type === "text"}
                      <textarea class="content-piece-element">{contentPiece.text}</textarea>
                  {/if}
                  {#if contentPiece.type === "video"}
                  <video controls class="content-piece-element">
                      <source src={contentPiece.path} type="video/mp4">
                  </video>
                  {/if}
                  {#if contentPiece.type === "audio"}
                  <audio controls class="content-piece-element">
                      <source src={contentPiece.path} type="audio/mpeg">
                  </audio>
                  {/if}
                  
                  <div class="edit-buttons-div">
                      <img src="../src/assets/icons/reload.svg" class="reset-button" on:click={revertContentPiece}>
                      {#if contentPiece.type != "text"}
                          <img src="../src/assets/icons/file.svg" class="change-file-button" on:click={changeContentButton}>
                      {/if}
                  </div>
              </div>
          {/each}
      </div>
  </div>
  <style>
      :root {
          font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
          font-size: 16px;
          line-height: 24px;
          font-weight: 400;
          color: white;
          background-color: #494646;
          text-align: center;
      }
      #content-edit-page {
          display: flex;
          flex-direction: column;
          align-items: center;
      }
      #entry-description {
          padding-bottom: 1rem;
          border-bottom: 3px rgb(36, 36, 36) solid;
          width: 40rem;
      }
      p {margin: 0px}
      #content-pieces {
          width: 30rem;
      }
      .content-piece-element {
          width: calc(100% - 2rem);
      }
      .content-piece-div textarea {
          font-family: inherit;
          resize: none;
          height: 8rem;
      }
      .reset-button, .change-file-button {
          width: 2rem;
          height: 2rem;
          cursor: pointer;
      }
      .edit-buttons-div {
          display: flex;
          flex-direction: column;
          align-self: flex-start;
      }
      .reset-button:hover, .change-file-button:hover {
          filter: invert(1);
      }
      .content-piece-div {
          justify-content: center;
          display: flex;
          flex-direction: row;
          align-items: center;
          margin-top: 2rem;
          border: solid 3px rgb(36, 36, 36);
          width: 100%;
      }
  </style>