<template>
  <!--<q-page class="row items-center justify-evenly"> hi </q-page>-->
  <q-page class="column">
    <FabricsList />
    <q-dialog v-model="showAddFabricDialog">
      <FabricInputs />
    </q-dialog>
    <q-page-sticky position="bottom-right" :offset="[18, 18]">
      <q-btn fab icon="add" color="accent" @click="showAddFabricDialog = !showAddFabricDialog" />
    </q-page-sticky>
  </q-page>
</template>

<script setup lang="ts">
import FabricsList from 'src/components/fabrics/FabricsList.vue';
import FabricInputs from 'src/components/fabrics/FabricInputs.vue';
import { ref } from 'vue';

const showAddFabricDialog = ref<boolean>(false);
</script>
