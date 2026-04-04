import type { Project } from 'src/types/project';
import type { FieldConfigProject } from '../generic/genericType';
import type { Ref } from 'vue';
import { useProjectStore } from 'src/stores/projectStore';

// TODO: Delete eslint ignore
/* eslint-disable */

export const schema: FieldConfigProject[] = [
  // { key: 'fotoPath', label: 'Foto', type: 'fotoPath' },
  { key: 'name', label: 'Name', type: 'text' },
  { key: 'size', label: 'Size', type: 'number' },
  // { key: 'length', label: 'Length', type: 'number' },
  // { key: 'width', label: 'Width', type: 'number' },
  // { key: 'kindOfProjectId', label: 'Kind of Project', type: 'KindOfProjectSelect' },
  // { key: 'costs', label: 'Costs', type: 'number' },
  // { key: 'producer', label: 'Producer', type: 'text' },
  // { key: 'dateOfPurchase', label: 'Date of purchase', type: 'date' },
];

export const emptyProject: Project = {
  id: -1,
  name: '',
  size: null,
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
