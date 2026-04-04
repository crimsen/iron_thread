<template>
  <div :class="`${classes} ${subclass}`">
    <div v-for="(_, index) in model" :key="`fabric_select_${index}`">
      <div>
        <FabricSelect
          v-if="typeof model[index] != 'undefined'"
          v-model="model[index]"
          :forbiddenToShow="addModel(model[index])"
        />
      </div>
    </div>
    <q-btn class="glass-btn" v-if="canAdd" @click="add">add</q-btn>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import FabricSelect from '../fabrics/FabricSelect.vue';
const model = defineModel<Array<number | null>>({ required: true, default: [] });
const add = () => {
  model.value.push(null);
};
const canAdd = computed(() => {
  return !model.value.some((p) => p === null);
});
const addModel = (fabricId: number | null) => {
  return model.value.filter((p) => p != fabricId);
};
withDefaults(
  defineProps<{
    classes?: string;
    subclass?: string;
  }>(),
  {
    classes: '',
    subclass: '',
  },
);
</script>
