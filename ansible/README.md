# docker
This ansible role will install the latest Docker version, Docker Compose and setup docker users on Centos, Redhat and Fedora.<br/>
<br/>
Requirements<br/>
ansible installation<br/>
<br/>
Role Variables
<br/>
Available variables are listed below, along with variables <br/>
vars/main.yml<br/>
<br/>
### Edition can be one of: 'ce' (Community Edition) or 'ee' (Enterprise Edition).<br/>
<pre>
docker_edition: 'ce'
docker_package: "docker-{{ docker_edition }}"
docker_package_state: present
</pre>
<br/>
The docker_edition should be either ce (Community Edition) or ee (Enterprise Edition). <br/>
<br/>
<pre>
docker_service_state: started
docker_service_enabled: true
docker_restart_handler_state: restarted
</pre>
<br/>
Variables to control the state of the docker service, and whether it should start on boot. If you're installing Docker inside a Docker container without systemd or sysvinit, you should set these to stopped and set the enabled variable to no.<br/>
<pre>
docker_install_compose: false
docker_compose_version: "1.22.0"
docker_compose_path: /usr/local/bin/docker-compose
</pre>
<br/>
Docker Compose installation options.<br/>
(Used only for RedHat/CentOS.) You can enable the Edge or Test repo by setting the respective vars to 1.<br/><br/>
<pre>
docker_yum_repo_url: https://download.docker.com/linux/{{ (ansible_distribution == "Fedora") | ternary("fedora","centos") }}/docker-{{ docker_edition }}.repo
docker_yum_repo_enable_edge: 0
docker_yum_repo_enable_test: 0
</pre>
<br/>
A list of system users to be added to the docker group (so they can use Docker on the server).<br/>
<pre>
docker_users:
  - user1
  - user2
</pre>
Dependencies<br/>
Example Playbook : ~/ansible/playbooks/test.yml<br/>
<pre>
---
- hosts: all
  become: true 
  roles:
    - docker
</pre>
Testing the role with vagrant<br/>
Dependencies: Vagrant must be installed<br/>
<pre>
vagrant up
</pre>
<br/>
License<br/>
Example Playbook : ~/ansible/playbooks/test.yml<br/>
<br/>
MIT / BSD
