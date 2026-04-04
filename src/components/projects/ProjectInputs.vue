<template>
  <q-card class="glass border-radius-all">
    <q-card-section v-show="title" class="caption">{{ title }}</q-card-section>
    <q-card-section class="column q-gutter-sm">
      <div @click="triggerUpload">
        <q-img class="border-radius-all thumpnail-input self-center" :src="computedImage">
          <q-btn
            v-if="imageURL || fabric.fotoPath"
            class="glass-btn glass-bg-negative q-ma-sm absolute-top-right"
            icon="delete"
            round
            size="sm"
          />
        </q-img>
      </div>
      <q-file
        ref="fileInputRef"
        v-model="fabricImage"
        v-show="false"
        @update:modelValue="updateFabricImage"
      />

      <q-input outlined dense type="text" label="Name" v-model="fabric.name" />
      <q-input type="number" label="Length" v-model.number="fabric.length" />
      <q-input type="number" label="Width" v-model.number="fabric.width" />
      <q-input type="number" label="Costs" v-model.number="fabric.costs" />
      <q-input type="text" label="Producer" v-model="fabric.producer" />
      <KindOfFabricSelect v-model="fabric.kindOfFabricId" />
      <q-input type="date" label="Date of Purchase" v-model="fabric.dateOfPurchase" />
    </q-card-section>
    <q-card-actions left>
      <q-btn
        class="glass-btn glass-bg-pirmary glass-text border-radius-all"
        label="Save"
        @click="saveNewFabric"
      ></q-btn>
      <q-btn
        class="glass-btn glass-bg-pirmary glass-text border-radius-all"
        label="Test"
        @click="test"
      ></q-btn>
    </q-card-actions>
  </q-card>
</template>
<script setup lang="ts">
import { useFabricStore } from 'src/stores/fabricStore';
import type { Fabric } from 'src/types/fabric';
import { computed, onBeforeMount, ref } from 'vue';
import KindOfFabricSelect from 'src/components/kindOfFabric/KindOfFabricSelect.vue';
import type { QFile } from 'quasar';
import { debug } from '@tauri-apps/plugin-log';
import { convertFileSrc } from '@tauri-apps/api/core';

defineProps<{
  title?: string;
}>();

onBeforeMount(() => {
  if (modelFabric.value) {
    fabric.value = modelFabric.value;
  }
});

const modelFabric = defineModel<Fabric | undefined | null>();
const fileInputRef = ref<QFile>();
const fabricImage = ref(null);
const imageURL = ref();

const computedImage = computed(() => {
  return imageURL.value
    ? imageURL.value
    : fabric.value.fotoPath
      ? convertFileSrc(fabric.value.fotoPath)
      : '/public/updload_fabric.png';
});

const emptyFabric: Fabric = {
  id: -1,
  name: null,
  length: null,
  width: null,
  costs: null,
  fotoPath: null,
  producer: null,
  kindOfFabricId: null,
  dateOfPurchase: null,
};

const triggerUpload = () => {
  debug(`hallo??? ${JSON.stringify(fileInputRef.value)}`).catch(() => {});
  fileInputRef.value?.pickFiles();
};

const updateFabricImage = async (file: File) => {
  if (imageURL.value) {
    URL.revokeObjectURL(imageURL.value);
  }
  await debug(`file: ${JSON.stringify(file)}`);
  imageURL.value = URL.createObjectURL(file);
  await debug(`url: ${JSON.stringify(imageURL.value)}`);
  // if (qImg.value) {
  //   qImg.value.src = imageURL.value;
  // }
};

const fabric = ref<Fabric>(emptyFabric);

const fabricStore = useFabricStore();

async function saveNewFabric() {
  await fabricStore.saveFabric(fabric.value, fabricImage.value);
}

const test = () => {
  //await debug(`file: ${JSON.stringify(fabricImage)}`);
  if (fabric.value.fotoPath)
    // await debug(`file: ${JSON.stringify(convertFileSrc(fabric.value.fotoPath))}`);
    console.log('file', convertFileSrc(fabric.value.fotoPath));
};
</script>
