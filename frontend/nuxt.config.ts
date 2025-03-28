// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  modules: ['@nuxtjs/tailwindcss', '@vueuse/nuxt', '@nuxt/icon'],
  runtimeConfig: {
    public: {
      baseURL: process.env.BASE_URL || 'http://localhost:8080/',
    },
  },
})