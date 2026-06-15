<script setup lang="ts">
    const router = useRouter()

    const onMenuSelect = (
        payload: {
            group: string
            value: string
        }
    ) => {
        router.push(payload.value)
    }
</script>

<template>
    <header class="header">
        <app-menu
            :items="[
                {
                    title: '全般',
                    items: [
                        { label: 'トップ', value: '/' },
                        { label: 'ツール情報', value: 'about' },
                        { label: '更新', value: 'update' }
                    ]
                },
                {
                    title: '統計',
                    items: [
                        { label: 'グラフ化', value: 'graph' },
                        { label: '地理的階層', value: 'geoHier' },
                        { label: 'VCard', value: 'vCard/' }
                    ]
                },
                {
                    title: '編集',
                    items: [
                        { label: '翻訳', value: 'translation' },
                        { label: 'VCardとListingの置換', value: 'convertListing' }
                    ]
                }
            ]"
            @select="onMenuSelect"
        />
    </header>
</template>