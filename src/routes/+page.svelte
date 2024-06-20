<script lang="ts">
  import Entry from "../components/entry.svelte"
</script>

<div class="container">
  <h1>Welcome to Archivement</h1>

  <p>Archive your Achievements</p>

  <div id="entry-list">
    <Entry title="Example Title"/>
  </div>

  <div id="footer">
    <img alt="Add Entry" id="add-archive-button" src="src/assets/icons/plus.svg">
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

  h1, p {
    text-align: center;
  }

  #entry-list {
    display: flex;
    justify-content: center;
    align-self: center;
    width: 100%;
  }

  #footer {
    position: fixed;
    left: 0;
    bottom: 0;
    width: 100%;
    background-color: rgb(49, 45, 45);
    align-items: center;
    justify-content: center;
    display: flex;
    padding-bottom: 10px;
    padding-top: 10px;
  }

  #add-archive-button {
    height: 50px;
    width: 50px;
    border-radius: 50%;
    border-color: black;
    border-width: 4px;
    border-style: solid;
    box-sizing: border-box;
    filter: invert(1);
  }
</style>
