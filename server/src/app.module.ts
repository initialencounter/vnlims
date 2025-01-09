import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { Project } from './project/project.entity';

@Module({
  imports: [
    TypeOrmModule.forRoot({
      type: 'mysql',
      host: 'localhost',
      port: 3306,
      username: 'root',
      password: '*',
      database: '*',
      entities: [__dirname + '/**/*.entity{.ts,.js}'],
      synchronize: true, // 开发环境下使用，生产环境需要设置为false
      extra: {
        allowPublicKeyRetrieval: true
      },
      ssl: false
    }),
    TypeOrmModule.forFeature([Project]),
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
