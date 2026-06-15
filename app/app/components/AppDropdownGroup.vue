<script setup lang="ts">
    import { ref } from 'vue'

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
        items = [],
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

    const openedIndex = ref<number | null>(null)

    const onSelect = (group: string, value: any) => {
        emit('select', { group, value })
    }

    const setOpen = (index: number, open: boolean) => {
        if (open) {
            openedIndex.value = index
            emit('open', items[index]!.title)
        } else {
            if (openedIndex.value === index) {
                openedIndex.value = null
            }
            emit('close', items[index]!.title)
        }
    }
</script>

<template>
    <nav class="app-menu">
        <AppDropdown
            v-for="(group, index) in items"
            :key="group.title"
            :items="group.items"
            :size="size"
            :open="openedIndex === index"
            @update:open="value => setOpen(index, value)"
            @select="value => onSelect(group.title, value)"
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
            &:first-child .app-button-small.app-button-rounded {
                border-top-left-radius: map.get(var.$scale, "radius", "sm");
                border-bottom-left-radius: map.get(var.$scale, "radius", "sm");
            }
            &:first-child .app-button-medium.app-button-rounded,
            &:first-child .app-button-large.app-button-rounded {
                border-top-left-radius: map.get(var.$scale, "radius", "md");
                border-bottom-left-radius: map.get(var.$scale, "radius", "md");
            }
            &:last-child .app-button-small.app-button-rounded {
                border-top-right-radius: map.get(var.$scale, "radius", "sm");
                border-bottom-right-radius: map.get(var.$scale, "radius", "sm");
            }
            &:last-child .app-button-medium.app-button-rounded,
            &:last-child .app-button-large.app-button-rounded {
                border-top-right-radius: map.get(var.$scale, "radius", "md");
                border-bottom-right-radius: map.get(var.$scale, "radius", "md");
            }
        }
    }
</style>