<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import type { EntrySettings } from "../../entrySettings";
    import {open} from "@tauri-apps/plugin-dialog"
    import { event } from "@tauri-apps/api";
    import { appDataDir, join } from "@tauri-apps/api/path";
    import { goto } from "$app/navigation";
    import { page } from '$app/stores';
    let editing = false

    let settings: EntrySettings = JSON.parse($page.url.searchParams.get("settings") as string)
    
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

    async function changeContentButton(event: MouseEvent) {
        let target = event.currentTarget as HTMLElement
        let contentPieceDiv = target.parentElement?.parentElement as HTMLDivElement;
        await invoke("change_content_piece", {uuid: settings.uuid, index: parseInt(contentPieceDiv.dataset.index as string), text: contentPieceDiv.dataset.type=="text" ? (contentPieceDiv.querySelector("textarea.content-piece-element") as HTMLInputElement).value : undefined})
    }

    event.listen("reload_entry", (event) => {
        reloadContentPieces(event.payload as string)
    })

    window.onclick = function(event) {
        if (!(event.target as HTMLElement).matches("#add-content-button, #add-content-button > *")) {
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

    function removeContentButton(event: MouseEvent) {
        let target = event.currentTarget as HTMLElement
        let contentPieceDiv = target.parentElement?.parentElement as HTMLDivElement;
        invoke("remove_content_piece", {uuid: settings.uuid, index: parseInt(contentPieceDiv.dataset.index as string)})
    }

    function changeGoalState(event: Event) {
        let target = event.currentTarget as HTMLInputElement
        let goalDiv = target.parentElement as HTMLDivElement
        let index = parseInt(goalDiv.dataset.index as string)
        invoke("change_goal_state", {uuid: settings.uuid, index, completed: target.checked})
    }

    function toggleEditing() {editing = !editing}
    function goBack() {goto("/")}

    function removeGoal(event: MouseEvent) {
        let target = event.currentTarget as HTMLElement
        let goalDiv = target.parentElement as HTMLDivElement
        let index = parseInt(goalDiv.dataset.index as string)
        invoke("remove_goal", {uuid: settings.uuid, index})
    }
    function changeGoalTitle(event: Event) {
        let target = event.currentTarget as HTMLInputElement
        let goalDiv = target.parentElement as HTMLDivElement
        let index = parseInt(goalDiv.dataset.index as string)
        invoke("change_goal_title", {uuid: settings.uuid, index, title: target.value})
    }
    function changeEntryTitle(event: Event) {
        let target = event.currentTarget as HTMLInputElement
        invoke("change_entry_title", {uuid: settings.uuid, title: target.value})
    }
    function changeEntryDescription(event: Event) {
        let target = event.currentTarget as HTMLTextAreaElement
        invoke("change_entry_description", {uuid: settings.uuid, description: target.value})
    }
    function deleteEntry() {
        invoke("delete_entry", {uuid: settings.uuid})
    }

    event.listen("entry_deleted", () => {goto("/")})
</script>
    <div id="content-edit-page">
        <div class="icon" id="back-button" style="mask-image: url('../src/assets/icons/arrow-right-circle.svg'); -webkit-mask-image: url('../src/assets/icons/arrow-right-circle.svg');" on:click={goBack}/>
        <div class="icon" id="edit-toggle" style="mask-image: url('../src/assets/icons/edit-pen-4.svg'); -webkit-mask-image: url('../src/assets/icons/edit-pen-4.svg');" on:click={toggleEditing}/>

        {#if editing}
            <div class="icon" id="delete-button" style="mask-image: url('../src/assets/icons/delete-bin.svg'); -webkit-mask-image: url('../src/assets/icons/delete-bin.svg');" on:click={deleteEntry}/>

            <input maxlength=32 placeholder="Title" on:input={changeEntryTitle} id="entry-title-input" type="text" value={settings.title}>
            <div class="entry-description-div">
                <textarea maxlength=128 placeholder="Description" on:input={changeEntryDescription} id="entry-description-input">{settings.description}</textarea>
            </div>
        {:else}
            <h1 id="entry-title">{settings.title}</h1>
            <div class="entry-description-div">
                <p id="entry-description">{settings.description}</p>
            </div>
        {/if}

        <div id="goals-list">
            {#each settings.goals as goal, index}
                <div class="goal-div" data-index={index}>
                    <input class="goal-checkbox" on:change={changeGoalState} checked={goal.completed} type="checkbox">
                    {#if editing}
                        <input class="goal-input" on:input={changeGoalTitle} value={goal.title}>
                        <div on:click={removeGoal} class="icon remove-goal-button" style="mask-image: url('../src/assets/icons/close.svg'); -webkit-mask-image: url('../src/assets/icons/close.svg');"/>
                    {:else}
                        {#if goal.completed}
                            <h3 class="goal-title" style="text-decoration: line-through; text-decoration-thickness: 3px;">{goal.title}</h3>
                        {:else}
                            <h3 class="goal-title">{goal.title}</h3>
                        {/if}
                    {/if}
                </div>
            {/each}
        </div>

        <div id="content-pieces">
            {#each settings.content as contentPiece, index}
                <div class="content-main-div">
                    <div class="content-piece-div" data-index={index} data-type={contentPiece.type}>
                        {#if contentPiece.type === "image"}
                            <img src={contentPiece.path} class="content-piece-element">
                        {/if}
                        {#if contentPiece.type === "text"}
                            <textarea readonly={editing?false:true} class="content-piece-element">{contentPiece.text}</textarea>
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
                        
                        {#if editing}
                            <div class="edit-buttons-div">
                                {#if contentPiece.type != "text"}
                                    <div class="icon change-content-button" style="mask-image: url('../src/assets/icons/file.svg'); -webkit-mask-image: url('../src/assets/icons/file.svg');" on:click={changeContentButton}/>
                                {/if}
                                {#if contentPiece.type === "text"}
                                    <div class="icon reset-button" style="mask-image: url('../src/assets/icons/reload.svg'); -webkit-mask-image: url('../src/assets/icons/reload.svg');" on:click={revertText}/>
                                    <div class="icon reset-button" style="mask-image: url('../src/assets/icons/save.svg'); -webkit-mask-image: url('../src/assets/icons/save.svg');" on:click={changeContentButton}/>
                                {/if}
                                <div class="icon remove-content-button" style="mask-image: url('../src/assets/icons/close.svg'); -webkit-mask-image: url('../src/assets/icons/close.svg');" on:click={removeContentButton}/>
                            </div>
                        {/if}
                    </div>
                    <p class="creation-date">Created: { new Date(contentPiece.creation_date*1000).toLocaleString("en-gb", {day: 'numeric', month: 'numeric', year: '2-digit', hour: '2-digit', minute: '2-digit'})} </p>
                </div>
            {/each}
        </div>
        {#if editing}
            <div id="footer">
                <div id="add-content-button" on:click={openAddContentDropdown}>
                    <div class="icon" style="mask-image: url('../src/assets/icons/plus.svg'); -webkit-mask-image: url('../src/assets/icons/plus.svg'); height:100%"/>
                </div>
                <div id="add-content-dropdown" class={addContentDropdownShown?"show":""} on:click={addContentPiece}>
                    <p data-type="text">Text</p>
                    <p data-type="image">Image</p>
                    <p data-type="video">Video</p>
                    <p data-type="audio">Audio</p>
                </div>
            </div>
        {/if}
    </div>
<style>
    #entry-title-input {
        font-size: x-large;
        text-align: center;
        margin-block-start: 0.67em;
        margin-block-end: 0.67em;
    }
    #entry-description-input {
        font-family: inherit;
        resize: none;
        width: 15rem;
        height: 5rem;
    }
    .goal-title {
        margin: 0;
    }
    .goal-input {
        padding: 0.5rem;
        width: fit-content;
    }
    .remove-goal-button {
        cursor: pointer;
        height: 100%;
        width: 2.5rem;
        height: 2.5rem;
    }
    .remove-goal-button:hover {
        background-color: var(--accent);
    }
    #back-button {
        cursor: pointer;
        width: 3rem;
        height: 3rem;
        position: absolute;
        top: 0.5rem;
        left: 0.5rem;
    }
    #goals-list {
        margin-top: 2rem;
        margin-bottom: 0.5rem;
    }
    .goal-div {
        display: flex;
        flex-direction: row;
        margin-bottom: 1rem;
        align-items: center;
        position: relative;
    }
    .goal-checkbox {
        margin-right: 1rem;
        width: 1.5rem;
        height: 1.5rem;
    }
    #add-content-dropdown {
        display: none;
        background-color: var(--primary);
        box-shadow: 0rem 0rem 1rem var(--secondary);
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
        background-color: var(--accent);
    }
    .show {display:block !important;}
    #content-edit-page {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-bottom: 5rem;
    }
    #entry-description {
        width: 25rem;
        overflow: hidden;
        line-break: anywhere;
        overflow: hidden;
        text-overflow: ellipsis;
        line-break: anywhere;
        -webkit-line-clamp: 5;
        display: -webkit-box;
        -webkit-box-orient: vertical;
    }

    p {margin: 0px}
    #content-pieces {
        border-top: 3px solid var(--accent);
        margin-top: 1rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        max-width: 35rem;
    }
    .content-piece-element {
        width: calc(100% - 3rem);
    }
    .content-piece-div textarea {
        font-family: inherit;
        resize: none;
        height: 8rem;
    }
    .reset-button, .change-content-button, .remove-content-button{
        width: 2rem;
        height: 2rem;
        cursor: pointer;
    }

    #edit-toggle {
        cursor: pointer;
        width: 3rem;
        height: 3rem;
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
    }
    #delete-button {
        cursor: pointer;
        width: 3rem;
        height: 3rem;
        position: absolute;
        top: 0.5rem;
        right: 4rem;
    }

    .edit-buttons-div {
        margin-left: 1rem;
        display: flex;
        flex-direction: column;
        align-self: flex-start;
    }
    .reset-button:hover, .change-content-button:hover, #add-content-button:hover, .remove-content-button:hover {
        background-color: var(--accent);
    }
    #edit-toggle:hover, #back-button:hover, #delete-button:hover {
        background-color: var(--accent);
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

    #add-content-button {
        cursor: pointer;
        height: 3rem;
        width: 3rem;
        border-radius: 50%;
        border-color: var(--text);
        border-width: 4px;
        border-style: solid;
        box-sizing: border-box;
    }

    .content-main-div {
        margin-top: 2rem;
        border: solid 3px var(--primary);
        background-color: var(--secondary);
        padding: 1rem;
        width: 35rem;
        border-radius: 1rem;
        box-sizing: border-box;
    }
    .content-piece-div {
        justify-content: center;
        display: flex;
        flex-direction: row;
        align-items: center;
        width: 100%;
        margin-bottom: 1rem;
    }

    #content-pieces :nth-child(2){
        margin-top: 0.5rem;
    }
  </style>