<template>
  <q-card class="q-ma-sm border" v-ripple @click="openFabric">
    <q-card-section horizontal>
      <div class="col-auto">
        <q-img class="thumpnail border-left" src="https://cdn.quasar.dev/img/parallax2.jpg" />
      </div>

      <div class="col">
        <!--<q-btn flat round color="red" icon="favorite" />-->
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
  <q-dialog v-model="showDialog">
    <FabricInputs v-model="modifyFabric" />
  </q-dialog>
</template>

<script setup lang="ts">
import { useKindOfFabricStore } from 'src/stores/kindOfFabricStore';
import type { Fabric } from 'src/types/fabric';
import { computed, onBeforeMount, ref } from 'vue';
import FabricInputs from './FabricInputs.vue';

onBeforeMount(async () => {
  await kindOfFabricStore.loadKindOfFabrics();
});

const props = defineProps<{ fabric: Fabric }>();

const kindOfFabric = computed(() => {
  return kindOfFabricStore.getKindOfFabricById(props.fabric.kindOfFabricId || -1);
});

const showDialog = ref(false);
const modifyFabric = ref<Fabric>();

const openFabric = () => {
  modifyFabric.value = props.fabric;
  showDialog.value = true;
};

const kindOfFabricStore = useKindOfFabricStore();
</script>
<style lang="scss" scoped>
.thumpnail {
  height: 6rem;
  width: 6rem;
  object-fit: cover;
}

.border {
  border-radius: 0.5rem;
}

.border-left {
  border-radius: 0.5rem 0 0 0.5rem;
}
</style>
