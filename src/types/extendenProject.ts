import type { Project as OP } from './project';

export type FabricForProject = { fabric_id: number; length?: number };

export type Project = OP & {
  fabrics: Array<FabricForProject>;
};
