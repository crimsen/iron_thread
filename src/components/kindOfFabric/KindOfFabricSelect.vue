<template>
  <q-select v-model.number="model" :options="computedOptions" option-label="name" option-value="id" use-input
    label="Kind of Fabric" input-debounce="0" @filter="filterFn"></q-select>
</template>

<script setup lang="ts">
//
import { useKindOfFabricStore } from 'src/stores/kindOfFabricStore';
import { computed, onBeforeMount, ref } from 'vue';
import type { KindOfFabric } from 'src/types/kindOfFabric';

onBeforeMount(async () => {
  await kindOfFabricStore.loadKindOfFabrics();
});

const model = defineModel<number | null>();

const kindOfFabricStore = useKindOfFabricStore();

const filter = ref<Array<KindOfFabric>>([]);

const computedOptions = computed({
  get: () => (filter.value.length > 0 ? filter.value : kindOfFabricStore.kindOfFabrics),
  set: (val) => (filter.value = val),
});

function filterFn(value: string, update: (callback: () => void) => void) {
  update(() => {
    computedOptions.value = kindOfFabricStore.filterKindOfFabricByName(value);
  });
}
</script>
