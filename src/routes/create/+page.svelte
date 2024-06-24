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
</script>

<div id="create-entry-page">
  <h1>Create a new Entry</h1>
  <p>You can still edit these later</p>

  <input id="title-input" type="text" placeholder="Enter a Title">

  <div id="goals-list">
    <GoalInput></GoalInput>
    <button on:click={addGoal} id="add-goal-button"/>
  </div>

  <textarea id="entry-description" placeholder="Description" maxlength=128/>

  <div id="footer">
    <button id="create-entry-button" on:click={createEntry}>Create Entry</button>
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

  #entry-description {
    margin-top: 0.5rem;
    font-family: inherit;
    resize: none;
    width: 20rem;
    height: 3rem;
  }

  #add-goal-button {
    cursor: pointer;
    background-color: white;
    border: 3px solid black;
    background-image: url("src/assets/icons/plus.svg");
    width: 80px;
    background-repeat: no-repeat;
    background-position: center;
    height: 35px;
  }

  #create-entry-page {
    margin-bottom: 5rem;
  }

  #goals-list {
    padding: 0.5rem;
    background-color: #494646;
    box-shadow: #000000 0 0 0.75rem;
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

  #title-input {
    font-size: x-large;
    margin-bottom: 2rem;
  }
  #create-entry-button{
    cursor: pointer;
    font-size: x-large;
  }
</style>
