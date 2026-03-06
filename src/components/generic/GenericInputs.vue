<template>
  <q-card class="glass border-radius-all">
    <q-toolbar v-if="title">{{ title }}</q-toolbar>
    <q-card-section class="column q-gutter-sm">
      <div v-for="field in schema" :key="field.key">
        <div v-if="field.type == 'fotoPath'" class="column items-center">
          <GenericPhoto v-model="modelCopy[field.key]" @update:file="updateFile" />
        </div>
        <KindOfFabricSelect
          v-else-if="field.type == 'KindOfFabricSelect'"
          v-model="modelCopy[field.key]"
        />
        <q-input
          v-else-if="field.type == 'number'"
          v-model.number="modelCopy[field.key]"
          :label="field.label"
          :type="field.type"
          :readonly="field.readonly"
        />
        <q-input
          v-else
          v-model="modelCopy[field.key]"
          :label="field.label"
          :type="field.type"
          :readonly="field.readonly"
        />
      </div>
    </q-card-section>
    <q-card-actions left>
      <q-btn
        class="glass-btn glass-bg-pirmary glass-text border-radius-all"
        label="Save"
        @click="saveModelValue"
      ></q-btn>
      <q-btn
        class="glass-btn glass-bg-negative glass-text border-radius-all"
        label="Cancel"
        @click="emits('cancel')"
      />
    </q-card-actions>
  </q-card>
</template>
<script setup lang="ts" generic="T extends Record<string, any>">
import { ref, watch } from 'vue';
import KindOfFabricSelect from '../kindOfFabric/KindOfFabricSelect.vue';
import GenericPhoto from './GenericPhoto.vue';
import type { FieldConfigFabric } from './genericType';
import { debug } from '@tauri-apps/plugin-log';

const modelValue = defineModel<T>({ required: true });
const modelCopy = ref<T>({ ...modelValue.value });

const updateFile = (value: File | undefined) => {
  file.value = value;
};

const file = ref<File>();

watch(
  () => modelValue.value,
  (newVal: T) => {
    modelCopy.value = { ...newVal };
  },
  { deep: true },
);

defineProps<{
  schema: FieldConfigFabric[];
  title?: string;
}>();

const emits = defineEmits<{
  save: [data: T, photo: File | undefined];
  cancel: [];
}>();
const saveModelValue = () => {
  modelValue.value = { ...modelCopy.value };
  debug(
    `hier bin ich ? ${JSON.stringify(modelValue.value)} ${JSON.stringify(modelCopy.value)}`,
  ).catch(() => {});
  emits('save', modelCopy.value, file.value);
};
</script>
