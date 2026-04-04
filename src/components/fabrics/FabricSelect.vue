<template>
  <q-select
    v-model="model"
    option-label="name"
    option-value="id"
    :options="options"
    use-input
    :readonly="readonly"
    label="Fabric"
    map-options
    emit-value
    @filter="filterFn"
    input-debounce="0"
  ></q-select>
</template>

<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import { computed, onBeforeMount, ref } from 'vue';

onBeforeMount(async () => {
  await fabricStore.loadFabrics();
});
const filterText = ref('');

const options = computed(() => {
  let all = fabricStore.fabrics;
  if (props.forbiddenToShow) {
    all = all.filter((p) => !props.forbiddenToShow?.some((p1) => p.id == p1));
  }
  if (!filterText.value) return all;

  const needle = filterText.value.toLowerCase();
  return all.filter((v) => v.name?.toLowerCase().includes(needle));
});

function filterFn(value: string, update: (callback: () => void) => void) {
  update(() => {
    filterText.value = value;
  });
}
const model = defineModel<number | null>();
const props = defineProps<{
  readonly?: boolean;
  forbiddenToShow?: Array<number | null>;
}>();

const fabricStore = useFabricStore();
</script>
