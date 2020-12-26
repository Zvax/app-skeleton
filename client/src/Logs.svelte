<script>
  let logs = [];
  window.set_logs = l => logs = l;
</script>

<section>
  {#each logs as log}
    <p>{log.msg}</p>
  {/each}
</section>

