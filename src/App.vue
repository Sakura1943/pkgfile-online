<script setup lang="ts">
import { Search } from "@element-plus/icons-vue"
import { ElMessage, ElNotification } from "element-plus"
import { ref, onMounted } from "vue"
import { useSearchStore } from "@/stores/search"
import { usePingStore } from "@/stores/ping"
import { debounce } from "@/utils/index"

/// Search store
const search = useSearchStore().fetch_data
const apiUrl = useSearchStore().apiUrl

// Variables
const title = "Pkgfile online"
const page_loading = ref(true)
const fileName = ref()
const tableData = ref()
const table_loading = ref(false)

let dark = true
let html_class_list = document.querySelector("html")?.classList

// On vue mounted
onMounted(() => {
    // Loading theme scheme
    if (sessionStorage.colorscheme == undefined) {
        sessionStorage.colorscheme = 'dark'
        dark = true
    }
    if (sessionStorage.colorscheme == 'light') {
        html_class_list?.remove("dark")
        html_class_list?.add("light")
        dark = false
    }

    // Confirm backend server is activated
    const ping = usePingStore().ping
    ping((pong) => {
        if (!pong) {
            ElNotification({
                title: 'Error',
                message: `Failed to connect backend ${usePingStore().apiUrl}`,
                type: 'error'
            })
            sessionStorage.connected = 'error'
        } else {
            if (sessionStorage.connected == undefined) {
                ElNotification({
                    title: 'Success',
                    message: `Connectted backend ${usePingStore().apiUrl}`,
                    type: 'success'
                })
            } else {
                if (sessionStorage.connected == 'error') {
                    ElNotification({
                        title: 'Success',
                        message: `Connectted backend ${usePingStore().apiUrl}`,
                        type: 'success'
                    })
                }
            }
            sessionStorage.connected = 'success'
        }
        page_loading.value = false
    })


})

// Bind to the button named `Dark/Light`
const changeThemeMode = () => {
    if (dark) {
        html_class_list?.remove("dark")
        html_class_list?.add("light")
        sessionStorage.colorscheme = 'light'
        dark = false
    } else {
        html_class_list?.add("dark")
        html_class_list?.remove("light")
        sessionStorage.colorscheme = 'dark'
        dark = true
    }
}

// Get package infomarion
const getInfo = () => {
    table_loading.value = true
    search(fileName.value, (data) => {
        if (data == undefined) {
            ElNotification({
                title: 'Error',
                message: `Failed to connect backend ${apiUrl}`,
                type: 'error'
            })
            table_loading.value = false
            return
        }
        if (data?.status == 200) {
            tableData.value = data.data
        } else {
            tableData.value = undefined
        }
        if (tableData.value != undefined) {
            table_loading.value = false
            ElMessage({
                message: 'Success',
                type: "success"
            })
            return
        }
        table_loading.value = false
        ElMessage({
            message: `Failed to get the pacman with file '${fileName.value}'`,
            type: "error"
        })
    })

}

// package url (only core, extra, community, community-testing)
const packageUrl = (repo: string, name: string) => {
    return `https://archlinux.org/packages/${repo}/x86_64/${name}`
}

const redirectPage = (repo: string, name: string) => {
    if (repo.includes("core") || repo.includes("community") || repo.includes("extra")) {
        ElMessage({
            message: `Redirecting...`,
            type: "info"
        })
        const url = packageUrl(repo, name)
        window.open(url, '_blank')
        ElMessage({
            message: `Redirected`,
            type: "success"
        })
    } else {
        ElMessage({
            message: `Unable to get the repositoty address`,
            type: "error"
        })
    }
}

</script>

<template v-loading="page_loading">
    <el-button class="change_theme" @click="debounce(changeThemeMode)()" round>Dark/Light</el-button>
    <div class="container" v-loading="page_loading">
        <h3 class="header">{{ title }}</h3>
        <div class="box">
            <el-input autofocus v-model="fileName" placeholder="File Name" @keyup.enter.native="debounce(getInfo)()"
                maxlength="255" show-word-limit class="input">
                <template #append>
                    <el-button :icon="Search" @click="debounce(getInfo)()" />
                </template>
            </el-input>
            <el-table v-loading="table_loading" :data="tableData" style="width: 100% ; border-radius:5px;" border
                class="table">
                <el-table-column prop="repo" label="Repo" />
                <el-table-column prop="name" label="Name">
                    <template v-slot="scope">
                        <el-popover placement="top" title="Package url" trigger="hover" :width="300"
                            :content="packageUrl(scope.row.repo, scope.row.name)"
                            v-if="scope.row.repo.includes('core') || scope.row.repo.includes('extra') || scope.row.repo.includes('community')">
                            <template #reference>
                                <p @click="redirectPage(scope.row.repo, scope.row.name)">{{ scope.row.name }}
                                </p>
                            </template>
                        </el-popover>
                        <div v-else>
                            <p @click="redirectPage(scope.row.repo, scope.row.name)">{{ scope.row.name }}</p>
                        </div>
                    </template>
                </el-table-column>
                <el-table-column prop="version" label="Version" />
                <el-table-column prop="path" label="Path" />
            </el-table>
        </div>
    </div>
</template>

<style scoped>
.container {
    width: 70%;
    height: 80%;
    text-align: center;
}

.header {
    margin-top: 50px;
    margin-bottom: 20px;
    font-size: 32px;
    font-weight: 500;
    font-family: 'JetBrains Mono';
}

.input {
    margin-top: 20px;
    margin-bottom: 50px;
    width: 40%;
    height: 45px;
    border-radius: 5px;
}

.box>* {
    font-size: 18px;
    font-family: 'JetBrains Mono';
}

.box {
    margin-bottom: 50px;
}

.change_theme {
    position: absolute;
    top: 0;
    right: 0;
    margin: 20px;
}

.table {
    box-shadow: 1.5px 1.5px 1px 1px rgba(0, 0, 0, 0.2);

}
</style>