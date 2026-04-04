<template>
  <!-- <q-list bordered seperator> -->
  <!--   <ProjectsListItem -->
  <!--     v-for="(project, index) in projectStore.projects" -->
  <!--     :key="`project-${index}`" -->
  <!--     :project="project" -->
  <!--   /> -->
  <!-- </q-list> -->
  <div
    class="row q-gutter-xs justify-center"
    v-for="(project, index) in projectStore.projects"
    :key="`project-${index}`"
  >
    <div class="col">
      <ProjectCard :project="project" />
    </div>
  </div>
</template>
<script setup lang="ts">
import { useProjectStore } from 'src/stores/projectStore';
// import ProjectsListItem from './ProjectsListItem.vue';
import ProjectCard from './ProjectCard.vue';
import { onBeforeMount } from 'vue';

const projectStore = useProjectStore();

onBeforeMount(async () => {
  await projectStore.loadProjects();
});
</script>
