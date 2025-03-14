<script setup lang="ts">
const {container} = defineProps<{ container: Container }>()

function getUptimeText(): string{
  const diff = new Date().getTime() / 1000 - container.started_at;

  const hours = Math.floor(diff / (60 * 60));
  const minutes = Math.floor((diff % (60 * 60)) / 60);
  const seconds = Math.floor((diff % 60));

  return `${hours}h ${minutes}m ${seconds}s`;
}

const uptime = ref(getUptimeText());

useIntervalFn(() => {
    uptime.value = getUptimeText();
}, 1000);

</script>

<template>
<div class="bg-gray-200 p-2 rounded-md">
    <div class="flex flex-row justify-around">
        <div>name: {{container.name}}</div>
        <div data-allow-mismatch>uptime: {{uptime}}</div>
    </div>
</div>
</template>

<style scoped>

</style>
