<script lang="ts">
  import { goto } from "$app/navigation";

  const { params } = $props<{ params: { id: string } }>();

  function goBack() {
    goto("/workspaces");
  }
</script>

<div class="min-h-[calc(100vh-4rem)] p-6 space-y-4">
  <h1 class="text-2xl font-semibold">Workspace ID</h1>
  <p class="text-muted-foreground">
    Current workspace id: <strong>{params.id}</strong>
  </p>

  <button
    class="mt-4 inline-flex items-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90"
    onclick={goBack}
  >
    Back to workspaces
  </button>
</div>
