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
    <q-file
      ref="fileInputRef"
      accept="image/*a"
      v-model="imageFile"
      v-show="false"
      capture="environment"
      @update:modelValue="updateFabricImage"
    />
  </div>
</template>
<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { QFile } from 'quasar';

const modelValue = defineModel<string>();

const fileInputRef = ref<QFile>();

const emit = defineEmits<{
  'update:file': [data: File | undefined];
}>();

const imageURL = ref();
const imageFile = ref<File>();

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

const updateFabricImage = (file: File) => {
  if (imageURL.value) {
    URL.revokeObjectURL(imageURL.value);
  }
  // await debug(`file: ${JSON.stringify(file)}`);
  imageURL.value = URL.createObjectURL(file);
  // await debug(`url: ${JSON.stringify(imageURL.value)}`);
  // if (qImg.value) {
  //   qImg.value.src = imageURL.value;
  // }
};

watch(imageFile, (newVal) => {
  emit('update:file', newVal);
});
</script>
