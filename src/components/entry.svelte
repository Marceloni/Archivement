<script lang="ts">
  export let title = ""
</script>

<div class="entry">
  <h2 class="entry-title">{title}</h2>
  <div class="content-preview">
    <img src="src/assets/example_image.jpg">
    <img src="src/assets/example_image.jpg">
    <img src="src/assets/example_image.jpg">
    <img src="src/assets/example_image.jpg">
    <img src="src/assets/example_image.jpg">
  </div>
</div>

<style>

  .entry {
    background-color: #7e7e7e;
    padding: 0.5rem;
    margin: 0.5rem;
    border: 3px solid rgb(51, 51, 51);
    text-align: center;
    width: auto;
    box-sizing: border-box;
  }
  .entry-title {
    margin: 0;
    padding-bottom: 0.25rem;
    margin-bottom: 0.5rem;
    border-bottom: 3px solid rgb(51, 51, 51);
  }
  .content-preview {
    margin: 0.25rem;
    width: auto;
    display: grid;
    justify-content: center;
    align-items: center;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(2, 1fr);
    gap: 0.25rem; /* Optional: Add space between the grid items */
  }
  .content-preview img {
    width: 100%;
    height: auto;
    display: block;
  }
  .content-preview img:nth-child(1) {
    grid-column: 1 / 3; /* Span 2 columns */
    grid-row: 1 / 3;    /* Span 2 rows */
  }
  .content-preview img:nth-child(2) {
    grid-column: 3 / 4;
    grid-row: 1 / 2;
  }
  .content-preview img:nth-child(3) {
    grid-column: 4 / 4;
    grid-row: 1 / 2;
  }
  .content-preview img:nth-child(4) {
    grid-column: 3 / 4;
    grid-row: 2 / 2;
  }
  .content-preview img:nth-child(5) {
    grid-column: 4 / 4;
    grid-row: 2 / 2;
  }
</style>
