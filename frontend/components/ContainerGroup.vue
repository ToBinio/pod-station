<script setup lang="ts">
const { containers } = defineProps<{ containers: Container[] }>();
</script>

<template>
    <div class="grid grid-cols-2 gap-4 p-4">
        <Container
            v-for="container in containers"
            :key="container.name"
            :container="container"
        />
    </div>
</template>

<style scoped></style>
