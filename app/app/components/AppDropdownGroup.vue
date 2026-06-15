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

    const props = withDefaults(
        defineProps<{
            items: MenuGroup[]
            size?: DropdownSize
            rounded?: boolean
        }>(),
        {
            items: () => [],
            rounded: true
        }
    )

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
            emit('open', props.items[index]!.title)
        } else {
            if (openedIndex.value === index) {
                openedIndex.value = null
            }
            emit('close', props.items[index]!.title)
        }
    }
</script>

<template>
    <nav class="app-menu">
        <app-dropdown
            v-for="(group, index) in items"
            :key="group.title"
            :items="group.items"
            :size="props.size"
            :open="openedIndex === index"
            @update:open="value => setOpen(index, value)"
            @select="value => onSelect(group.title, value)"
        >
            <app-button
                :rounded="props.rounded"
                type="primary"
                :size="props.size"
            >
                {{ group.title }}
            </app-button>
        </app-dropdown>
    </nav>
</template>

<style lang="scss">
@use "~/assets/styles/components/app-dropdown-group";
</style>