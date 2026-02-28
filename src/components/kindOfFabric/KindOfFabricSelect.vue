<template>
  <q-select
    v-model="model"
    :options="options"
    option-label="name"
    option-value="id"
    use-input
    :readonly="readonly"
    label="Kind of Fabric"
    input-debounce="0"
    map-options
    emit-value
    @filter="filterFn"
  ></q-select>
</template>

<script setup lang="ts">
//
import { useKindOfFabricStore } from 'src/stores/kindOfFabricStore';
import { computed, onBeforeMount, ref } from 'vue';
import type { KindOfFabric } from 'src/types/kindOfFabric';

onBeforeMount(async () => {
  await kindOfFabricStore.loadKindOfFabrics();
});

const model = defineModel<number | null>();
defineProps<{
  readonly?: boolean;
}>();

const kindOfFabricStore = useKindOfFabricStore();

const filter = ref<Array<KindOfFabric>>([]);

const options = ref<KindOfFabric[]>([]);

function filterFn(value: string, update: (callback: () => void) => void) {
  if (value == '') {
    update(() => {
      options.value = kindOfFabricStore.kindOfFabrics;
    });
    return;
  }
  update(() => {
    options.value = kindOfFabricStore.kindOfFabrics.filter((k) =>
      k.name.toLocaleLowerCase().includes(value.toLocaleLowerCase()),
    );
  });
}
</script>
