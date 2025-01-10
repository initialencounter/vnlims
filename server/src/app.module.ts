import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { Project } from './project/project.entity';
import { ConfigModule, ConfigService } from '@nestjs/config';

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true, // 使配置在全局可用
    }),
    TypeOrmModule.forRootAsync({
      imports: [ConfigModule], // 导入ConfigModule
      useFactory: (configService: ConfigService) => ({
        type: 'mysql',
        host: configService.get<string>('DB_HOST'), // 从配置中获取
        port: +configService.get<number>('DB_PORT'), // 从配置中获取
        username: configService.get<string>('DB_USERNAME'), // 从配置中获取
        password: configService.get<string>('DB_PASSWORD'), // 从配置中获取
        database: configService.get<string>('DB_DATABASE'), // 从配置中获取
        entities: [__dirname + '/**/*.entity{.ts,.js}'],
        synchronize: true, // 开发环境下使用，生产环境需要设置为false
        extra: {
          allowPublicKeyRetrieval: true,
        },
        ssl: false,
      }),
      inject: [ConfigService], // 注入ConfigService
    }),
    TypeOrmModule.forFeature([Project]),
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule { }
