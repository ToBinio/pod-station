<script setup lang="ts">
import { VisXYContainer, VisLine, VisAxis, VisBulletLegend } from "@unovis/vue";
const { container } = defineProps<{ container: Container }>();

function getUptimeText(): string {
    const diff = new Date().getTime() / 1000 - container.started_at;

    const hours = Math.floor(diff / (60 * 60));
    const minutes = Math.floor((diff % (60 * 60)) / 60);
    const seconds = Math.floor(diff % 60);

    return `${hours}h ${minutes}m ${seconds}s`;
}

const uptime = ref(getUptimeText());

useIntervalFn(() => {
    uptime.value = getUptimeText();
}, 1000);

interface DataRecord {
    time_stamp: number;
    mem_percentage: number;
    cpu_percentage: number;
}

const data = ref<DataRecord[]>([]);

watch(
    () => container,
    (new_data) => {
        data.value.push({
            time_stamp: Date.now(),
            mem_percentage: new_data.memory_percent,
            cpu_percentage: new_data.cpu_percent,
        });

        if (data.value.length > 60) {
            data.value.shift();
        }
    },
);
</script>

<template>
    <div class="bg-stone-800 text-white p-2 rounded-md">
        <div class="flex flex-row justify-around">
            <div>name: {{ container.name }}</div>
            <div data-allow-mismatch>uptime: {{ uptime }}</div>
        </div>
        <div class="flex flex-row justify-around">
            <div>cpu: {{ container.cpu_percent }}%</div>
            <div>
                memory: {{ container.memory_percent }}%
                {{ container.memory_usage }}
            </div>
        </div>
        <div>
            <div class="flex flex-row justify-center">
                <VisBulletLegend
                    :items="[{ name: 'Memory' }, { name: 'CPU' }]"
                />
            </div>
            <VisXYContainer :data="data" class="h-48">
                <VisLine
                    :x="(data: DataRecord) => data.time_stamp"
                    :y="[
                        (data: DataRecord) => data.mem_percentage,
                        (data: DataRecord) => data.cpu_percentage,
                    ]"
                    :duration="0"
                />
                <VisAxis
                    type="x"
                    :tickFormat="
                        (value: number) =>
                            Intl.DateTimeFormat('en-US', {
                                hour: 'numeric',
                                minute: 'numeric',
                                second: 'numeric',
                                hourCycle: 'h23',
                            }).format(value)
                    "
                />
                <VisAxis
                    type="y"
                    :position="'left'"
                    :tickFormat="(value: number) => value + '%'"
                />
            </VisXYContainer>
        </div>
    </div>
</template>
