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

    const {
        items,
        size,
        rounded = true
    } = defineProps<{
        items: MenuGroup[]
        size?: DropdownSize
        rounded?: boolean
    }>()

    const emit = defineEmits<{
        (e: 'select', payload: { group: string; value: any }): void
        (e: 'open', group: string): void
        (e: 'close', group: string): void
    }>()

    const onSelect = (group: string, value: any) => {
        emit('select', { group, value })
    }

    const onOpen = (group: string) => {
        emit('open', group)
    }

    const onClose = (group: string) => {
        emit('close', group)
    }
</script>

<template>
    <nav class="app-menu">
        <AppDropdown
            v-for="group in items"
            :key="group.title"
            :items="group.items"
            :size="size"
            @select="value => onSelect(group.title, value)" @open="onOpen(group.title)" @close="onClose(group.title)"
        >
            <AppButton
                :rounded="rounded"
                type="primary"
                :size="size"
            >
                {{ group.title }}
            </AppButton>
        </AppDropdown>
    </nav>
</template>

<style lang="scss">
    @use "sass:map";
    @use "../assets/styles/variables.scss" as var;

    .app-menu {
        display: flex;
        align-items: stretch;

        .app-button {
            border-radius: 0;
        }

        &>.app-dropdown {
            &:first-child .app-button-small {
                border-top-left-radius: map.get(var.$scale, "radius", "sm");
                border-bottom-left-radius: map.get(var.$scale, "radius", "sm");
            }
            &:first-child .app-button-medium,
            &:first-child .app-button-large {
                border-top-left-radius: map.get(var.$scale, "radius", "md");
                border-bottom-left-radius: map.get(var.$scale, "radius", "md");
            }
            &:last-child .app-button-small {
                border-top-right-radius: map.get(var.$scale, "radius", "sm");
                border-bottom-right-radius: map.get(var.$scale, "radius", "sm");
            }
            &:last-child .app-button-medium,
            &:last-child .app-button-large {
                border-top-right-radius: map.get(var.$scale, "radius", "md");
                border-bottom-right-radius: map.get(var.$scale, "radius", "md");
            }
        }
    }
</style>