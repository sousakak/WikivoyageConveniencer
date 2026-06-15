<script setup lang="ts">
    type DropdownItem = {
        label: string
        value: any
        disabled?: boolean
    }

    type MenuGroup = {
        title: string
        items: DropdownItem[]
    }

    type DropdownSize = 'small' | 'medium' | 'large'

    const props = defineProps<{
        items: MenuGroup[]
        size?: DropdownSize
    }>()

    const emit = defineEmits<{
        (e: 'select', payload: { group: string; value: any }): void
        (e: 'open', group: string): void
        (e: 'close', group: string): void
    }>()

    // そのまま上に流す（薄いラッパー）
    const onSelect = (payload: { group: string; value: any }) => {
        emit('select', payload)
    }

    const onOpen = (group: string) => {
        emit('open', group)
    }

    const onClose = (group: string) => {
        emit('close', group)
    }
</script>

<template>
    <div class="app-menu-bar">
        <AppDropdownGroup
            :items="items"
            :size="size"
            :rounded="false"
            @select="onSelect"
            @open="onOpen"
            @close="onClose"
        />
    </div>
</template>

<style lang="scss">
    @use "sass:map";
    @use '../assets/styles/variables.scss' as var;

    $height: 45px;

    .app-menu-bar {
        width: 100%;
        background: map.get(var.$colors, "primary");
        display: flex;
        align-items: stretch;
        height: $height;
    }
</style>