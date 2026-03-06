<template>
  <q-select
    v-model="model"
    :options="options"
    option-label="name"
    option-value="id"
    use-input
    :readonly="readonly"
    label="Kind of Fabric"
    map-options
    emit-value
    @filter="filterFn"
    input-debounce="0"
  ></q-select>
</template>

<script setup lang="ts">
//
import { useKindOfFabricStore } from 'src/stores/kindOfFabricStore';
import { computed, onBeforeMount, ref } from 'vue';

onBeforeMount(async () => {
  await kindOfFabricStore.loadKindOfFabrics();
});
const filterText = ref('');

const options = computed(() => {
  const all = kindOfFabricStore.kindOfFabrics;
  if (!filterText.value) return all;

  const needle = filterText.value.toLowerCase();
  return all.filter((v) => v.name.toLowerCase().includes(needle));
});

function filterFn(value: string, update: (callback: () => void) => void) {
  update(() => {
    filterText.value = value;
  });
}
const model = defineModel<number | null>();
defineProps<{
  readonly?: boolean;
}>();

const kindOfFabricStore = useKindOfFabricStore();
</script>
