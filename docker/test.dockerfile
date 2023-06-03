FROM ubuntu:22.04

ADD /scripts/install.sh /install.sh
ADD /docker/entrypoint.sh /entrypoint.sh

RUN chmod +x /install.sh && \
  env ENVFILE=/env.sh /install.sh && \
  rm -f /install.sh && \
  mkdir -p /project && \
  apt-get clean autoclean && \
  apt-get autoremove --yes && \
  rm -rf /var/cache/apt/archives && \
  rm -rf /var/lib/apt/lists/* && \
  chmod +x /entrypoint.sh

WORKDIR /project
ENTRYPOINT ["/entrypoint.sh"]
CMD ["/bin/bash"]
