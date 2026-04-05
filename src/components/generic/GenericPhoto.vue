<template>
  <div class="relative-position self-center" style="width: fit-content">
    <q-img
      :class="`border-radius-all ${thumpnail ? 'thumpnail' : 'thumpnail-input'} self-center`"
      :src="computedImage"
      @click="triggerUpload"
    >
    </q-img>
    <q-btn
      v-if="(imageURL || modelValue) && !readonly"
      class="glass-btn glass-bg-negative q-ma-sm absolute-top-right"
      icon="delete"
      round
      size="sm"
      style="z-index: 999; pointer-events: auto"
      @click.stop="deleteImage"
    />
    <slot></slot>
    <q-file
      v-if="!readonly"
      ref="fileInputRef"
      accept="image/*"
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
// import { debug } from '@tauri-apps/plugin-log';

const modelValue = defineModel<string | null>();
const props = defineProps<{
  readonly?: boolean;
  thumpnail?: boolean;
}>();

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
  if (props.readonly) return;
  fileInputRef.value?.pickFiles();
};

const updateFabricImage = (file: File) => {
  if (imageURL.value) {
    URL.revokeObjectURL(imageURL.value);
  }
  imageURL.value = URL.createObjectURL(file);
};

const deleteImage = () => {
  if (imageURL.value) {
    URL.revokeObjectURL(imageURL.value);
    imageURL.value = undefined;
    return;
  }
  if (modelValue.value) {
    modelValue.value = undefined;
  }
  // await debug('delete ausgelöst');
};

watch(imageFile, (newVal) => {
  emit('update:file', newVal);
});
</script>
