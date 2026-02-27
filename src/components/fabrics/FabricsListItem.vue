<template>
  <q-item>
    <q-item-section>
      <q-item-label>{{ fabric.name }}</q-item-label>
      <q-item-label caption>Length: {{ fabric.length }}</q-item-label>
      <q-item-label caption>Costs: {{ fabric.costs }}</q-item-label>
    </q-item-section>
  </q-item>
</template>
<script setup lang="ts">
import type { Fabric } from 'src/types/fabric';

defineProps<{
  fabric: Fabric;
}>();
</script>
