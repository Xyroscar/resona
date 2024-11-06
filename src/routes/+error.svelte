<script>
  import { page } from '$app/stores';
</script>

<div class="min-h-screen flex items-center justify-center">
  <div class="text-center">
    <h1 class="text-9xl font-bold text-base-content/20">{$page.status}</h1>
    <p class="text-xl mt-4">{$page.error?.message || 'Page not found'}</p>
    <a href="/" class="btn btn-primary mt-8">Go Home</a>
  </div>
</div> 