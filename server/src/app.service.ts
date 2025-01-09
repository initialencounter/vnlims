import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Project, ProjectDataModel } from './project/project.entity';
import { readFileSync } from 'fs';
import { ProjectJson } from './types';
import { getJsonFilePaths } from './utils';
import { resolve } from 'path';
import { Like } from 'typeorm';

@Injectable()
export class AppService {
  constructor(
    @InjectRepository(Project)
    private projectsRepository: Repository<Project>,
  ) { }

  getHello(): string {
    return 'Hello World!';
  }

  async findAllProjects(): Promise<Project[]> {
    return this.projectsRepository.find();
  }

  async search(project: Project): Promise<Project[]> {
    const whereConditions: any = {};
    
    if (project.systemId) whereConditions.systemId = Like(`%${project.systemId}%`);
    if (project.appraiserName) whereConditions.appraiserName = Like(`%${project.appraiserName}%`);
    if (project.itemCName) whereConditions.itemCName = Like(`%${project.itemCName}%`);
    if (project.principalName) whereConditions.principalName = Like(`%${project.principalName}%`);
    if (project.projectNo) whereConditions.projectNo = Like(`%${project.projectNo}%`);
    if (project.mnotes) whereConditions.mnotes = Like(`%${project.mnotes}%`);
    if (project.tnotes) whereConditions.tnotes = Like(`%${project.tnotes}%`);

    let result = await this.projectsRepository.find({
      where: whereConditions,
    });
    return result;
  }

  async insert(project: Project): Promise<Project> {
    return this.projectsRepository.save(project);
  }

  async searchTNotes(tnotes: string): Promise<Project[]> {
    return this.projectsRepository.find({
      where: {
        tnotes: Like(`%${tnotes}%`),
      },
    });
  }

  async searchMNotes(mnotes: string): Promise<Project[]> {
    return this.projectsRepository.find({
      where: {
        mnotes: Like(`%${mnotes}%`),
      },
    });
  }

  async ImportProject(): Promise<void> {
    let dir = resolve(__dirname, '../..', 'data/ProjectData');
    let fileList = getJsonFilePaths(dir);
    for (const json of fileList) {
      const data: ProjectJson = JSON.parse(readFileSync(json, 'utf-8'));
      const projects = data.rows;
      for (const project of projects) {
        project['selfId'] = project.id;
        delete project.id;
        console.log(project.projectNo);
        await this.projectsRepository.save(project as unknown as Project);
      }
    }
  }
}
