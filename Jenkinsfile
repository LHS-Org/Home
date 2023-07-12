pipeline {
    agent any
    
    environment {
        IMAGE_NAME = "LHS-Home/Home"
    }

    stages {        
        // stage("Cleanup Workspace") {
        //     steps {
        //         cleanWs()
        //     }
        // }

        stage('Build & Push with Kaniko') {
            steps {
                container(name: 'kaniko', shell: '/busybox/sh') {
                    sh '''#!/busybox/sh
                        
                        cp /kaniko/.docker/.dockerconfigjson /kaniko/.docker/config.json
                        cat /kaniko/.docker/config.json
                        /kaniko/executor --dockerfile `pwd`/Dockerfile --context `pwd` --destination gitea.loeksangers.nl/${IMAGE_NAME}:latest
                    '''
                }
            }
        }
    }
}