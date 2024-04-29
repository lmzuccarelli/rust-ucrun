# the FROM instruction will not be parsed
FROM alpine

COPY rootfs/image /image

LABEL org.opencontainers.image.maintainer="luzuccar@redhat.com"  
LABEL org.opencontainers.image.authors="luzuccar@redhat.com"        
LABEL org.opencontainers.image.title="nanovm-unikernel"                           
LABEL org.opencontainers.image.description="ops nanovm container for Kubernetes/OpenShift" 
LABEL org.opencontainers.image.version="release"
LABEL org.opencontainers.image.base.name="redis-server" 

LABEL com.ucrun.unikernel.binary="/image"
LABEL com.ucrun.unikernel.type="ops-nanovm"
LABEL com.ucrun.unikernel.hypervisor="qemu"

ENTRYPOINT ["/bin/sh","-c","while true; do sleep 60; done"]
