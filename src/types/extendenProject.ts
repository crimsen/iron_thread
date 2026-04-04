import type { Project as OP } from './project';

export type Project = OP & {
  fabricIds: Array<number | null>;
};
