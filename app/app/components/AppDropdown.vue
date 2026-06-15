<script setup lang="ts">
    import {
        computed,
        onMounted,
        onBeforeUnmount,
        ref
    } from 'vue'

    type DropdownItem = {
        label: string
        value: any
        disabled?: boolean
    }

    type DropdownSize = 'small' | 'medium' | 'large'

    const props = defineProps<{
        items: DropdownItem[]
        size?: DropdownSize
        disabled?: boolean
        open?: boolean
    }>()

    const emit = defineEmits<{
        (e: 'select', value: any): void
        (e: 'open'): void
        (e: 'close'): void
        (e: 'update:open', value: boolean): void
    }>()

    const isOpen = computed({
        get: () => props.open ?? false,
        set: (value: boolean) => emit('update:open', value)
    })
    const root = ref<HTMLElement>()

    const classes = computed(() => [
        'app-dropdown',
        `app-dropdown-${props.size ?? 'medium'}`,
        props.disabled && 'app-dropdown-disabled'
    ])

    const open = () => {
        if (props.disabled || isOpen.value) return

        isOpen.value = true
        emit('open')
    }

    const close = () => {
        if (!isOpen.value) return

        isOpen.value = false
        emit('close')
    }

    const toggle = () => {
        isOpen.value
            ? close()
            : open()
    }

    const selectItem = (item: DropdownItem) => {
        if (item.disabled) return

        emit('select', item.value)
        close()
    }

    const onDocumentClick = (event: MouseEvent) => {
        if (!root.value) return

        const target = event.target as Node

        if (!root.value.contains(target)) {
            close()
        }
    }

    onMounted(() => {
        document.addEventListener('click', onDocumentClick)
    })

    onBeforeUnmount(() => {
        document.removeEventListener('click', onDocumentClick)
    })
</script>

<template>
    <div ref="root" :class="classes">
        <div class="app-dropdown-trigger" @click.stop="toggle">
            <slot />
        </div>

        <Transition name="app-dropdown-transition">
            <ul v-if="isOpen" class="app-dropdown-menu">
                <li v-for="item in items" :key="item.label" class="app-dropdown-item" :class="{
                    'app-dropdown-item-disabled': item.disabled
                }" @click.stop="selectItem(item)">
                    {{ item.label }}
                </li>
            </ul>
        </Transition>
    </div>
</template>

<style lang="scss">
@use "~/assets/styles/components/app-dropdown";
</style>
