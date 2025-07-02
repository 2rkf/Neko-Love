<template>
  <div class="min-h-screen flex flex-col bg-zinc-800 text-zinc-100 noselect">
    <Navbar />
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
    { name: "theme-color", content: "#ffbb88" },
    { "http-equiv": "x-ua-compatible", content: "IE=edge" },
    { name: "viewport", content: "width=device-width, initial-scale=1.0" },
  ],
});
</script>
