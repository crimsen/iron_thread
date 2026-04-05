import type { Project as OP } from './project';

export type Project = OP & {
  fabrics: Array<{ fabric_id: number; length: number } | null>;
};
