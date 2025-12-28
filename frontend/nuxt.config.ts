// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    modules: [
        "@nuxt/eslint",
        "@nuxt/ui",
        "@nuxt/hints",
        "@nuxt/image",
        "@nuxt/scripts",
        "@nuxt/test-utils"
    ],

    devtools: {
        enabled: true
    },

    css: ["~/assets/css/main.css"],

    // appConfig: {
    //   apiUrl: process.env.API_URL || 'http://localhost:3001'
    // },

    runtimeConfig: {
        apiUrl: process.env.SSR_API_URL || "http://backend:3001",
        public: {
            apiUrl: process.env.CLIENT_API_URL || "http://localhost:3001"
        }
    },

    routeRules: {
        "/": { prerender: true }
    },

    devServer: {
        https: {
            key: "./certs/localhost.key",
            cert: "./certs/localhost.crt"
        }
    },

    compatibilityDate: "2025-01-15",

    eslint: {
        config: {
            stylistic: {
                commaDangle: "never",
                braceStyle: "1tbs"
            }
        }
    }
});
