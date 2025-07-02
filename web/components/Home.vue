<template>
  <div class="min-h-screen flex flex-col bg-zinc-800 text-zinc-100 noselect">
    <header
      class="flex items-center justify-between px-6 py-4 border-b bg-zinc-900 border-zinc-700"
    >
      <NuxtLink to="/" class="flex items-center gap-2">
        <img
          src="/assets/logo.png"
          alt="Logo"
          class="h-10 w-10"
          draggable="false"
        />
        <h1 class="text-2xl font-extrabold tracking-wide">
          Neko-<span class="text-orange-200">Love</span>
        </h1>
      </NuxtLink>
      <div>
        <div class="flex items-center gap-4">
          <UButton
            variant="ghost"
            size="lg"
            label="Discord Server"
            to="https://discord.gg/Az8RWJJ6fT"
            class="font-medium text-base cursor-pointer text-orange-200"
            target="_blank"
          />
          <UButton
            variant="ghost"
            size="lg"
            label="Login"
            class="font-medium text-base cursor-pointer text-orange-200"
          />
        </div>
      </div>
    </header>

    <main class="flex-grow flex items-center justify-center text-center px-4">
      <div class="space-y-4">
        <div class="flex flex-col items-center gap-2">
          <img
            src="/assets/logo.png"
            alt="Logo"
            class="h-32 w-32"
            draggable="false"
          />
          <h1 class="text-6xl font-extrabold text-white">
            Neko-<span class="text-orange-200">Love</span>
          </h1>
        </div>
        <p class="max-w-xl text-zinc-400 text-lg">
          Neko-Love is a collection of high-quality images featuring characters
          with anime-style appearances.
        </p>
      </div>
    </main>
  </div>
</template>

<script setup>
useHead({
  title: "Neko-Love",
  meta: [
    { property: "og:title", content: "Neko-Love" },
    { property: "og:site_name", content: "2rkf" },
    {
      property: "og:description",
      content:
        "A collection of high-quality images featuring characters with anime-style appearances.",
    },
    { property: "og:image", content: "/neko-love.png" },
    { property: "og:image:type", content: "image/png" },
    { property: "og:image:width", content: "1200" },
    { property: "og:image:height", content: "630" },
    { name: "theme-color", content: "#cc536e" },
    { "http-equiv": "x-ua-compatible", content: "IE=edge" },
    { name: "viewport", content: "width=device-width, initial-scale=1.0" },
  ],
});
</script>
