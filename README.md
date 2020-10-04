# Building
It should be possible to build and deploy this project to AWS using
a couple of handy (even on Windows!).

#### Build `aws-serverless-docker` image
```shell script
docker build -t aws-serverless-docker aws-serverless-docker
```

#### Run it with fresh workspace
```shell script
docker run -it --name lambda-git -v lambda-git-workspace:/workspace -v /var/run/docker.sock:/var/run/docker.sock aws-serverless-docker
```

#### Configure AWS
```shell script
aws configure
```

#### Clone project
```shell script
cd /workspace && git clone https://github.com/tocisz/lambda-git.git .
```
(alternatively, use a locally hosted repo)

#### Init serverless
```shell script
npm i
```

#### Install to AWS
```shell script
npx serverless deploy
```
This step uses modified version of `sererless-rust` plugin (see `package.json`).

The following configuration in `sererless.yml` file tells serverless to use
docker volume as sources:
```yaml
custom:
  rust:
    volumePath: lambda-git-workspace
```