<script lang="ts">
  export let closable: boolean = false;
  let input: HTMLDivElement;
  export let onRemove: () => void = ()=>{};

  function removeGoal() {
    input.remove();
  }
</script>

<div bind:this={input} class="goal-input-div">
  <input class="goal-input" type="text" placeholder="Enter a Goal">
  <div on:click={removeGoal} class="icon remove-goal-button" style="mask-image: url('../src/assets/icons/close.svg'); -webkit-mask-image: url('../src/assets/icons/close.svg');"/>
</div>

<style>
  .goal-input-div {
    display: inline-flex;
    align-items: center;
    justify-items: center;
    height: fit-content;
    max-height: 2rem;
    overflow: hidden;
    margin-bottom: 0.5rem;
    padding-left: 3rem;
  }

  .remove-goal-button {
    cursor: pointer;
    height: 100%;
    width: 3rem;
    height: 3rem;
  }
  .remove-goal-button:hover {
    filter: invert(1);
  }

  .goal-input {
    box-sizing: border-box;
    font-size: larger;
    height: 2rem;
  }
</style>
