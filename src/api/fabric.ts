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
  async saveFabricWithImage(fabric: Fabric, fabricImage: File): Promise<string | undefined> {
    // 1. Datei in Bytes umwandeln
    const arrayBuffer = await fabricImage.arrayBuffer();
    const bytes = new Uint8Array(arrayBuffer);

    try {
      // 2. An Rust Command senden
      const filePath: string = await invoke('upload_fabric_image', {
        fileName: fabricImage.name,
        fileData: Array.from(bytes), // Als Array für JSON-Serialisierung
        fabricId: fabric.id,
      });

      console.log('Bild unter diesem Pfad gespeichert:', filePath);
      return filePath;
    } catch (error) {
      console.error('Fehler beim Speichern:', error);
    }
  },
  async deleteFabric(fabric: Fabric): Promise<number | undefined> {
    try {
      const retVal: number = await invoke('delete_fabric', { fabricId: fabric.id });
      return retVal;
    } catch (e) {
      console.error('Fehler beim Löschen:', e);
    }
  },
};
