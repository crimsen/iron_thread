<template>
  <q-select
    v-model="overwriteModel"
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
      <q-card class="column items-center q-ma-sm bg-transparent" @click.stop.prevent flat>
        <q-card-section>
          <generic-photo :model-value="scope.opt.fotoPath" readonly thumpnail> </generic-photo>
          <q-btn
            class="class-btn glass-bg-negative absolute-top-right q-ma-sm"
            icon="delete"
            round
            size="sm"
            @click.stop.prevent="scope.removeAtIndex(scope.index)"
          />
          <q-card
            v-if="scope.opt.name"
            class="absolute-bottom-right text-subtitle2 text-white glass-bg-accent q-px-xs"
            >{{ scope.opt.name }}</q-card
          >
        </q-card-section>
        <q-card-section class="q-pa-xs" v-if="model && model[scope.index]">
          <q-input
            type="number"
            label="Length"
            :model-value="(<{ fabricId: number; length?: number }>model[scope.index]).length || 0"
            @update:model-value="(val) => updateLength(scope.index, val)"
          />
        </q-card-section>
      </q-card>
    </template>
    <template v-slot:option="scope">
      <q-item v-bind="scope.itemProps">
        <!-- :class="scope.selected ? 'glass-bg-positive' : ''" -->
        <q-item-section>
          <!-- <generic-photo :model-value="scope.opt.fotoPath" readonly thumpnail /> -->
          <q-card
            :class="`column items-center q-ma-sm bg-transparent ${scope.selected ? 'glass-bg-positive' : ''}`"
            flat
          >
            <q-card-section>
              <generic-photo :model-value="scope.opt.fotoPath" readonly thumpnail> </generic-photo>
              <q-card
                v-if="scope.opt.name"
                class="absolute-bottom-right text-subtitle2 text-white glass-bg-accent q-px-xs"
                >{{ scope.opt.name }}</q-card
              >
            </q-card-section>
          </q-card>
        </q-item-section>
      </q-item>
    </template>
    <template v-slot:selected>
      <div>test</div>
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
const updateLength = (index: number, val: number | string | null) => {
  if (model.value && model.value?.length > 0 && model.value[index] && val) {
    model.value[index].length = <number>val;
  }
};

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
const model = defineModel<Array<{ fabricId: number; length?: number | undefined }> | null>();
const overwriteModel = computed({
  get: () => model.value?.map(({ fabricId }) => fabricId),
  set: (val: Array<number>) => {
    const retVal = model.value?.filter((m) => val.includes(m.fabricId)) || [];
    const notIncluded = val.filter((v) => {
      if (retVal.findIndex((r) => r.fabricId == v) >= 0) return false;
      return true;
    });
    notIncluded.forEach((v) => {
      retVal.push({ fabricId: v, length: undefined });
    });
    model.value = retVal;
  },
});
const props = defineProps<{
  readonly?: boolean;
  forbiddenToShow?: Array<number | null>;
}>();

const fabricStore = useFabricStore();
</script>
