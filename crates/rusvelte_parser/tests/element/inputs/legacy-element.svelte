<svelte:component this={A}>
  <svelte:fragment slot="a"></svelte:fragment>
</svelte:component>
{#if a > 0}
  <svelte:self a={a - 1} />
{:else}
  a
{/if}<slot></slot>
