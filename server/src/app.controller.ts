import { Controller, Get, Post, Query } from '@nestjs/common';
import { AppService } from './app.service';
import { Project, ProjectDataModel } from './project/project.entity';

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
  searchTNotes(@Query('tNotes') tNotes: string): Promise<Project[]> {
    return this.appService.searchTNotes(tNotes, 10);
  }

  @Get('searchMNotes')
  searchMNotes(@Query('mNotes') mNotes: string): Promise<Project[]> {
    return this.appService.searchMNotes(mNotes, 10);
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
}
