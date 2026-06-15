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
        <app-dropdown-group
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
@use "~/assets/styles/components/app-menu";
</style>