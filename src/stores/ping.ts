import { defineStore } from 'pinia'


export const usePingStore = defineStore('ping', () => {
    const apiUrl = "http://127.0.0.1:3000"
    const ping = (callback: (pong: boolean) => void) => {
        fetch(`${apiUrl}/ping`)
            .then(_ => { callback(true) })
            .catch(_ => { callback(false) })
    }
    return { ping, apiUrl }
})