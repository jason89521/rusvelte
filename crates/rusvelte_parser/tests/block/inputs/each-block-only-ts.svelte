<script lang="ts"></script>

{#each items as a: number[] ,i(item.id)}
	<li>{item.name} x {item.qty}</li>
{/each}