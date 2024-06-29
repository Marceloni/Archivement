<script lang="ts">
  import {invoke} from '@tauri-apps/api/core';
  import GoalInput from "../../components/goalInput.svelte";
  import { goto } from '$app/navigation';
  let goals: GoalInput[] = []
  function addGoal() {
    goals.forEach((goal) => {goal.$set({closable: false})})
    const goalInstance = new GoalInput({target: document.getElementById("goals-list") as HTMLElement, anchor: document.getElementById("add-goal-button") as HTMLElement, props: {closable: true, onRemove: () => {goals.splice(goals.indexOf(goalInstance), 1); goals.at(-1)?.$set({closable: true})}}})
    goals.push(goalInstance)
  }
  function createEntry() {
    let title = (document.getElementById("title-input") as HTMLInputElement).value
    let goalValues = [...document.getElementsByClassName("goal-input")].map((goal) => (goal as HTMLInputElement).value)
    let description = (document.getElementById("entry-description") as HTMLTextAreaElement).value

    invoke("create_entry", {title, goals: goalValues, description})
    goto("/")
  }
  function goBack() {goto("/")}
</script>

<div id="create-entry-page">
  <div class="icon" id="back-button" style="mask-image: url('../src/assets/icons/arrow-right-circle.svg'); -webkit-mask-image: url('../src/assets/icons/arrow-right-circle.svg');" on:click={goBack}/>

  <h1>Create a new Entry</h1>
  <p>You can still edit these later</p>

  <input id="title-input" type="text" placeholder="Enter a Title">

  <div id="goals-list">
    <GoalInput></GoalInput>
    <div on:click={addGoal} id="add-goal-button">
      <div class="icon" style="mask-image: url('src/assets/icons/plus.svg'); -webkit-mask-image: url('src/assets/icons/plus.svg'); height: 100%"/>
    </div>
  </div>

  <textarea id="entry-description" placeholder="Description" maxlength=128/>

  <div id="footer">
    <button id="create-entry-button" on:click={createEntry}>Create Entry</button>
  </div>
</div>

<style>
  #back-button {
    cursor: pointer;
    width: 3rem;
    height: 3rem;
    position: absolute;
    top: 0.5rem;
    left: 0.5rem;
  }
  #back-button:hover {
    background-color: var(--accent);
  }
  #entry-description {
    margin-top: 0.5rem;
    font-family: inherit;
    resize: none;
    width: 20rem;
    height: 3rem;
  }

  #add-goal-button {
    cursor: pointer;
    border: 3px solid var(--text);
    border-radius: 50%;
    width: 2rem;
    height: 2rem;
  }
  #add-goal-button:hover {
    filter: invert(1);
  }

  #create-entry-page {
    margin-bottom: 5rem;
  }

  #goals-list {
    padding: 0.5rem;
    background-color: var(--primary);
    box-shadow: 0rem 0rem 1rem var(--secondary);
    margin: auto;
    align-items: center;
    width: 22rem;
    display: flex;
    flex-direction: column;
    margin-bottom: 2rem;
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

  #title-input {
    font-size: x-large;
    margin-bottom: 2rem;
  }
  #create-entry-button{
    cursor: pointer;
    font-size: x-large;
  }
</style>
