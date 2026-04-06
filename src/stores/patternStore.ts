import { acceptHMRUpdate, defineStore } from 'pinia';
import { fabricApi } from 'src/api/fabric';
import type { Fabric } from 'src/types/extendedFabric';
import { debug, error } from '@tauri-apps/plugin-log';

export const useFabricStore = defineStore('fabricStore', {
  state: () => ({
    fabrics: [] as Fabric[],
    isLoading: false,
    hasError: null as string | null,
  }),
  // getters: {},
  actions: {
    async loadFabrics() {
      this.isLoading = true;
      await debug('loading Fabrics');
      try {
        await debug('waiting for Fabrics');
        this.fabrics = await fabricApi.getAll();
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        await debug('finish loading Fabrics');
        this.isLoading = false;
      }
    },
    async saveFabric(fabric: Fabric) {
      this.isLoading = true;
      await debug(`save Fabric ${JSON.stringify(fabric)}`);
      try {
        await debug('wait to save Fabric');
        this.fabrics.push(await fabricApi.save(fabric));
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        await debug('finish save Fabric');
        this.isLoading = false;
      }
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useFabricStore, import.meta.hot));
}
