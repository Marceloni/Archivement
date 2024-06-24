<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import type { EntrySettings } from "../../../entrySettings";
    import {open} from "@tauri-apps/plugin-dialog"
    import { event } from "@tauri-apps/api";
    import { appDataDir, join } from "@tauri-apps/api/path";
    /** @type {import('./$types').PageData} */
    export let data;
    let settings: EntrySettings = JSON.parse(data.settings)
    function revertText(e: MouseEvent) {
        let target = e.currentTarget as HTMLElement
        let parentElement = target.parentElement?.parentElement as HTMLDivElement
        let contentPieceElement = parentElement.querySelector("textarea.content-piece-element") as HTMLInputElement
        let index = parseInt(parentElement.dataset.index as string)
        contentPieceElement.value = settings.content[index].text as string
    }

    async function reloadContentPieces(uuid: String) {
        let new_settings: EntrySettings = await invoke("get_settings", {uuid})
        new_settings.content = await Promise.all(new_settings.content.map(async (contentPiece) => {
            if (contentPiece.type === "image" || contentPiece.type === "video" || contentPiece.type === "audio") {
                contentPiece.path = await convertFileSrc(await join(await appDataDir(), "entries", new_settings.uuid, "content", contentPiece.path as string))
            }
            return contentPiece
        }))
        settings = new_settings
    }

    async function changeContentButton(e: MouseEvent) {
        let target = e.currentTarget as HTMLElement
        let contentPieceDiv = target.parentElement?.parentElement as HTMLDivElement;
        await invoke("change_content_piece", {uuid: settings.uuid, index: parseInt(contentPieceDiv.dataset.index as string), text: contentPieceDiv.dataset.type=="text" ? (contentPieceDiv.querySelector("textarea.content-piece-element") as HTMLInputElement).value : undefined})
    }

    event.listen("reload_entry", (event) => {
        reloadContentPieces(event.payload as string)
    })

    window.onclick = function(event) {
        if (!(event.target as HTMLElement).matches("#add-content-button")) {
            addContentDropdownShown = false
        }
    } 

    let addContentDropdownShown = false
    function openAddContentDropdown() {
        addContentDropdownShown = !addContentDropdownShown
    }

    function addContentPiece(event: MouseEvent) {
        let target = event.target as HTMLElement
        invoke("add_content_piece", {uuid: settings.uuid, type: target.dataset.type})
    }
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
                        {#key contentPiece.path}
                            <video controls class="content-piece-element">
                                <source src={contentPiece.path} type="video/mp4">
                            </video>
                        {/key}
                    {/if}
                    {#if contentPiece.type === "audio"}
                        {#key contentPiece.path}
                            <audio controls class="content-piece-element">
                                <source src={contentPiece.path} type="audio/mpeg">
                            </audio>
                        {/key}
                    {/if}
                    
                    <div class="edit-buttons-div">
                        {#if contentPiece.type != "text"}
                            <img src="../src/assets/icons/file.svg" class="change-content-button" on:click={changeContentButton}>
                        {/if}
                        {#if contentPiece.type === "text"}
                            <img src="../src/assets/icons/reload.svg" class="reset-button" on:click={revertText}>
                            <img src="../src/assets/icons/save.svg" class="change-content-button" on:click={changeContentButton}>
                        {/if}
                    </div>
              </div>
          {/each}
      </div>
      <div id="footer">
        <img id="add-content-button" src="../src/assets/icons/plus.svg" on:click={openAddContentDropdown}>
        <div id="add-content-dropdown" class={addContentDropdownShown?"show":""} on:click={addContentPiece}>
            <p data-type="text">Text</p>
            <p data-type="image">Image</p>
            <p data-type="video">Video</p>
            <p data-type="audio">Audio</p>
          </div>
      </div>
  </div>
  <style>
    #add-content-dropdown {
        display: none;
        background-color: rgb(110, 110, 110);
        margin-bottom: 3.5rem;
        position: absolute;
        width: 5rem;
        z-index: 1;
        align-self: end;
    }

    #add-content-dropdown p {
        cursor: pointer;
        color: inherit;
        padding: 0.5rem 0.5rem;
        text-decoration: none;
        display: block;
    }
    #add-content-dropdown p:hover {
        background-color: rgb(66, 66, 66);
    }
    .show {display:block !important;}
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
        margin-bottom: 5rem;
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
    .reset-button, .change-content-button {
        width: 2rem;
        height: 2rem;
        cursor: pointer;
        user-select: none;
        -webkit-user-select: none;
        -webkit-user-drag: none;
    }
    .edit-buttons-div {
        display: flex;
        flex-direction: column;
        align-self: flex-start;
    }
    .reset-button:hover, .change-content-button:hover, #add-content-button:hover {
        filter: invert(1);
    }


    #footer {
        position: fixed;
        left: 0;
        bottom: 0;
        width: 100%;
        height: 3rem;
        background-color: rgb(49, 45, 45);
        align-items: center;
        justify-content: center;
        display: flex;
        padding-bottom: 0.5rem;
        padding-top: 0.5rem;
    }

    #add-content-button {
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