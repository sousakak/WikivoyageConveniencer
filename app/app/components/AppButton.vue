<script setup lang="ts">
import { computed } from 'vue'

type ButtonType = 'primary' | 'secondary' | 'danger'
type ButtonSize = 'small' | 'medium' | 'large'

const props = defineProps<{
    type?: ButtonType
    size?: ButtonSize
    disabled?: boolean
    ariaLabel?: string
}>()

const emit = defineEmits<{
    (e: 'click', event: MouseEvent): void
}>()

const onClick = (e: MouseEvent) => {
    if (props.disabled) return
    emit('click', e)
}

const classes = computed(() => [
    'app-button',
    `app-button-${props.type ?? 'primary'}`,
    `app-button-${props.size ?? 'medium'}`,
    props.disabled && 'app-button-disabled'
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
@use "sass:color";
@use "sass:map";
@use '../assets/styles/variables.scss' as var;

.app-button {
    background: none;
    border: 2px solid;
    font: inherit;
    line-height: 1;
    padding: 0.5em 1em;
    transition: 0.25s;

    @mixin typeSettings($mainColor, $reversed: false) {
        @if $reversed {
            border-color: color.adjust($mainColor, $hue: -5deg);
            box-shadow: inset 0 0 0 2em color.adjust($mainColor, $hue: -5deg);
            color: map.get(var.$colors, "surface");

            &:hover {
                border-color: $mainColor;
                box-shadow: none;
                background: none;
                color: $mainColor;
            }
            @media (any-hover: none) {
                &:focus {
                    border-color: $mainColor;
                    box-shadow: none;
                    background: none;
                    color: $mainColor;
                }
            }
        }
        @else {
            color: $mainColor;

            &:hover {
                border-color: color.adjust($mainColor, $hue: -5deg);
                box-shadow: inset 0 0 0 2em color.adjust($mainColor, $hue: -5deg);
                color: map.get(var.$colors, "surface");
            }
            @media (any-hover: none) {
                &:focus {
                    border-color: color.adjust($mainColor, $hue: -5deg);
                    box-shadow: inset 0 0 0 2em color.adjust($mainColor, $hue: -5deg);
                    color: map.get(var.$colors, "surface");
                }
            }
        }
    }

    &-primary {
        @include typeSettings(map.get(var.$colors, "primary"));
    }
    &-secondary {
        @include typeSettings(map.get(var.$colors, "primary"), true);
    }
    &-danger {
        @include typeSettings(map.get(var.$colors, "danger"));
    }

    &-disabled {
        opacity: 0.5;
        pointer-events: none;
    }

    &-small {
        padding: 0.1em 0.2em;
        border-radius: map.get(var.$scale, "radius", "sm");
    }
    &-medium {
        padding: 0.5em 1em;
        border-radius: map.get(var.$scale, "radius", "md");
    }
    &-large {
        padding: 1em 2em;
        border-radius: map.get(var.$scale, "radius", "md");
    }

    &:has(+ &) {
        border-top-right-radius: 0;
        border-bottom-right-radius: 0;
    }
    & + & {
        border-top-left-radius: 0;
        border-bottom-left-radius: 0;
    }
}
</style>