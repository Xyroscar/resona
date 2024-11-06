<script lang="ts">
  import { currentResponse } from '../../stores';
  let activeTab = 'body';
  
  function formatJson(json: string): string {
    try {
      return JSON.stringify(JSON.parse(json), null, 2);
    } catch {
      return json;
    }
  }

  function getStatusColor(status: number): string {
    if (status >= 200 && status < 300) return 'text-success';
    if (status >= 400) return 'text-error';
    return 'text-warning';
  }
</script>

{#if $currentResponse}
  <div class="card bg-base-100 shadow-lg border border-base-300">
    <div class="card-body p-4">
      <!-- Status and Time -->
      <div class="flex justify-between items-center">
        <div class="flex items-center space-x-2">
          <span class={getStatusColor($currentResponse.status)}>
            {$currentResponse.status}
          </span>
          <span class="text-base-content/70">
            {$currentResponse.statusText}
          </span>
        </div>
        <span class="text-base-content/70">
          {$currentResponse.time}ms
        </span>
      </div>

      <!-- Tabs -->
      <div class="tabs tabs-boxed bg-base-200 mt-4">
        <button 
          class="tab" 
          class:tab-active={activeTab === 'body'}
          on:click={() => activeTab = 'body'}
        >
          Response Body
        </button>
        <button 
          class="tab" 
          class:tab-active={activeTab === 'headers'}
          on:click={() => activeTab = 'headers'}
        >
          Response Headers
        </button>
      </div>

      <!-- Content -->
      {#if activeTab === 'body'}
        <div class="bg-base-200 rounded-lg mt-4">
          <pre class="overflow-x-auto p-4 text-sm">
            <code>{formatJson($currentResponse.body)}</code>
          </pre>
        </div>
      {:else}
        <div class="overflow-x-auto mt-4">
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Header</th>
                <th>Value</th>
              </tr>
            </thead>
            <tbody>
              {#each Object.entries($currentResponse.headers) as [key, value]}
                <tr>
                  <td class="font-mono text-sm">{key}</td>
                  <td class="font-mono text-sm">{value}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>
{/if} 