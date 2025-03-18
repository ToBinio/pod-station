<script setup lang="ts">
const containers = await useContainers();
</script>

<template>
    <h1 class="text-3xl font-bold text-center p-4">Pod Station</h1>
    <div>
        <ContainerGroup :containers="containers" />
    </div>
</template>
