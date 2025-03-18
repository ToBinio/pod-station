<script setup lang="ts">
import type { Container } from "~/utils/containers";

const config = useRuntimeConfig();

const { data: containers } = await useFetch<Container[]>("/containers", {
    baseURL: config.public.baseURL as string,
});
</script>

<template>
    <h1 class="text-3xl font-bold text-center p-4">Pod Station</h1>
    <div v-if="containers !== null">
        <ContainerGroup :containers="containers" />
    </div>
</template>
