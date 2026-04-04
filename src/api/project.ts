import { invoke } from '@tauri-apps/api/core';
import type { Project } from 'src/types/project.ts';
export const projectApi = {
  async getAll(): Promise<Project[]> {
    return await invoke<Project[]>('get_projects');
  },
  async save(project: Project): Promise<Project> {
    if (typeof project.id != 'number') project.id = -1;
    return await invoke<Project>('save_project', { projectData: project });
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
