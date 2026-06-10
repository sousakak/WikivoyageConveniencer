<script setup lang="ts">
import { useWiki } from '../composables/test'

const { test, getRecentChanges } = useWiki()
</script>

<template>
    <AppButton
        type="primary"
        size="medium"
        @click="test"
    >
        Rust呼び出しテスト
    </AppButton>
    <AppButton
        type="primary"
        size="medium"
        @click="getRecentChanges"
    >
        最近の編集を取得
    </AppButton>
</template>