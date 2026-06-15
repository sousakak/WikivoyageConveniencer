<script setup lang="ts">
import { computed } from 'vue'

type ButtonType = 'primary' | 'secondary' | 'danger'
type ButtonSize = 'small' | 'medium' | 'large'

const {
    type,
    size,
    disabled,
    ariaLabel,
    rounded = true
} = defineProps<{
    type?: ButtonType
    size?: ButtonSize
    disabled?: boolean
    ariaLabel?: string
    rounded?: boolean
}>()

const emit = defineEmits<{
    (e: 'click', event: MouseEvent): void
}>()

const onClick = (e: MouseEvent) => {
    if (disabled) return
    emit('click', e)
}

const classes = computed(() => [
    'app-button',
    `app-button-${type ?? 'primary'}`,
    `app-button-${size ?? 'medium'}`,
    rounded ? 'app-button-rounded' : '',
    disabled && 'app-button-disabled'
])
</script>

<template>
    <button
        :class="classes"
        :disabled="disabled"
        :aria-disabled="disabled"
        :aria-label="ariaLabel"
        @click="onClick"
    >
        <slot />
    </button>
</template>

<style lang="scss">
@use "~/assets/styles/components/app-button";
</style>