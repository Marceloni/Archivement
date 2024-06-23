<script lang="ts">
    import type { EntrySettings } from "../../../entrySettings";

    /** @type {import('./$types').PageData} */
	export let data;
    let settings: EntrySettings = JSON.parse(data.settings)
</script>
<div id="content-edit-page">
    <h1 id="entry-title">{settings.title}</h1>
    <p id="entry-description">{settings.description}</p>
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
<style>
    #content-edit-page {
        text-align: center;
    }
    #entry-description {
        padding-bottom: 1rem;
        border-bottom: 3px rgb(36, 36, 36) solid;
        margin-bottom: 1rem;
    }
    p {margin: 0px}
</style>