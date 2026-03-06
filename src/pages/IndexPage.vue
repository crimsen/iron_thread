<template>
  <!--<q-page class="row items-center justify-evenly"> hi </q-page>-->
  <q-page class="column">
    <FabricsList />
    <q-dialog v-model="showAddFabricDialog">
      <GenericInputs
        v-model="fab"
        :schema="schema"
        @save="save"
        @cancel="cancel"
        title="New Fabric"
      />
    </q-dialog>
    <q-page-sticky position="bottom-right" :offset="[18, 18]">
      <q-btn
        class="glass-btn glass-bg-primary glass-text"
        fab
        icon="add"
        @click="showAddFabricDialog = !showAddFabricDialog"
      />
    </q-page-sticky>
  </q-page>
</template>

<script setup lang="ts">
import FabricsList from 'src/components/fabrics/FabricsList.vue';
import GenericInputs from 'src/components/generic/GenericInputs.vue';
import { emitCancel, emitSave, emptyFabric, schema } from 'src/components/fabrics/models';
import { ref } from 'vue';
import type { Fabric } from 'src/types/fabric';

const fab = ref({ ...emptyFabric });

const showAddFabricDialog = ref<boolean>(false);

const save = async (fabric: Fabric, file: File | undefined = undefined) => {
  await emitSave(fabric, file, showAddFabricDialog);
};

const cancel = () => {
  emitCancel(undefined, showAddFabricDialog);
};
</script>
