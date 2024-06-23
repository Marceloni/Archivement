<script lang="ts">
    import type { EntrySettings } from "../../../entrySettings";

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
            <img src="../src/assets/icons/reload.svg" class="reset-button" on:click={revertContentPiece}>
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
        height: auto;
        width: calc(100% - 2rem);
    }
    .reset-button {
        width: 2rem;
        height: 2rem;
        cursor: pointer;
        align-self: flex-start;
    }
    .reset-button:hover {
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