docker_build('frontend', '.', dockerfile="k8s/Dockerfile")
k8s_yaml('k8s/app.yaml')
k8s_resource('kindle-highlights', port_forwards=8001)
