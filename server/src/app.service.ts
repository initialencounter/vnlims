import { Injectable, Logger } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Project, ProjectDataModel } from './project/project.entity';
import { makeQueryString, makeRequest, sleep } from './utils';
import { Like } from 'typeorm';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class AppService {
  private readonly logger = new Logger(AppService.name);
  constructor(
    @InjectRepository(Project)
    private projectsRepository: Repository<Project>,
    private configService: ConfigService,
  ) { }

  getHello(): string {
    return 'Hello World!';
  }

  async findAllProjects(): Promise<Project[]> {
    return this.projectsRepository.find();
  }

  async search(project: Project, rows: number): Promise<Project[]> {
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
      take: rows,
    });
    return result;
  }

  async insert(project: Project): Promise<Project> {
    return this.projectsRepository.save(project);
  }

  async searchTNotes(tnotes: string, rows: number): Promise<Project[]> {
    return this.projectsRepository.find({
      where: {
        tnotes: Like(`%${tnotes}%`),
      }, take: rows,
    });
  }

  async searchMNotes(mnotes: string, rows: number): Promise<Project[]> {
    return this.projectsRepository.find({
      where: {
        mnotes: Like(`%${mnotes}%`),
      },
      take: rows,
    });
  }

  async ImportProject(sessionId: string, userName: string, date: string): Promise<string> {
    if (!date) {
      date = new Date().toISOString().split('T')[0];
    }
    if (!sessionId || !userName) {
      this.logger.error('sessionId 或 userName 不能为空');
      return 'sessionId 或 userName 不能为空';
    }
    let projectNo = 'PEKGZ' + date.replace(/-/g, '');
    let projects = await this.projectsRepository.find({
      where: {
        projectNo: Like(`%${projectNo}%`),
      },
    });
    if (projects.length > 0) {
      this.logger.debug(`${date} 已发现 ${projects.length} 条数据，明天再更新吧。`);
      return `${date} 已发现 ${projects.length} 条数据，明天再更新吧。`;
    }
    const systemIdList = ['aek', 'pek', 'sek', 'rek'];
    const projectsToSave: Project[] = [];
    for (const systemId of systemIdList) {
      let queryString = makeQueryString(date, systemId);
      let projects: ProjectDataModel[] = await makeRequest(queryString, this.configService.get('BASE_URL'), sessionId, userName);
      this.logger.log(`${date} ${systemId} 已发现 ${projects.length} 条数据`);
      for (const project of projects) {
        project['selfId'] = project.id;
        delete project.id;
        projectsToSave.push(project as unknown as Project);
      }
      await sleep(5000);
    }
    await this.projectsRepository.save(projectsToSave);
    return `成功导入 ${projectsToSave.length} 条数据`;
  }
}
