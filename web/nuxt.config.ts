export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  css: ["~/assets/style.css"],
  devtools: { enabled: true },
  modules: ['@nuxt/ui', '@nuxt/eslint'],
  nitro: {
    routeRules: {
      "/api/**": {
        proxy: "http://localhost:3030/api/**",
      },
    },
  },
});
