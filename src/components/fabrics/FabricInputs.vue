<template>
  <q-card>
    <q-card-section class="caption"> New Fabric </q-card-section>
    <q-card-section class="column q-gutter-sm">
      <q-input outlined dense type="text" label="Name" v-model="name" />
      <q-input type="number" label="Length" v-model.number="length" />
      <q-input type="number" label="Costs" v-model.number="costs" />
    </q-card-section>
    <q-card-section>
      <q-btn label="Save" colo="primary" @click="saveNewFabric"></q-btn>
    </q-card-section>
  </q-card>
</template>
<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import type { Fabric } from 'src/types/fabric';
import { ref } from 'vue';

const name = ref<string>('');
const length = ref<number>(0);
const costs = ref<number>(0);

const fabricStore = useFabricStore();

async function saveNewFabric() {
  await fabricStore.saveFabric(<Fabric>{
    name: name.value,
    length: length.value,
    costs: costs.value,
  });
}
</script>
