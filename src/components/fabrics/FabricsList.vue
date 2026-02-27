<template>
  <q-list bordered seperator>
    <FabricsListItem v-for="(fabric, index) in fabricStore.fabrics" :key="`fabric-${index}`" :fabric="fabric" />
  </q-list>
</template>
<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import FabricsListItem from './FabricsListItem.vue';
import { onBeforeMount } from 'vue';

const fabricStore = useFabricStore();

onBeforeMount(async () => {
  await fabricStore.loadFabrics();
});
</script>
