<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'

type UserData = {
    name: string
    registration: string | null
    edit_count: number
}

const users = ref<UserData[]>([])

onMounted(async () => {
    try {
        users.value = await invoke<UserData[]>(
            'get_users'
        )

        console.log(users.value)
    }
    catch (e) {
        console.error(e)
    }
})
</script>

<template>
    <div>
        <h1>利用者一覧</h1>

        <table>
            <thead>
                <tr>
                    <th>利用者名</th>
                    <th>登録日時</th>
                    <th>編集数</th>
                </tr>
            </thead>

            <tbody>
                <tr
                    v-for="user in users"
                    :key="user.name"
                >
                    <td>{{ user.name }}</td>
                    <td>{{ user.registration }}</td>
                    <td>{{ user.edit_count }}</td>
                </tr>
            </tbody>
        </table>
    </div>
</template>
