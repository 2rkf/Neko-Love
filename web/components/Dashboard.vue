<template>
  <div class="min-h-screen bg-zinc-800 text-zinc-100 noselect flex flex-col">
    <Navbar />
    <main class="flex-grow px-6 py-10 space-y-8">
      <div class="text-center">
        <h1 class="text-4xl font-extrabold text-white">Dashboard</h1>
        <p class="text-zinc-400 mt-2">Welcome back, here's your overview.</p>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        <UCard
          class="bg-zinc-900 border border-zinc-700"
          :ui="{ body: 'p-5 space-y-2 text-left' }"
        >
          <h2 class="text-xl font-bold text-orange-200">Images Uploaded</h2>
          <p class="text-3xl font-extrabold text-white">128</p>
        </UCard>

        <UCard
          class="bg-zinc-900 border border-zinc-700"
          :ui="{ body: 'p-5 space-y-2 text-left' }"
        >
          <h2 class="text-xl font-bold text-orange-200">Users</h2>
          <p class="text-3xl font-extrabold text-white">87</p>
        </UCard>

        <UCard
          class="bg-zinc-900 border border-zinc-700"
          :ui="{ body: 'p-5 space-y-2 text-left' }"
        >
          <h2 class="text-xl font-bold text-orange-200">Favourites</h2>
          <p class="text-3xl font-extrabold text-white">342</p>
        </UCard>
      </div>

      <div class="text-zinc-400 text-center text-sm pt-10">
        More widgets coming soon...
      </div>
    </main>
  </div>
</template>

<script setup>
useHead({
  title: "Dashboard — Neko-Love",
  meta: [
    { name: "description", content: "View your dashboard on Neko-Love." },
    { name: "theme-color", content: "#ffbb88" },
  ],
});
</script>
