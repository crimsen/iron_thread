import { acceptHMRUpdate, defineStore } from 'pinia';
import { debug, error } from '@tauri-apps/plugin-log';
import type { KindOfFabric } from 'src/types/kindOfFabric';
import { kindOfFabricApi } from 'src/api/kindOfFabric';

export const useKindOfFabricStore = defineStore('kindOfFabricStore', {
  state: () => ({
    kindOfFabrics: [] as KindOfFabric[],
    isLoading: false,
    hasError: null as string | null,
  }),
  getters: {
    getKindOfFabricById: (state) => {
      return (id: number): KindOfFabric | undefined => {
        debug(`search kind of fabric width id = ${id}`).catch(() => {});
        const retVal = state.kindOfFabrics.find((k: KindOfFabric) => {
          return k.id == id;
        });
        debug(`found kind of fabric = ${JSON.stringify(retVal)}`).catch(() => {});
        return retVal;
      };
    },
    filterKindOfFabricByName: (state) => {
      return (filterName: string): KindOfFabric[] => {
        debug(`filterName: ${filterName}`).catch((e) => console.error(e));
        return state.kindOfFabrics.filter((k) => {
          const lowerKey = k.name.toLowerCase();
          debug(`lowerKey: ${lowerKey}`).catch((e) => console.error(e));
          const retVal = lowerKey.includes(filterName.toLowerCase());
          debug(`retVal: ${retVal}`).catch((e) => console.error(e));
          return retVal;
        });
      };
    },
  },
  actions: {
    async loadKindOfFabrics() {
      this.isLoading = true;
      await debug('loading KindOfFabrics');
      try {
        await debug('waiting for KindOfFabrics');
        this.kindOfFabrics = await kindOfFabricApi.getAll();
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        await debug('finish loading KindOfFabrics');
        this.isLoading = false;
      }
    },
    async saveKindOfFabric(kindOfFabric: KindOfFabric) {
      this.isLoading = true;
      await debug(`save KindOfFabric ${JSON.stringify(kindOfFabric)}`);
      try {
        await debug('wait to save KindOfFabric');
        this.kindOfFabrics.push(await kindOfFabricApi.save(kindOfFabric));
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        await debug('finish save KindOfFabric');
        this.isLoading = false;
      }
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useKindOfFabricStore, import.meta.hot));
}
