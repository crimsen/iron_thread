<template>
  <q-card>
    <q-card-section class="caption"> New Fabric </q-card-section>
    <q-card-section class="column q-gutter-sm">
      <q-input outlined dense type="text" label="Name" v-model="fabric.name" />
      <q-input type="number" label="Length" v-model.number="fabric.length" />
      <q-input type="number" label="Width" v-model.number="fabric.width" />
      <q-input type="number" label="Costs" v-model.number="fabric.costs" />
      <q-input type="text" label="Producer" v-model="fabric.producer" />
      <q-select v-model.number="fabric.kindOfFabricId" :options="computedOptions" option-label="name" option-value="id"
        use-input input-debounce="0" @filter="filterFn"></q-select>
      <q-input type="date" label="Date of Purchase" v-model="fabric.dateOfPurchase" />
    </q-card-section>
    <q-card-section>
      <q-btn label="Save" colo="primary" @click="saveNewFabric"></q-btn>
    </q-card-section>
  </q-card>
</template>
<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import { useKindOfFabricStore } from 'src/stores/kindOfFabricStore';
import type { Fabric } from 'src/types/fabric';
import type { KindOfFabric } from 'src/types/kindOfFabric';
import { computed, ref } from 'vue';
const emptyFabric: Fabric = {
  id: -1,
  name: null,
  length: null,
  width: null,
  costs: null,
  fotoPath: null,
  producer: null,
  kindOfFabricId: null,
  dateOfPurchase: null,
};
const fabric = ref<Fabric>(emptyFabric);

const fabricStore = useFabricStore();
const kindOfFabricStore = useKindOfFabricStore();

async function saveNewFabric() {
  await fabricStore.saveFabric(fabric.value);
}

const filter = ref<Array<KindOfFabric>>([]);

const computedOptions = computed({
  get: () => (filter.value.length > 0 ? filter.value : kindOfFabricStore.kindOfFabrics),
  set: (val) => (filter.value = val),
});

async function filterFn(value: string, update: (callback: () => void) => void) {
  update(() => {
    computedOptions.value = kindOfFabricStore.filterKindOfFabricByName(value);
  });
}
</script>
