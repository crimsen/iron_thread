import { invoke } from '@tauri-apps/api/core';
import type { Fabric } from 'src/types/fabric.ts';
export const fabricApi = {
  async getAll(): Promise<Fabric[]> {
    return await invoke<Fabric[]>('get_fabrics');
  },
  async save(fabric: Fabric): Promise<Fabric> {
    if (typeof fabric.id != 'number') fabric.id = -1;
    return await invoke<Fabric>('save_fabric', { fabricData: fabric });
  },
};
