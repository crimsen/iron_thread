<template>
  <q-page class="column">
    <ProjectsList />
    <q-dialog v-model="showAddProjectDialog">
      <GenericInputs
        v-model="fab"
        :schema="schema"
        @save="save"
        @cancel="cancel"
        title="New Project"
      />
    </q-dialog>
    <q-page-sticky position="bottom-right" :offset="[18, 18]">
      <q-btn
        class="glass-btn glass-bg-primary glass-text"
        fab
        icon="add"
        @click="showAddProjectDialog = !showAddProjectDialog"
      />
    </q-page-sticky>
  </q-page>
</template>

<script setup lang="ts">
import ProjectsList from 'src/components/projects/ProjectsList.vue';
import GenericInputs from 'src/components/generic/GenericInputs.vue';
import { emitCancel, emitSave, emptyProject, schema } from 'src/components/projects/models';
import { ref } from 'vue';
import { Project } from 'src/types/extendenProject';

const fab = ref({ ...emptyProject });

const showAddProjectDialog = ref<boolean>(false);

const save = async (project: Project, file: File | undefined = undefined) => {
  await emitSave(project, file, showAddProjectDialog);
};

const cancel = () => {
  emitCancel(undefined, showAddProjectDialog);
};
</script>
