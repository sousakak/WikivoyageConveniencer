<template>
    <header class="header">
        <nav class="menu">
            <AppDropdown
                :items="[
                    { label: '開く', value: 'open' },
                    { label: '保存', value: 'save' },
                    { label: '終了', value: 'exit' }
                ]"
                @select="console.log"
            >
                <AppButton
                    type="secondary"
                    size="medium"
                >
                    ファイル
                </AppButton>
            </AppDropdown>
        </nav>
    </header>
</template>

<style lang="scss">

</style>