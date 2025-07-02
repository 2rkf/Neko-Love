<template>
  <div class="min-h-screen flex flex-col bg-zinc-800 text-zinc-100 noselect">
    <Navbar />
    <main class="flex-grow flex items-start justify-center px-4 pt-40 pb-16">
      <UCard
        class="w-full max-w-md shadow-lg border border-zinc-700 bg-zinc-900"
        :ui="{ body: 'p-6 space-y-6 text-center' }"
      >
        <div class="space-y-4">
          <h2 class="text-4xl font-extrabold text-white">Login</h2>
          <p class="text-zinc-400">
            Sign in to Neko-Love using your Discord account.
          </p>

          <UButton
            icon="i-simple-icons-discord"
            size="lg"
            block
            color="primary"
            class="font-semibold cursor-pointer"
            label="Login with Discord"
            @click="loginWithDiscord"
          />
        </div>
      </UCard>
    </main>
  </div>
</template>

<script setup>
useHead({
  title: "Login — Neko-Love",
  meta: [
    { name: "description", content: "Login to Neko-Love." },
    { name: "theme-color", content: "#ffbb88" },
  ],
});
</script>
