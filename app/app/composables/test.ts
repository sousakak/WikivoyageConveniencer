import { invoke } from '@tauri-apps/api/core'

export const useWiki = () => {
    const test = async () => {
        const res = await invoke('greet', { name: 'Wiki' })
        console.log("Rust response:", res)
    }

    const getRecentChanges = async () => {
        try {
            const data = await invoke('recent_changes')
            console.log('SUCCESS:', data)
            return data
        } catch (e) {
            console.error('FAILED:', e)
        }
    }

    return {
        test,
        getRecentChanges
    }
}