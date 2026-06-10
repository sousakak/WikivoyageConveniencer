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
}>()

const emit = defineEmits<{
    (e: 'select', value: any): void
    (e: 'open'): void
    (e: 'close'): void
}>()

const isOpen = ref(false)
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
@use "sass:map";
@use "../assets/styles/variables.scss" as var;

.app-dropdown {
    position: relative;
    display: inline-block;

    &-disabled {
        opacity: 0.5;
        pointer-events: none;
    }

    &-trigger {
        cursor: pointer;
    }

    &-menu {
        position: absolute;
        top: 100%;
        left: 0;

        min-width: 180px;

        list-style: none;

        padding: map.get(var.$scale, "space", "1") 0;

        background: map.get(var.$colors, "surface");

        border: 1px solid map.get(var.$colors, "border");

        border-radius: map.get(var.$scale, "radius", "md");

        box-shadow: 0 8px 24px rgb(0 0 0 / 12%);

        backdrop-filter: blur(8px);

        z-index: 1000;
    }

    &-item {
        cursor: pointer;
        user-select: none;
        padding: map.get(var.$scale, "space", "2") map.get(var.$scale, "space", "4");
        color: map.get(var.$colors, "text");
        transition:
            background 0.15s ease,
            color 0.15s ease;

        &:hover {
            background: map.get(var.$colors, "primary");

            color: map.get(var.$colors, "surface");
        }

        &-disabled {
            opacity: 0.5;
            cursor: default;

            &:hover {
                background: none;
                color: map.get(var.$colors, "text");
            }
        }
    }

    &-small {
        .app-dropdown-menu {
            min-width: 140px;
            font-size: 12px;
        }
    }

    &-medium {
        .app-dropdown-menu {
            min-width: 180px;
            font-size: 14px;
        }
    }

    &-large {
        .app-dropdown-menu {
            min-width: 220px;
            font-size: 16px;
        }
    }
}

.app-dropdown-transition-enter-active,
.app-dropdown-transition-leave-active {
    transition:
        opacity 0.15s ease,
        transform 0.15s ease;
}

.app-dropdown-transition-enter-from,
.app-dropdown-transition-leave-to {
    opacity: 0;
    transform: translateY(-4px);
}
</style>