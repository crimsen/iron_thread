import type { Project } from 'src/types/extendenProject';
import type { FieldConfigProject } from '../generic/genericType';
import type { Ref } from 'vue';
import { useProjectStore } from 'src/stores/projectStore';

// TODO: Delete eslint ignore
/* eslint-disable */

export const schema: FieldConfigProject[] = [
  // { key: 'fotoPath', label: 'Foto', type: 'fotoPath' },
  { key: 'name', label: 'Name', type: 'text' },
  { key: 'size', label: 'Size', type: 'number' },
  { key: 'fabricIds', label: 'Fabrics', type: 'ModifyFabrics' },
];

export const emptyProject: Project = {
  id: -1,
  name: '',
  size: null,
  fabricIds: [],
};

export const emitCancel = (
  modifyProject: Ref<Project | undefined> | undefined = undefined,
  showDialog: Ref<boolean> | undefined = undefined,
) => {
  if (modifyProject) modifyProject.value = undefined;
  if (showDialog) showDialog.value = false;
};

export const emitSave = async (
  value: Project,
  photo: File | undefined = undefined,
  showDialog: Ref<boolean> | undefined = undefined,
) => {
  const projectStore = useProjectStore();
  await projectStore.saveProject(value);
  if (showDialog) showDialog.value = false;
};

export const emitDelete = async (
  project: Project,
  showDialog: Ref<boolean> | undefined = undefined,
) => {
  const projectStore = useProjectStore();
  // await projectStore.deleteProject(project);
  if (showDialog) showDialog.value = false;
};
