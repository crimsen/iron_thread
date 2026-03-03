<template>
  <q-card class="glass border-radius-all">
    <q-toolbar v-if="title">{{ title }}</q-toolbar>
    <q-card-section class="column q-gutter-sm">
      <div v-for="field in schema" :key="field.key">
        <GenericPhoto v-if="field.type == 'fotoPath'" v-model="modelValue[field.key]" />
        <KindOfFabricSelect
          v-else-if="field.type == 'KindOfFabricSelect'"
          v-model="modelValue[field.key]"
        />
        <q-input
          v-else
          v-model="modelValue[field.key]"
          :label="field.label"
          :type="field.type"
          :readonly="field.readonly"
        />
      </div>
    </q-card-section>
  </q-card>
</template>
<script setup lang="ts" generic="T extends Record<string, any>">
import KindOfFabricSelect from '../kindOfFabric/KindOfFabricSelect.vue';
import GenericPhoto from './GenericPhoto.vue';
import type { FieldConfigFabric } from './genericType';

const modelValue = defineModel<T>({ required: true });

defineProps<{
  schema: FieldConfigFabric[];
  title?: string;
}>();

defineEmits(['save', 'cancel']);
</script>
