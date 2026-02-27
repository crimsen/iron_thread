import { invoke } from '@tauri-apps/api/core';
import type { KindOfFabric } from 'src/types/kindOfFabric.ts';
export const kindOfFabricApi = {
  async getAll(): Promise<KindOfFabric[]> {
    return await invoke<KindOfFabric[]>('get_kind_of_fabrics');
  },
  async save(kindOfFabric: KindOfFabric): Promise<KindOfFabric> {
    if (typeof kindOfFabric.id != 'number') kindOfFabric.id = -1;
    return await invoke<KindOfFabric>('save_kind_of_fabric', {
      kindOfFabricData: kindOfFabric,
    });
  },
};
