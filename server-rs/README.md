# 启动

在项目根目录创建.env 文件，写入以下内容

```ini
DATABASE_URL=sqlite://project.sqlite?mode=rwc
SERVER_URL=0.0.0.0:4000
LIMS_BASE_URL=LIMS域名 https://lims.xxx.com.cn
LIMS_USER_NAME=Lims账号
LIMS_PASS_WORD=Lims密码
```

cargo run --release 启动
