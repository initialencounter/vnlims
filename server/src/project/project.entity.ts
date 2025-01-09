import { Entity, Column, PrimaryGeneratedColumn, PrimaryColumn } from 'typeorm';

export interface ProjectDataModel {
  id: string;
  appraiserName: string;
  assigneeName: string;
  auditorName: string;
  conclusions: number;
  displayStatus: string;
  itemCName: string;
  itemEName: string;
  mnotes: string;
  nextYear: boolean;
  principalName: string;
  projectId: string;
  projectNo: string;
  repeat: boolean;
  reportNo: string;
  reportType: number;
  submitDate: string;
  surveyorNames: null;
  systemId: string;
  tnotes: string;
}

@Entity()
export class Project {
  @PrimaryGeneratedColumn()
  id: number;

  @PrimaryColumn()
  selfId: string;

  @Column()
  appraiserName: string;

  @Column()
  assigneeName: string;

  @Column({ nullable: true })
  auditorName?: string;

  @Column({ nullable: true })
  conclusions?: number;

  @Column()
  displayStatus: string;

  @Column({ length: 1024 })
  itemCName: string;

  @Column({ length: 1024 })
  itemEName: string;

  @Column({ nullable: true, length: 1024 })
  mnotes?: string;

  @Column({ nullable: true })
  nextYear?: boolean;

  @Column({ nullable: true })
  principalName?: string;

  @Column()
  projectId: string;

  @Column({ nullable: true })
  projectNo?: string;

  @Column()
  repeat: boolean;

  @Column({ nullable: true, length: 1024 })
  reportNo?: string;

  @Column()
  reportType: number;

  @Column()
  submitDate: string;

  @Column({ nullable: true })
  surveyorNames?: string;

  @Column()
  systemId: string;

  @Column({ nullable: true, length: 1024 })
  tnotes?: string;
}
