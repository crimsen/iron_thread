import { acceptHMRUpdate, defineStore } from 'pinia';
import { projectApi } from 'src/api/project';
import type { Project } from 'src/types/project';
import { debug, error } from '@tauri-apps/plugin-log';

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
    },
    async saveProject(project: Project) {
      this.isLoading = true;
      await debug(`save Project ${JSON.stringify(project)}`);
      try {
        await debug('wait to save Project');
        this.projects.push(await projectApi.save(project));
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
