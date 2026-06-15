<script setup lang="ts">
import { computed } from 'vue'

type Column = {
    label: string
    id: string
}

const props = defineProps<{
    columns: Column[]
    data: Record<string, any>[]
}>()

const safeData = computed(() => Array.isArray(props.data) ? props.data : [])
</script>

<template>
    <div class="app-table">
        <slot name="header" />

        <table>
            <thead>
                <tr>
                    <th v-for="col in columns" :key="col.id">
                        {{ col.label }}
                    </th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="(item, i) in safeData" :key="i">
                    <td v-for="col in columns" :key="col.id">
                        <slot
                            :name="`item-${col.id}`"
                            :item="item[col.id]"
                            :row="item"
                        >
                            {{ item[col.id] }}
                        </slot>
                    </td>
                </tr>
            </tbody>
        </table>

        <slot name="footer" />
    </div>
</template>

<style scoped>
.app-table {
    width: 100%;
}

table {
    width: 100%;
    border-collapse: collapse;
}

th, td {
    padding: 8px;
    border-bottom: 1px solid #ddd;
}
</style>