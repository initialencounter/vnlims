import { Controller, Get, Post, Query, Req } from '@nestjs/common';
import { AppService } from './app.service';
import { Project, ProjectDataModel } from './project/project.entity';
import { Request } from 'express';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) { }

  @Get()
  getHello(): string {
    return this.appService.getHello();
  }

  @Get('search')
  search(
    @Query('systemId') systemId: string = '',
    @Query('appraiserName') appraiserName: string = '',
    @Query('itemName') itemName: string = '',
    @Query('principal') principal: string = '',
    @Query('projectNo') projectNo: string = '',
    @Query('page') page: number = 1,
    @Query('rows') rows: number = 10,
    @Query('mnotes') mnotes: string = '',
    @Query('tnotes') tnotes: string = '',
  ): Promise<Project[]> {
    let project = new Project();
    project.systemId = systemId;
    project.appraiserName = appraiserName;
    project.itemCName = itemName;
    project.principalName = principal;
    project.projectNo = projectNo;
    project.mnotes = mnotes;
    project.tnotes = tnotes;
    return this.appService.search(project, rows);
  }

  @Get('searchTNotes')
  searchTNotes(@Query('tNotes') tNotes: string, @Req() req: Request): Promise<Project[]> {
    let ipAddress = req.ip;
    if (req.headers['x - forwarded - for']) {
      ipAddress = req.headers['x - forwarded - for'].toString().split(',')[0].trim();
    }
    this.appService.logger.debug(`searchTNotes${ipAddress}：${tNotes}`);
    return this.appService.searchTNotes(tNotes, 100);
  }

  @Get('searchMNotes')
  searchMNotes(@Query('mNotes') mNotes: string, @Req() req: Request): Promise<Project[]> {
    let ipAddress = req.ip;
    if (req.headers['x - forwarded - for']) {
      ipAddress = req.headers['x - forwarded - for'].toString().split(',')[0].trim();
    }
    this.appService.logger.debug(`searchTNotes${ipAddress}：${mNotes}`);
    return this.appService.searchMNotes(mNotes, 100);
  }

  @Get('searchItemCName')
  searchItemCName(@Query('itemCName') itemCName: string): Promise<Project[]> {
    return this.appService.searchItemCName(itemCName, 100);
  }

  @Post('insert')
  insert(project: Project) {
    return this.appService.insert(project);
  }

  @Get('import')
  import(
    @Query('sessionId') sessionId: string,
    @Query('userName') userName: string,
    @Query('date') date: string,
  ) {
    return this.appService.ImportProject(sessionId, userName, date);
  }

  @Get('getLastUpdated')
  getLastUpdated() {
    return this.appService.getLastUpdated();
  }
}
