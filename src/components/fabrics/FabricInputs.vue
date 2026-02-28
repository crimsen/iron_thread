<template>
  <q-card>
    <q-card-section v-show="title" class="caption">{{ title }}</q-card-section>
    <q-card-section class="column q-gutter-sm">
      <q-input outlined dense type="text" label="Name" v-model="fabric.name" />
      <q-input type="number" label="Length" v-model.number="fabric.length" />
      <q-input type="number" label="Width" v-model.number="fabric.width" />
      <q-input type="number" label="Costs" v-model.number="fabric.costs" />
      <q-input type="text" label="Producer" v-model="fabric.producer" />
      <KindOfFabricSelect v-model="fabric.kindOfFabricId" />
      <q-input type="date" label="Date of Purchase" v-model="fabric.dateOfPurchase" />
    </q-card-section>
    <q-card-section>
      <q-btn label="Save" colo="primary" @click="saveNewFabric"></q-btn>
    </q-card-section>
  </q-card>
</template>
<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import type { Fabric } from 'src/types/fabric';
import { onBeforeMount, ref } from 'vue';

defineProps<{
  title?: string;
}>();

onBeforeMount(() => {
  if (modelFabric.value) {
    fabric.value = modelFabric.value;
  }
});

const modelFabric = defineModel<Fabric | undefined>();

import KindOfFabricSelect from 'src/components/kindOfFabric/KindOfFabricSelect.vue';

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

async function saveNewFabric() {
  await fabricStore.saveFabric(fabric.value);
}
</script>
