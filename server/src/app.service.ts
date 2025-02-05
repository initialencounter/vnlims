import { Injectable, Logger } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Project, ProjectDataModel } from './project/project.entity';
import { makeQueryString, makeRequest, sleep } from './utils';
import { Like } from 'typeorm';
import { ConfigService } from '@nestjs/config';
import * as fs from 'fs';
import * as path from 'path';

@Injectable()
export class AppService {
  readonly logger = new Logger(AppService.name);
  private lastUpdatedFilePath = path.join('lastUpdated.txt'); // 文件路径
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
    this.updateLastUpdated(); // 在插入时更新最后更新时间
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

  async searchItemCName(itemCName: string, rows: number): Promise<Project[]> {
    return this.projectsRepository.find({
      where: {
        itemCName: Like(`%${itemCName}%`),
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
    // if (projects.length > 0) {
    //   this.logger.debug(`${date} 已发现 ${projects.length} 条数据，明天再更新吧。`);
    //   return `${date} 已发现 ${projects.length} 条数据，明天再更新吧。`;
    // }
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
    this.updateLastUpdated(); // 在导入完成后更新最后更新时间
    return `成功导入 ${projectsToSave.length} 条数据`;
  }

  // 更新最后更新时间并持久化到文件
  updateLastUpdated(): void {
    const currentTime = new Date();
    fs.writeFileSync(this.lastUpdatedFilePath, currentTime.toString()); // 写入文件
    this.logger.log(`最后更新时间: ${currentTime}`); // 记录日志
  }

  // 从文件中获取最后更新时间
  getLastUpdated(): string {
    if (fs.existsSync(this.lastUpdatedFilePath)) {
      const lastUpdated = fs.readFileSync(this.lastUpdatedFilePath, 'utf-8');
      return lastUpdated; // 返回最后更新时间
    }
    return 'null'; // 如果文件不存在，返回 null
  }
}
