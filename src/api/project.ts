import { invoke } from '@tauri-apps/api/core';
import { useFabricXProject } from 'src/stores/fabricXProject';
import type { Project } from 'src/types/extendenProject';
import type { FabricXProject } from 'src/types/fabricXProject';

export const projectApi = {
  async getAll(): Promise<Project[]> {
    return await invoke<Project[]>('get_projects');
  },
  async save(project: Project): Promise<Project> {
    if (typeof project.id != 'number') project.id = -1;
    const _project = await invoke<Project>('save_project', { projectData: project });
    const fabricXProjectStore = useFabricXProject();
    await Promise.all(
      project.fabricIds.map(async (p) => {
        const fabricXProject: FabricXProject = {
          fabricId: <number>p,
          projectId: _project.id,
          fabricLength: null,
        };
        return fabricXProjectStore.saveFabricXProject(fabricXProject);
      }),
    );
    return _project;
  },
  async saveProjectWithImage(project: Project, projectImage: File): Promise<string | undefined> {
    // 1. Datei in Bytes umwandeln
    const arrayBuffer = await projectImage.arrayBuffer();
    const bytes = new Uint8Array(arrayBuffer);

    try {
      // 2. An Rust Command senden
      const filePath: string = await invoke('upload_project_image', {
        fileName: projectImage.name,
        fileData: Array.from(bytes), // Als Array für JSON-Serialisierung
        projectId: project.id,
      });

      console.log('Bild unter diesem Pfad gespeichert:', filePath);
      return filePath;
    } catch (error) {
      console.error('Fehler beim Speichern:', error);
    }
  },
  async deleteProject(project: Project): Promise<number | undefined> {
    try {
      const retVal: number = await invoke('delete_project', { projectId: project.id });
      return retVal;
    } catch (e) {
      console.error('Fehler beim Löschen:', e);
    }
  },
};
