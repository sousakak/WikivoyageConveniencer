<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/core'
    import { ref, onMounted } from 'vue'

    type UserData = {
        name: string
        registration: string | null
        edit_count: number
    }

    type UserResponse = {
        users: UserData[]
        next_from: string | null
    }

    const users = ref<UserData[]>([])
    const nextCursor = ref<string | null>(null)

    const loading = ref(false)
    const hasMore = ref(true)
    const error = ref<string | null>(null)

    const columns = [
        { label: '利用者名', id: 'name' },
        { label: '登録日時', id: 'registration' },
        { label: '編集数', id: 'edit_count' }
    ]

    const load = async (cursor: string | null = null) => {
        if (loading.value || !hasMore.value) return

        loading.value = true
        error.value = null

        try {
            const res = await invoke<UserResponse>('get_users', {
                aufrom: cursor
            })

            users.value.push(...res.users)
            nextCursor.value = res.next_from

            if (!res.next_from) {
                hasMore.value = false
            }

        } catch (e) {
            console.error('[load more failed]', e)

            error.value = String(e)
            hasMore.value = false
        } finally {
            loading.value = false
        }
    }

    onMounted(() => load())
</script>

<template>
    <div class="graph-page">
        <h1>利用者一覧</h1>

        <div v-if="error" style="color: red; margin-bottom: 10px;">
            {{ error }}
        </div>

        <app-table
            :columns="columns"
            :data="users"
        >
            <template #item-registration="{ item }">
                {{ item ? new Date(item).toLocaleString() : '-' }}
            </template>

            <template #header>
                <div>
                    生データ（Wikivoyage users）
                </div>
            </template>

            <template #footer>
                <div style="margin-top: 12px;">
                    件数: {{ users.length }}

                    <div style="margin-top: 12px;">
                        <AppButton
                            v-if="hasMore"
                            type="primary"
                            size="medium"
                            :disabled="loading"
                            @click="load(nextCursor)"
                        >
                            {{ loading ? '読み込み中...' : 'さらに読み込む' }}
                        </AppButton>
                    </div>
                </div>
            </template>
        </app-table>
    </div>
</template>