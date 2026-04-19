<template>
  <q-page class="column">
    <FabricsList />
    <q-dialog v-model="showAddFabricDialog">
      <GenericInputs v-model="fab" :schema="schema" @save="save" @cancel="cancel" title="New Fabric" />
    </q-dialog>
    <q-page-sticky position="bottom-right" :offset="[18, 18]">
      <q-btn class="glass-btn glass-bg-primary glass-text" fab icon="add"
        @click="showAddFabricDialog = !showAddFabricDialog" />
    </q-page-sticky>
  </q-page>
</template>

<script setup lang="ts">
import FabricsList from 'src/components/fabrics/FabricsList.vue';
import GenericInputs from 'src/components/generic/GenericInputs.vue';
import { emitCancel, emitSave, emptyFabric, schema } from 'src/components/fabrics/models';
import { ref } from 'vue';
import type { Fabric } from 'src/types/extendedFabric';
import { toBase64 } from 'src/utils';

const fab = ref({ ...emptyFabric });

const showAddFabricDialog = ref<boolean>(false);

const save = async (fabric: Fabric, file: File | undefined = undefined) => {
  if (file) {
    const fileData = await toBase64(file);
    fabric.fileData = fileData;
  }
  await emitSave(fabric, showAddFabricDialog);
};

const cancel = () => {
  emitCancel(undefined, showAddFabricDialog);
};
</script>
