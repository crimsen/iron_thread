import { acceptHMRUpdate, defineStore } from 'pinia';
import { fabricApi } from 'src/api/fabric';
import type { Fabric } from 'src/types/fabric';
import { debug, error } from '@tauri-apps/plugin-log';

export const useFabricStore = defineStore('fabricStore', {
  state: () => ({
    fabrics: [] as Fabric[],
    isLoading: false,
    hasError: null as string | null,
  }),
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
    async saveFabric(fabric: Fabric, fabricImage: File | null = null) {
      this.isLoading = true;
      await debug(`save Fabric ${JSON.stringify(fabric)}`);
      try {
        await debug('wait to save Fabric');
        const newFabric = await fabricApi.save(fabric);
        if (fabricImage) {
          const path = await fabricApi.saveFabricWithImage(newFabric, fabricImage);
          if (path) newFabric.fotoPath = path;
        }
        await debug(`new Fabric is ${JSON.stringify(newFabric)}`);
        const idx = this.fabrics.findIndex((f) => f.id == newFabric.id);
        if (idx < 0) {
          this.fabrics.push(newFabric);
        } else {
          this.fabrics[idx] = { ...newFabric };
        }
        // this.fabrics.push(await fabricApi.save(fabric));
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        await debug('finish save Fabric');
        this.isLoading = false;
      }
    },
    async deleteFabric(fabric: Fabric) {
      try {
        const retVal = await fabricApi.deleteFabric(fabric);
        await debug(`delete ${JSON.stringify(retVal)}`);
        const idx = this.fabrics.findIndex((f) => f.id == retVal);
        if (idx >= 0) {
          this.fabrics.splice(idx, 1);
        }
      } catch (e) {
        await error(`error: ${JSON.stringify(e)}`);
      }
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useFabricStore, import.meta.hot));
}
