<template>
    <header class="header">
        <app-menu
            :items="[
                {
                    title: '全般',
                    items: [
                        { label: 'ツール情報', value: 'about' },
                        { label: '更新', value: 'update' }
                    ]
                },
                {
                    title: '統計',
                    items: [
                        { label: 'グラフ化', value: 'graph' },
                        { label: '地理的階層', value: 'geoHier' },
                        { label: '投稿分析', value: 'anlysContrib' }
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
        />
    </header>
</template>