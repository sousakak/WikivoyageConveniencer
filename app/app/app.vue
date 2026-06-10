<script setup lang="ts">
import { useWiki } from '~/composables/test'

const { test, getRecentChanges } = useWiki()
</script>

<template>
    <div class="app">
        <AppHeader />

        <main>
            <NuxtPage />
        </main>
    </div>
</template>