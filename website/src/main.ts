import { mount } from 'svelte';
import './app.css';
import App from './App.svelte';

import initWasm, { Rusvelte } from '@rusvelte/rusvelte_wasm';

await initWasm();
const rusvelte = new Rusvelte();
rusvelte.parse(`
<script>let a = $state()</script>
{a}
`);
console.log(rusvelte.ast);

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
