import { error, info } from '@tauri-apps/plugin-log';
import { defineStore, acceptHMRUpdate } from 'pinia';
import { fabricXProjectApi } from 'src/api/fabricXProject';
import type { FabricXProject } from 'src/types/fabricXProject';

export const useFabricXProject = defineStore('fabricXProjectStore', {
  state: () => ({
    fabricXProjects: new Map<{ fabricId: number; projectId: number }, number | null>(),
    isLoading: false,
    hasError: null as string | null,
  }),
  getters: {},
  actions: {
    async loadingFabricXProject() {
      this.isLoading = true;
      try {
        const projectsArray = await fabricXProjectApi.getAll();
        this.fabricXProjects = new Map(
          projectsArray.map((project) => [
            { fabricId: project.fabricId, projectId: project.projectId },
            project.fabricLength,
          ]),
        );
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        this.isLoading = false;
      }
    },
    async saveFabricXProject(fabricXProject: FabricXProject) {
      this.isLoading = true;
      try {
        const savedProject = await fabricXProjectApi.save(fabricXProject);
        await info(`savedProjec: ${JSON.stringify(savedProject)}`);
        // const key = { fabricId: savedProject.fabricId, projectId: savedProject.projectId };

        // this.fabricXProjects.set(key, savedProject.fabricLength);
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        this.isLoading = false;
      }
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useFabricXProject, import.meta.hot));
}
