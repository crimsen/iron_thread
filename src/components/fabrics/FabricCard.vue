<template>
  <q-card class="glass q-ma-sm border-radius-all" v-ripple @click="openFabric">
    <q-card-section horizontal>
      <div class="col-auto">
        <q-img class="thumpnail border-radius-left" :src="image" />
      </div>

      <div class="col">
        <div class="row full-width justify-center">
          <div class="col-6 q-px-sm q-py-xs">
            <q-input readonly :modelValue="fabric.length" label="Length" />
          </div>
          <div class="col-6 q-px-sm q-py-xs">
            <q-input readonly :modelValue="fabric.width" label="Width" />
          </div>
          <div class="col-6 q-px-sm q-py-xs">
            <q-input readonly :modelValue="fabric.producer" label="producer" />
          </div>
          <div class="col-6 q-px-sm q-py-xs">
            <q-input readonly :modelValue="kindOfFabric?.name" />
          </div>
        </div>
      </div>
    </q-card-section>
  </q-card>
  <q-dialog v-model="showDialog" v-if="modifyFabric">
    <!--<FabricInputs v-model="modifyFabric" />-->
    <GenericInputs v-model="modifyFabric" :schema="schema" />
  </q-dialog>
</template>

<script setup lang="ts">
import { useKindOfFabricStore } from 'src/stores/kindOfFabricStore';
import type { Fabric } from 'src/types/fabric';
import { computed, onBeforeMount, ref } from 'vue';
// import FabricInputs from './FabricInputs.vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import { debug } from '@tauri-apps/plugin-log';
import type { FieldConfigFabric } from '../generic/genericType';
import GenericInputs from '../generic/GenericInputs.vue';

onBeforeMount(async () => {
  await kindOfFabricStore.loadKindOfFabrics();
  if (props.fabric.fotoPath) await debug(props.fabric.fotoPath);
});

const props = defineProps<{ fabric: Fabric }>();

const schema: FieldConfigFabric[] = [
  { key: 'fotoPath', label: 'Foto', type: 'fotoPath' },
  { key: 'name', label: 'Name', type: 'text' },
  { key: 'length', label: 'Length', type: 'number' },
  { key: 'width', label: 'Width', type: 'number' },
  { key: 'kindOfFabricId', label: 'Kind of Fabric', type: 'KindOfFabricSelect' },
  { key: 'costs', label: 'Costs', type: 'number' },
  { key: 'producer', label: 'Producer', type: 'text' },
  { key: 'dateOfPurchase', label: 'Date of purchase', type: 'date' },
];

const kindOfFabric = computed(() => {
  return kindOfFabricStore.getKindOfFabricById(props.fabric.kindOfFabricId || -1);
});

const showDialog = ref(false);
const modifyFabric = ref<Fabric>();

const openFabric = () => {
  modifyFabric.value = props.fabric;
  showDialog.value = true;
};

const image = computed(() => {
  if (props.fabric.fotoPath) {
    const letVal = convertFileSrc(props.fabric.fotoPath);
    return letVal;
  }
  return '/public/updload_fabric.png';
});

const kindOfFabricStore = useKindOfFabricStore();
</script>
<style lang="scss" scoped></style>
