import type { Fabric } from 'src/types/fabric';
import type { FieldConfigFabric } from '../generic/genericType';
import type { Ref } from 'vue';
import { useFabricStore } from 'src/stores/fabricStore';

export const schema: FieldConfigFabric[] = [
  { key: 'fotoPath', label: 'Foto', type: 'fotoPath' },
  { key: 'name', label: 'Name', type: 'text' },
  { key: 'length', label: 'Length', type: 'number' },
  { key: 'width', label: 'Width', type: 'number' },
  { key: 'kindOfFabricId', label: 'Kind of Fabric', type: 'KindOfFabricSelect' },
  { key: 'costs', label: 'Costs', type: 'number' },
  { key: 'producer', label: 'Producer', type: 'text' },
  { key: 'dateOfPurchase', label: 'Date of purchase', type: 'date' },
];

export const emptyFabric: Fabric = {
  id: -1,
  name: null,
  length: null,
  width: null,
  costs: null,
  fotoPath: null,
  producer: null,
  kindOfFabricId: null,
  dateOfPurchase: null,
};

export const emitCancel = (
  modifyFabric: Ref<Fabric | undefined> | undefined = undefined,
  showDialog: Ref<boolean> | undefined = undefined,
) => {
  if (modifyFabric) modifyFabric.value = undefined;
  if (showDialog) showDialog.value = false;
};

export const emitSave = async (
  value: Fabric,
  photo: File | undefined = undefined,
  showDialog: Ref<boolean> | undefined = undefined,
) => {
  const fabricStore = useFabricStore();
  await fabricStore.saveFabric(value, photo);
  if (showDialog) showDialog.value = false;
};
