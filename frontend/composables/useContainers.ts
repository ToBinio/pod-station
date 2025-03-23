import type { Container } from "~/utils/containers";

export async function useContainers() {
  const config = useRuntimeConfig();

  const containers = ref<Container[]>([]);

  const { data: fetchContainers } = await useFetch<Container[]>("/containers", {
    baseURL: config.public.baseURL as string,
  });

  const { data: wsContainers } = useWebSocket<string>(
    "ws://localhost:8080/containers/ws",
  );

  watch(
    fetchContainers,
    (data) => {
      if (data) containers.value = data;
    },
    { immediate: true },
  );

  watch(wsContainers, (data) => {
    if (data) containers.value = JSON.parse(data);
  });

  return containers;
}
