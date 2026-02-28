<template>
  <!--<q-list bordered seperator>
    <FabricsListItem v-for="(fabric, index) in fabricStore.fabrics" :key="`fabric-${index}`" :fabric="fabric" />
  </q-list>-->
  <div
    class="row q-gutter-xs justify-center"
    v-for="(fabric, index) in fabricStore.fabrics"
    :key="`fabric-${index}`"
  >
    <div class="col">
      <FabricCard :fabric="fabric" />
    </div>
  </div>
</template>
<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
// import FabricsListItem from './FabricsListItem.vue';
import FabricCard from './FabricCard.vue';
import { onBeforeMount } from 'vue';

const fabricStore = useFabricStore();

onBeforeMount(async () => {
  await fabricStore.loadFabrics();
});
</script>
