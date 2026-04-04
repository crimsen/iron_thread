import { invoke } from '@tauri-apps/api/core';
import type { FabricXProject } from 'src/types/fabricXProject';
export const fabricXProjectApi = {
  async getAll(): Promise<FabricXProject[]> {
    return await invoke<Array<FabricXProject>>('get_fabric_x_projects');
  },
  // async save(fabricXProject: FabricXProject): Promise<FabricXProject> {
  //   return Promise({})
  // }
};
