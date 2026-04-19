import type { Fabric as OF } from './fabric';

export type ProjectForFabric = { project_id: number; length?: number };

export type Fabric = OF & {
  projects: Array<ProjectForFabric>;
  fileData?: string | null;
};
