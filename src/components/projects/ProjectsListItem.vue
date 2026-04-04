<template>
  <q-item>
    <q-item-section>
      <q-item-label>{{ project.name }}</q-item-label>
      <q-item-label>{{ project.size }}</q-item-label>
      <!-- <q-item-label caption>Length: {{ project.length }}</q-item-label> -->
      <!-- <q-item-label caption>Costs: {{ project.costs }}</q-item-label> -->
    </q-item-section>
  </q-item>
</template>
<script setup lang="ts">
import type { Project } from 'src/types/extendenProject';

defineProps<{
  project: Project;
}>();
</script>
