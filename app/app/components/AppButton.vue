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
@use "sass:color";
@use "sass:map";
@use '../assets/styles/variables.scss' as var;

.app-button {
    background: map.get(var.$colors, "surface");
    border: 2px solid;
    font: inherit;
    line-height: 1;
    padding: 0.5em 1em;
    transition: 0.25s;
    cursor: pointer;

    @mixin typeSettings($mainColor, $reversed: false) {
        @if $reversed {
            color: $mainColor;

            &:hover {
                border-color: $mainColor;
                box-shadow: inset 0 0 0 2em $mainColor;
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
        @else {
            border-color: $mainColor;
            box-shadow: inset 0 0 0 2em $mainColor;
            color: map.get(var.$colors, "surface");

            &:hover {
                border-color: $mainColor;
                box-shadow: none;
                background: map.get(var.$colors, "surface");
                color: $mainColor;
            }
            @media (any-hover: none) {
                &:focus {
                    border-color: $mainColor;
                    box-shadow: none;
                    background: map.get(var.$colors, "surface");
                    color: $mainColor;
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
        &.app-button-rounded {
            border-radius: map.get(var.$scale, "radius", "sm");
        }
    }
    &-medium {
        padding: 0.5em 1em;
        &.app-button-rounded {
            border-radius: map.get(var.$scale, "radius", "md");
        }
    }
    &-large {
        padding: 1em 2em;
        &.app-button-rounded {
            border-radius: map.get(var.$scale, "radius", "md");
        }
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