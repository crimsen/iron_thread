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
        <FabricSelect
          v-else-if="field.type == 'ModifyFabrics'"
          v-model="modelCopy[field.key]"
          :label="field.label"
          :type="field.type"
        />
        <!-- <ModifyFabrics -->
        <!--   v-else-if="field.type == 'ModifyFabrics'" -->
        <!--   v-model="modelCopy[field.key]" -->
        <!--   :label="field.label" -->
        <!--   :type="field.type" -->
        <!--   :readonly="field.readonly" -->
        <!--   :classes="'column'" -->
        <!--   :subclass="'q-gutter-sm'" -->
        <!-- /> -->
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
        v-if="deletable"
        class="glass-btn glass-bg-negative glass-text border-radius-all"
        label="Delete"
        @click="deleteModelValue"
      >
      </q-btn>
      <q-btn
        class="glass-btn glass-bg-negative glass-text border-radius-all"
        label="Cancel"
        @click="emits('cancel')"
      />
      <q-btn
        class="glass-btn glass-bg-pirmary glass-text border-radius-all"
        label="Save"
        @click="saveModelValue"
      ></q-btn>
    </q-card-actions>
  </q-card>
</template>
<script setup lang="ts" generic="T extends Record<string, any>">
import { onBeforeMount, ref, watch } from 'vue';
import KindOfFabricSelect from '../kindOfFabric/KindOfFabricSelect.vue';
// import ModifyFabrics from '../projects/ModifyFabrics.vue';
import GenericPhoto from './GenericPhoto.vue';
import type { FieldConfigFabric, FieldConfigProject } from './genericType';
import { debug, info } from '@tauri-apps/plugin-log';
import FabricSelect from '../fabrics/FabricSelect.vue';

onBeforeMount(async () => {
  await info(`mop: ${JSON.stringify(modelValue.value)}`);
  await info(`schema: ${JSON.stringify(props.schema)}`);
});

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

const props = defineProps<{
  schema: FieldConfigFabric[] | FieldConfigProject[];
  title?: string;
  deletable?: boolean;
}>();

const emits = defineEmits<{
  save: [data: T, photo: File | undefined];
  delete: [data: T];
  cancel: [];
}>();
const saveModelValue = () => {
  modelValue.value = { ...modelCopy.value };
  debug(
    `hier bin ich ? ${JSON.stringify(modelValue.value)} ${JSON.stringify(modelCopy.value)}`,
  ).catch(() => {});
  emits('save', modelCopy.value, file.value);
};
const deleteModelValue = () => {
  emits('delete', modelCopy.value);
};
</script>
