<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { activeRequest, variables, currentResponse } from '../../stores';
    import type { Header, QueryParam } from '../../types';
    
    let url = '';
    let method = 'GET';
    let headers: Header[] = [];
    let queryParams: QueryParam[] = [];
    let body = '';
    let loading = false;
    let activeTab = 'headers';
  
    const methods = ['GET', 'POST', 'PUT', 'DELETE', 'PATCH'];
  
    // Subscribe to activeRequest changes
    $: if ($activeRequest) {
      url = $activeRequest.url;
      method = $activeRequest.method;
      headers = $activeRequest.headers;
      queryParams = $activeRequest.queryParams || [];
      body = $activeRequest.body || '';
    }
  
    function addHeader() {
      headers = [...headers, {
        id: crypto.randomUUID(),
        key: '',
        value: '',
        enabled: true
      }];
    }
  
    function addQueryParam() {
      queryParams = [...queryParams, {
        id: crypto.randomUUID(),
        key: '',
        value: '',
        enabled: true
      }];
    }
  
    function removeHeader(id: string) {
      headers = headers.filter(h => h.id !== id);
    }
  
    function removeQueryParam(id: string) {
      queryParams = queryParams.filter(p => p.id !== id);
    }
  
    function interpolateVariables(value: string): string {
      return value.replace(/\{\{(.+?)\}\}/g, (_, key) => {
        const variable = $variables.find(v => v.name === key.trim());
        return variable ? variable.value : `{{${key}}}`;
      });
    }
  
    function buildUrl(baseUrl: string, params: QueryParam[]): string {
      try {
        if (!baseUrl) return '';
        
        const url = new URL(baseUrl);
        params
          .filter(p => p.enabled && p.key)
          .forEach(p => {
            try {
              url.searchParams.append(
                interpolateVariables(p.key),
                interpolateVariables(p.value || '') // Handle undefined value
              );
            } catch (error) {
              console.error('Error appending parameter:', error);
            }
          });
        return url.toString();
      } catch (error) {
        console.error('Error building URL:', error);
        return baseUrl; // Return original URL if there's an error
      }
    }
  
    async function sendRequest() {
      loading = true;
      try {
        const interpolatedHeaders = headers.map(h => ({
          ...h,
          key: interpolateVariables(h.key),
          value: interpolateVariables(h.value)
        }));
  
        const request = {
          url: buildUrl(interpolateVariables(url), queryParams),
          method,
          headers: interpolatedHeaders,
          body: body ? interpolateVariables(body) : undefined
        };
  
        const response = await invoke<{
          status: number;
          statusText: string;
          headers: Record<string, string>;
          body: string;
          time: number;
        }>('send_api_request', { request });
        
        currentResponse.set(response);
      } catch (error) {
        console.error('Error:', error);
      } finally {
        loading = false;
      }
    }

    // Add these functions to handle tab clicks explicitly
    function setActiveTab(tab: 'headers' | 'body' | 'query') {
      activeTab = tab;
    }

    // Separate function to handle query param updates
    function updateQueryParam(param: QueryParam, field: 'key' | 'value' | 'enabled', value: string | boolean) {
      queryParams = queryParams.map(p => 
        p.id === param.id 
          ? { ...p, [field]: value }
          : p
      );
    }
  </script>
  
  <div class="card bg-base-100 shadow-lg border border-base-300">
    <div class="card-body p-4">
      <div class="flex space-x-2">
        <div class="join flex-1">
          <select 
            class="select select-bordered join-item w-28 font-medium"
            bind:value={method}
          >
            {#each methods as m}
              <option value={m}>{m}</option>
            {/each}
          </select>
  
          <input 
            type="text"
            placeholder="Enter request URL"
            class="input input-bordered join-item flex-1 min-w-[400px]"
            bind:value={url}
          />
  
          <button 
            class="btn join-item btn-primary"
            on:click={sendRequest}
            disabled={loading || !url}
          >
            {#if loading}
              <span class="loading loading-spinner loading-sm"></span>
            {:else}
              Send
            {/if}
          </button>
        </div>
      </div>
  
      <div class="tabs tabs-boxed bg-base-200 mt-4">
        <a 
          href="#headers"
          class="tab" 
          class:tab-active={activeTab === 'headers'}
          on:click|preventDefault={() => setActiveTab('headers')}
        >
          Headers
        </a>
        <a 
          href="#body"
          class="tab"
          class:tab-active={activeTab === 'body'}
          on:click|preventDefault={() => setActiveTab('body')}
        >
          Body
        </a>
        <a 
          href="#query"
          class="tab"
          class:tab-active={activeTab === 'query'}
          on:click|preventDefault={() => setActiveTab('query')}
        >
          Query
        </a>
      </div>
  
      <!-- Headers Section -->
      <div class="mt-4" class:hidden={activeTab !== 'headers'}>
        <div class="flex justify-between items-center mb-2">
          <h3 class="text-sm font-medium opacity-70">Request Headers</h3>
          <button 
            type="button"
            class="btn btn-sm btn-ghost" 
            on:click={() => addHeader()}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
            </svg>
            Add Header
          </button>
        </div>
  
        <div class="space-y-2">
          {#each headers as header (header.id)}
            <div class="join w-full">
              <input
                type="checkbox"
                class="checkbox join-item ml-2"
                bind:checked={header.enabled}
              />
              <input
                type="text"
                placeholder="Header name"
                class="input input-bordered input-sm join-item w-1/3"
                bind:value={header.key}
              />
              <input
                type="text"
                placeholder="Value"
                class="input input-bordered input-sm join-item flex-1"
                bind:value={header.value}
              />
              <button 
                type="button"
                class="btn btn-sm join-item btn-ghost text-error"
                on:click={() => removeHeader(header.id)}
              >
                ×
              </button>
            </div>
          {/each}
        </div>
      </div>
  
      <!-- Body Section -->
      <div class="mt-4" class:hidden={activeTab !== 'body'}>
        <h3 class="text-sm font-medium opacity-70 mb-2">Request Body</h3>
        <textarea
          class="textarea textarea-bordered w-full h-48 font-mono text-sm"
          placeholder="Enter JSON body"
          bind:value={body}
        ></textarea>
      </div>
  
      <!-- Query Section -->
      <div class="mt-4" class:hidden={activeTab !== 'query'}>
        <div class="flex justify-between items-center mb-2">
          <h3 class="text-sm font-medium opacity-70">Query Parameters</h3>
          <button 
            type="button"
            class="btn btn-sm btn-ghost" 
            on:click={addQueryParam}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
            </svg>
            Add Parameter
          </button>
        </div>
  
        <div class="space-y-2">
          {#each queryParams as param (param.id)}
            <div class="join w-full">
              <input
                type="checkbox"
                class="checkbox join-item ml-2"
                checked={param.enabled}
                on:change={(e) => updateQueryParam(param, 'enabled', e.currentTarget.checked)}
              />
              <input
                type="text"
                placeholder="Parameter name"
                class="input input-bordered input-sm join-item w-1/3"
                value={param.key}
                on:input={(e) => updateQueryParam(param, 'key', e.currentTarget.value)}
              />
              <input
                type="text"
                placeholder="Value"
                class="input input-bordered input-sm join-item flex-1"
                value={param.value}
                on:input={(e) => updateQueryParam(param, 'value', e.currentTarget.value)}
              />
              <button 
                type="button"
                class="btn btn-sm join-item btn-ghost text-error"
                on:click={() => removeQueryParam(param.id)}
              >
                ×
              </button>
            </div>
          {/each}
        </div>
  
        {#if queryParams.length > 0 && url}
          <div class="mt-4">
            <h4 class="text-sm font-medium opacity-70 mb-2">Preview URL</h4>
            <div class="bg-base-200 p-3 rounded-lg">
              <code class="text-xs break-all">
                {buildUrl(url, queryParams)}
              </code>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
    