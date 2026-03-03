<template>
  <div @click="triggerUpload">
    <q-img class="border-radius-all thumpnail-input self-center" :src="computedImage">
      <q-btn
        v-if="imageURL || modelValue"
        class="glass-btn glass-bg-negative q-ma-sm absolute-top-right"
        icon="delete"
        round
        size="sm"
      />
    </q-img>
  </div>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { QFile } from 'quasar';

const modelValue = defineModel<string>();

const fileInputRef = ref<QFile>();

const imageURL = ref();

const computedImage = computed(() => {
  return imageURL.value
    ? imageURL.value
    : modelValue.value
      ? convertFileSrc(modelValue.value)
      : '/updload_fabric.png';
});

const triggerUpload = () => {
  // debug(`hallo??? ${JSON.stringify(fileInputRef.value)}`).catch(() => {});
  fileInputRef.value?.pickFiles();
};
</script>
