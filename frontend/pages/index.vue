<script setup lang="ts">
const { data: containers } = await useApiFetch<string[]>('/containers')
</script>

<template>
    <h1 class="text-xl ">Pod Station</h1>
    <div>
        <h2>Containers</h2>
        <div v-for="container in containers">
            {{ container }}
        </div>
    </div>
</template>
