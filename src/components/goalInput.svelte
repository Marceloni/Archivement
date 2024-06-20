<script lang="ts">
  export let closable: Boolean = true;
  let input: HTMLDivElement;

  function removeGoal() {
    input.remove();
  }
</script>

<div bind:this={input} class={closable?"goal-input-div padded-goal-input-div":"goal-input-div"}>
  <input class="goal-input" type="text" placeholder="Enter a Goal">
  {#if closable}
    <img on:click={removeGoal} class="remove-goal-button" src="src/assets/icons/close.svg" alt="Remove Goal">
  {/if}
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
  }

  .remove-goal-button {
    display: inline-block;
    height: 100%;
    width: 3rem;
    height: 3rem;
  }

  .goal-input {
    box-sizing: border-box;
    font-size: larger;
    height: 2rem;
  }

  .padded-goal-input-div {
    padding-left: 3rem;
  }
</style>
