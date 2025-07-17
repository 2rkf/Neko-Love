<template>
  <div class="min-h-screen flex flex-col bg-gradient-to-b from-pink-100 via-pink-50 to-rose-100 font-[Nunito] noselect transition-all">
    <Navbar />
    <main class="flex-grow flex items-center justify-center text-center px-4">
      <div class="space-y-6 animate-fade-in-up">
        <div class="flex flex-col items-center gap-2">
          <img
            src="/assets/logo.png"
            alt="Logo"
            class="h-32 w-32 animate-wiggle-slow drop-shadow-lg"
            draggable="false"
          />
          <h1 class="text-6xl font-extrabold text-pink-600 drop-shadow-pink">
            Ne<span class="text-orange-400">koi</span>
          </h1>
        </div>
        <p class="max-w-xl text-pink-700 text-lg font-medium backdrop-blur-sm bg-white/40 p-4 rounded-xl shadow-md">
          Nekoi is a collection of high-quality images featuring characters
          with anime-style appearances.
        </p>
      </div>
    </main>
  </div>
</template>

<script setup>
useHead({
  title: "Nekoi",
  meta: [
    { property: "og:title", content: "Nekoi" },
    { property: "og:site_name", content: "2rkf" },
    {
      property: "og:description",
      content: "A collection of adorable, high-quality anime-style character images.",
    },
    { property: "og:image", content: "/nekoi.png" },
    { property: "og:image:type", content: "image/png" },
    { property: "og:image:width", content: "1200" },
    { property: "og:image:height", content: "630" },
    { name: "theme-color", content: "#ffd8b1" },
    { "http-equiv": "x-ua-compatible", content: "IE=edge" },
    { name: "viewport", content: "width=device-width, initial-scale=1.0" },
  ],
});
</script>
