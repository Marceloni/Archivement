<script lang="ts">
  import GoalInput from "../../components/goalInput.svelte";

  function addGoal() {
    new GoalInput({target: document.getElementById("goals-list") as HTMLElement, anchor: document.getElementById("add-goal-button") as HTMLElement})
  }
</script>

<div id="create-entry-page">
  <h1>Create a new Entry</h1>

  <p>You can still edit these later</p>
  <div id="goals-list">
    <GoalInput closable={false}></GoalInput>
    <button on:click={addGoal} id="add-goal-button"/>
  </div>

  <div id="footer">
    <button id="create-entry-button">Create Entry</button>
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

  #add-goal-button {
    background-color: white;
    border: 3px solid black;
    background-image: url("src/assets/icons/plus.svg");
    width: 40px;
    height: 40px;
  }

  #create-entry-page {
    margin-bottom: 4rem;
  }

  #goals-list {
    margin: auto;
    align-items: center;
    width: 20rem;
    display: flex;
    flex-direction: column;
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
    font-size: x-large;
  }
</style>
