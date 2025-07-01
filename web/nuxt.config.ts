// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: true },
  modules: ['@nuxt/ui', '@nuxt/eslint'],
  nitro: {
    devProxy: {
      "/api": {
        target: "http://localhost:3030",
        changeOrigin: true,
        prependPath: false,
      }
    },
  },
});
