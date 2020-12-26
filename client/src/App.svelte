<script>
	import Logs from './Logs.svelte';

	let text = "";
	let numbers = [];
	const save_log = () => invoke({ cmd: 'saveLog', msg: text });
	const load_logs = () => invoke({cmd: 'loadLogs'});
	const error_notif = () => invoke({cmd: 'errorNotif', msg: 'Oh no!'});
	window.set_numbers = n => numbers = n;
</script>

<main>
	<h1>Hello World!</h1>
	<div>
		<textarea bind:value={text} placeholder="write something"></textarea>
		<button on:click={save_log}>Log to database</button>
	</div>
	<div>
		<button on:click={load_logs}>load logs</button>
	</div>
	<div>
		<button on:click={error_notif}>oh no!</button>
	</div>
	<Logs />
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;

		display: grid;
		grid-template-columns: 1fr;
		grid-gap: 12px;
	}
	textarea {
		margin: 0;
	}
	main > * {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
	}
	button {
		margin: 0 12px 0;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>