import { acceptHMRUpdate, defineStore } from 'pinia';
import { projectApi } from 'src/api/project';
import type { Project } from 'src/types/extendenProject';
import { debug, error, info } from '@tauri-apps/plugin-log';
import { useFabricXProject } from './fabricXProject';

export const useProjectStore = defineStore('projectStore', {
  state: () => ({
    projects: [] as Project[],
    isLoading: false,
    hasError: null as string | null,
  }),
  // getters: {},
  actions: {
    async loadProjects() {
      this.isLoading = true;
      await debug('loading Projects');
      try {
        await debug('waiting for Projects');
        this.projects = await projectApi.getAll();
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        await debug('finish loading Projects');
        this.isLoading = false;
      }
      const fabricXProject = useFabricXProject();
      await fabricXProject.loadingFabricXProject();
      this.projects.forEach((p) => {
        p.fabricIds = [
          ...fabricXProject.fabricXProjects
            .entries()
            .filter(([key]) => key.projectId == p.id)
            .map(([key]) => key.fabricId),
        ];
      });
    },
    async saveProject(project: Project) {
      this.isLoading = true;
      await debug(`save Project ${JSON.stringify(project)}`);
      try {
        await debug('wait to save Project');
        const _project = await projectApi.save(project);
        const fabricXProjectStore = useFabricXProject();
        await fabricXProjectStore.loadingFabricXProject();
        _project.fabricIds = [
          ...fabricXProjectStore.fabricXProjects
            .entries()
            .filter(([key]) => key.projectId == _project.id)
            .map(([key]) => key.fabricId),
        ];
        await info(`project is ${JSON.stringify(_project)}`);
        const idx = this.projects.findIndex((p) => p.id == _project.id);
        if (idx > -1) {
          this.projects[idx] = _project;
          return;
        }
        this.projects.push(_project);
      } catch (err) {
        this.hasError = err as string;
        await error(this.hasError);
      } finally {
        await debug('finish save Project');
        this.isLoading = false;
      }
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useProjectStore, import.meta.hot));
}
