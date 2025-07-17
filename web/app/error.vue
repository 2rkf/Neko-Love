<template>
  <div
    class="min-h-screen flex flex-col bg-gradient-to-b from-pink-100 via-pink-50 to-rose-100 font-[Nunito] noselect transition-all"
  >
    <Navbar />
    <main class="flex-grow flex items-center justify-center text-center px-4">
      <div class="space-y-6 animate-fade-in-up max-w-lg">
        <div class="flex flex-col items-center gap-2">
          <img
            src="/assets/logo.png"
            alt="Nekoi Error"
            class="h-32 w-32 animate-wiggle-slow drop-shadow-lg"
            draggable="false"
          />
          <h1 class="text-5xl font-extrabold text-pink-600 drop-shadow-pink">
            Oops! <span class="text-orange-400">Something Went Wrong</span>
          </h1>
        </div>
        <p
          class="text-pink-700 text-lg font-medium backdrop-blur-sm bg-white/40 p-4 rounded-xl shadow-md"
        >
          {{
            error.statusCode === 404
              ? "Page not found! It looks like this page wandered off like a curious neko."
              : error.statusCode === 500
                ? "Server error! Our nekos are scratching their heads trying to fix this."
                : "An unexpected error occurred. Don't worry, our nekos are on the case!"
          }}
        </p>
        <div class="mt-4">
          <NuxtLink
            to="/"
            class="inline-block px-6 py-3 bg-pink-500 text-white font-semibold rounded-lg shadow-md hover:bg-pink-600 transition-colors duration-300"
          >
            Back to Home
          </NuxtLink>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
const error = useError();

useHead({
  title: `Error ${error.statusCode || "Unknown"} — Nekoi`,
  meta: [
    { property: "og:title", content: `Error ${error.statusCode || "Unknown"} — Nekoi` },
    { property: "og:site_name", content: "2rkf" },
    {
      property: "og:description",
      content:
        error.statusCode === 404
          ? "Page not found on Nekoi. Return to explore our anime-style character images."
          : "An error occurred on Nekoi. We're working to get things back on track!",
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

<style scoped>
.drop-shadow-pink {
  filter: drop-shadow(0 4px 6px rgba(244, 114, 182, 0.3));
}
</style>
