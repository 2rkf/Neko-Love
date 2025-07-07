export default defineNuxtConfig({
    compatibilityDate: '2025-05-15',
    css: ["~/assets/style.css"],
    devtools: { enabled: true },
    modules: ['@nuxt/ui', '@nuxt/eslint', "@pinia/nuxt"],
    appConfig: {
        API_URL: process.env.API_URL,
        API_KEY: process.env.API_KEY,
    },
    runtimeConfig: {
        API_URL: process.env.API_URL,
        API_KEY: process.env.API_KEY,
    }
});
