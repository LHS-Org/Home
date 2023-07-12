pipeline {
    agent any
    
    environment {
        IMAGE_NAME = "lhs-home/home"
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
                    
                        /kaniko/executor --dockerfile `pwd`/Dockerfile --context `pwd` --destination gitea.loeksangers.nl/${IMAGE_NAME}:latest
                    '''
                }
            }
        }
    }
}