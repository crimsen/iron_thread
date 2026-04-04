import { defineStore, acceptHMRUpdate } from 'pinia';
import type { FabricXProject } from 'src/types/fabricXProject';

export const useMyStore = defineStore('fabricXProjectStore', {
  state: () => ({
    fabricXProjects: [] as Array<FabricXProject>,
  }),
  getters: {},
  actions: {},
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useMyStore, import.meta.hot));
}
