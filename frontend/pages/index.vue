<script setup lang="ts">
const containers = await useContainers();
</script>

<template>
    <h1 class="text-3xl font-bold text-center p-4 text-white">Pod Station</h1>
    <div>
        <ContainerGroup :containers="containers" />
    </div>
</template>

<style>
:root {
    background: oklch(0.216 0.006 56.043);
    --vis-axis-tick-label-color: white;
}
</style>
