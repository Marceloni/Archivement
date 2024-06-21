<script lang="ts">
  import { goto } from "$app/navigation";
  import Entry from "../components/entry.svelte"
  function addEntry() {
    goto("/create")
  }
</script>

<div id="home-page">
  <h1>Welcome to Archivement</h1>

  <p>Archive your Achievements</p>

  <div id="entry-list">
    <Entry title="Example Title"/>
    <Entry title="Example Title"/>
  </div>

  <div id="footer">
    <img alt="Create Entry" on:click={addEntry} id="create-entry-button" src="src/assets/icons/plus.svg">
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
  }

  #home-page {
    margin-bottom: 4rem;
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
    background-color: rgb(49, 45, 45);
    align-items: center;
    justify-content: center;
    display: flex;
    padding-bottom: 0.5rem;
    padding-top: 0.5rem;
  }

  #create-entry-button {
    cursor: pointer;
    height: 50px;
    width: 50px;
    border-radius: 50%;
    border-color: black;
    border-width: 4px;
    border-style: solid;
    box-sizing: border-box;
    filter: invert(1);
    user-select: none;
    -webkit-user-select: none;
    -webkit-user-drag: none;
  }
</style>
