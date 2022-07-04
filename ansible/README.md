# Mangata
This ansible role will install the latest Docker version, Docker Compose and setup docker users, deploy Mangata nodes on Ubuntu.<br/>
<br/>

## Requirements<br/>
infrastructure deployment via terraform<br/>
ansible installation<br/>
<br/>

## Role Variables
<br/>
Available variables are listed below, along with variables <br/>
ansible/vars/main.yml<br/>
<br/>
<pre>
docker_users: [ubuntu]
mangata_user: "ubuntu"
mangata_directory_name: "devops"
mangata_env: "second-interview"
docker_daemon_options: {}
deployment_path: /home/{{ mangata_user }}/{{ mangata_directory_name }}
mangata_image_version: rococo-latest
stackcomponents:
  - { name: 'second-interview', path: '{{ deployment_path }}/parachain/env/' }
</pre>
<br/>

## Ansible installation
ansible is installed with the help of Python Package Installer PIP in the workflow<br/>
<br/>

## Configmanagement rollout
<pre>
ansible-playbook -i inventories/inventory.yml --extra-vars="ansible_ssh_private_key_file=${PRIVATE_KEY_PATH} playbook.yml<br/>
</pre><br/>
The playbook is as shown below
<br/>
<pre>
---
- hosts: all 
  vars_files:
    - "vars/main.yml"
    - "defaults/main.yml"
  become: true
  gather_facts: true
  roles:
    - docker
    - mangata
</pre>
<br/>

## License
This project is licensed under the GPLv3 License - see the [LICENSE](LICENSE.md) file for details. For more informations: [License Article]

## MIT / BSD
