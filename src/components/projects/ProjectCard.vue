<template>
  <q-card class="glass q-ma-sm border-radius-all" v-ripple @click="openProject">
    <q-card-section horizontal>
      <div class="col-auto">
        <q-img class="thumpnail border-radius-left" :src="image" />
      </div>

      <div class="col">
        <div class="row full-width justify-center">
          <div class="col-6 q-px-sm q-py-xs">
            <q-input readonly :modelValue="project.name" label="Name" />
          </div>
          <div class="col-6 q-px-sm q-py-xs">
            <q-input readonly :modelValue="project.size" label="Size" />
          </div>
        </div>
      </div>
    </q-card-section>
  </q-card>
  <q-dialog v-model="showDialog" v-if="modifyProject">
    <GenericInputs
      v-model="modifyProject"
      :schema="schema"
      @save="save"
      @cancel="cancel"
      @delete="deleteProject"
      deletable
    />
  </q-dialog>
</template>

<script setup lang="ts">
import type { Project } from 'src/types/extendenProject';
import { computed, onBeforeMount, ref } from 'vue';
// import { convertFileSrc } from '@tauri-apps/api/core';
// import { debug } from '@tauri-apps/plugin-log';
import GenericInputs from '../generic/GenericInputs.vue';
// TODO: emits must be rewrite
// import { emitSave, schema, emitCancel, emitDelete } from './models';
import { schema, emitCancel } from './models';
import { info } from '@tauri-apps/plugin-log';

onBeforeMount(async () => {
  // await kindOfProjectStore.loadKindOfProjects();
  // if (props.project.fotoPath) await debug(props.project.fotoPath);
  // TODO: loading of fabric_x_projects
  await info(`fabric: ${JSON.stringify(props.project)}`);
});

const props = defineProps<{ project: Project }>();

// TODO: replace with fabric_x_project
// const kindOfProject = computed(() => {
//   return kindOfProjectStore.getKindOfProjectById(props.project.kindOfProjectId || -1);
// });

const showDialog = ref(false);
const modifyProject = ref<Project>();

const openProject = () => {
  modifyProject.value = props.project;
  showDialog.value = true;
};

const image = computed(() => {
  // TODO: Project has no fotos yet.
  // if (props.project.fotoPath) {
  //   const letVal = convertFileSrc(props.project.fotoPath);
  //   return letVal;
  // }
  return '/public/updload_fabric.png';
});

// TODO: replace with fabric_x_project
// const kindOfProjectStore = useKindOfProjectStore();
/* eslint-disable */
// TODO: does not work yet.
const save = async (project: Project, file: File | undefined = undefined) => {
  // await emitSave(project, file, showDialog);
  await info('click save');
};
const cancel = () => {
  emitCancel(modifyProject, showDialog);
};
const deleteProject = async (project: Project) => {
  // await emitDelete(project, showDialog);
  await info('click delete');
};
/* eslint-enable */
</script>
<style lang="scss" scoped></style>
