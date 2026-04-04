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
  >
    <template v-slot:selected-item="scope">
      <fabric-card :fabric="<Fabric>scope.opt" readonly />
    </template>
    <template v-slot:option="scope">
      <div v-bind="scope.itemProps">
        <fabric-card :fabric="<Fabric>scope.opt" readonly />
      </div>
    </template>
  </q-select>
</template>

<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import { computed, onBeforeMount, ref } from 'vue';
import FabricCard from './FabricCard.vue';
import type { Fabric } from 'src/types/fabric';

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
