import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface ResultData {
  status: number,
  data: ResponseData[]
}

export interface ResponseData {
  repo: string,
  name: string,
  version: string,
  path: string[]
}

export const useSearchStore = defineStore('pkgfile', () => {
  const apiUrl = "http://127.0.0.1:3000"
  const fetch_data = (fileName: string, callback: (data?: ResultData) => void) => {
    fetch(`${apiUrl}/api/search?f=${fileName}`)
      .then(data => data.json())
      .then((data: ResultData) => {
        callback(data)
      })
      .catch(err => {
        callback(undefined)
      })
  }
  return { fetch_data, apiUrl }
})
