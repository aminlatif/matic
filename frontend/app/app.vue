<script setup lang="ts">
useHead({
    meta: [
        { name: "viewport", content: "width=device-width, initial-scale=1" }
    ],
    link: [
        { rel: "icon", href: "/favicon.ico" }
    ],
    htmlAttrs: {
        lang: "en"
    }
});

const title = "Matic";
const description = "A simple and lightweight organizer.";

useSeoMeta({
    title,
    description,
    ogTitle: title,
    ogDescription: description,
    ogImage: "https://ui.nuxt.com/assets/templates/nuxt/starter-light.png",
    twitterImage: "https://ui.nuxt.com/assets/templates/nuxt/starter-light.png",
    twitterCard: "summary_large_image"
});

const healthStatus = await useApi()("/health");
</script>

<template>
    <UApp>
        <UHeader>
            <template #left>
                <NuxtLink to="/">
                    <AppLogo class="w-auto h-6 shrink-0" />
                </NuxtLink>

                <TemplateMenu />
            </template>

            <template #right>
                <UColorModeButton />

                <UButton to="https://github.com/nuxt-ui-templates/starter" target="_blank" icon="i-simple-icons-github"
                    aria-label="GitHub" color="neutral" variant="ghost" />

                <div v-if="healthStatus.status === 'healthy'">
                    healthy
                </div>
                <div v-else>
                    unhealthy
                </div>
            </template>
        </UHeader>

        <UMain>


            <NuxtPage />
        </UMain>

        <USeparator icon="i-simple-icons-nuxtdotjs" />

        <UFooter>
            <template #left>
                <p class="text-sm text-muted">
                    Built with Nuxt UI • © {{ new Date().getFullYear() }}
                </p>
            </template>

            <template #right>
                <UButton to="https://github.com/nuxt-ui-templates/starter" target="_blank" icon="i-simple-icons-github"
                    aria-label="GitHub" color="neutral" variant="ghost" />
            </template>
        </UFooter>
    </UApp>
</template>
