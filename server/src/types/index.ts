import { ProjectDataModel } from "src/project/project.entity";

export interface ProjectJson {
  total: number;
  rows: ProjectDataModel[];
}