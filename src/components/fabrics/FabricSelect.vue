<template>
  <q-select
    v-model="model"
    option-label="name"
    option-value="id"
    :options="options"
    multiple
    use-input
    :readonly="readonly"
    label="Fabric"
    map-options
    emit-value
    @filter="filterFn"
    input-debounce="0"
  >
    <template v-slot:selected-item="scope">
      <q-card class="column items-center q-ma-sm bg-transparent">
        <generic-photo :model-value="scope.opt.fotoPath" readonly thumpnail>
          <!-- style="z-index: 999; pointer-events: auto" -->
        </generic-photo>
        <q-btn
          class="class-btn glass-bg-negative absolute-top-right q-ma-sm"
          icon="delete"
          round
          size="sm"
          @click.stop.prevent="scope.removeAtIndex(scope.index)"
        />
        <q-card-section class="q-pa-xs">
          <q-input type="number" label="Length" v-model="length" />
        </q-card-section>
      </q-card>
    </template>
    <template v-slot:option="scope">
      <q-item v-bind="scope.itemProps">
        <q-item-section>
          <generic-photo :model-value="scope.opt.fotoPath" readonly thumpnail />
        </q-item-section>
      </q-item>
    </template>
  </q-select>
</template>

<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import { computed, onBeforeMount, ref } from 'vue';
// import FabricCard from './FabricCard.vue';
// import type { Fabric } from 'src/types/fabric';
import GenericPhoto from '../generic/GenericPhoto.vue';

onBeforeMount(async () => {
  await fabricStore.loadFabrics();
});
const filterText = ref('');
const length = ref();

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
const model = defineModel<Array<number> | null>();
const props = defineProps<{
  readonly?: boolean;
  forbiddenToShow?: Array<number | null>;
}>();

const fabricStore = useFabricStore();
</script>
