<script lang="ts">
	import { onMount } from "svelte";
    import {DefaultService, OpenAPI, type Pet} from "../generated/index"
    let service = new DefaultService();
    let pet: Pet | null = null;

    onMount(async () => {
        OpenAPI.CREDENTIALS = 'include';
        OpenAPI.TOKEN = "hello world"
        pet = await DefaultService.postPets({
            name: "Hello",
            id: 1
        });
    })
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#if !pet}
    loading...
{:else}
    {JSON.stringify(pet, null, ' ')}
{/if}