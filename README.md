# Remove docker images

Remove obsolete docker images.

## Prerequisite

Install Docker.

## Supported tag syntax

The tags following this regex are supported:

    ^[vV]?(\d{1,5})(\.\d{1,5})?(\.\d{1,5})?(-.*)?$

ex) Supported tags:

    1.0
    1.0-SNAPSHOT
    v2.10.1
    2.0-BR291-SNAPSHOT
    8-jdk

ex) Non supported tag:

    123456.1 (major version has more than 5 digits)
    ver1.2

## Canonical version

If the tag matches the following regex, it is recognized as a canonical (non snapshot) version.

    ^[vV]?(\d{1,5})(\.\d{1,5})?(\.\d{1,5})?

## Snapshot version

If the tag ends with "-SNAPSHOT", it is recognized as a snapshot version.

## Branch

If non "-SNAPSHOT" string follows, it is recognized as a branch.

ex) Branch: BR102(canonical version)

    1.4-BR102

ex) Branch: BR102(snapshot version)

    1.4-BR102-SNAPSHOT

## How it works

1. Invoke "docker images" to obtain the list of docker image.

| Image name | tag |
-|-
| foo/myapp | 1.0 |
| foo/myapp | 1.0-SNAPSHOT |
| bar/app | 1.2 |
| foo/myapp | 1.1 |
| foo/myapp | 1.1-SNAPSHOT |
| foo/myapp | 1.2 |
| bar/app | 2.2 |
| foo/myapp | 1.3 |

2. Grouping the list by image name.

| Image name | tag |
-|-
| foo/myapp | 1.0 |
| foo/myapp | 1.0-SNAPSHOT |
| foo/myapp | 1.3 |
| foo/myapp | 1.1 |
| foo/myapp | 1.1-SNAPSHOT |
| foo/myapp | 1.2 |
| bar/app | 2.2 |
| bar/app | 1.2 |

3. Grouping the list by canonical/snapshot version.

| Image name | tag |
-|-
| foo/myapp | 1.0-SNAPSHOT |
| foo/myapp | 1.1-SNAPSHOT |
| foo/myapp | 1.0 |
| foo/myapp | 1.3 |
| foo/myapp | 1.1 |
| foo/myapp | 1.2 |
| bar/app | 2.2 |
| bar/app | 1.2 |

4. Sort by version number.

| Image name | tag |
-|-
| foo/myapp | 1.0-SNAPSHOT |
| foo/myapp | 1.1-SNAPSHOT |
| foo/myapp | 1.0 |
| foo/myapp | 1.1 |
| foo/myapp | 1.2 |
| foo/myapp | 1.3 |
| bar/app | 1.2 |
| bar/app | 2.2 |

5. Apply the following rule.<br/>
For snapshot version, keep the latest version. For canonical version, keep the three newest versions. You can modify the keep count by argument.

| Image name | tag | delete |
-|-|-
| foo/myapp | 1.0-SNAPSHOT | delete |
| foo/myapp | 1.1-SNAPSHOT | keep |
| foo/myapp | 1.0 | delete |
| foo/myapp | 1.1 | keep |
| foo/myapp | 1.2 | keep |
| foo/myapp | 1.3 | keep |
| bar/app | 1.2 | keep |
| bar/app | 2.2 | keep |

6. Invoke "docker rmi" to remove image

## Argument

- --version<br/>
Show tool version.

- --dry-run<br/>
Do not invoke "docker rmi" instead, just show the images that will be deleted.

- --keep [count]<br/>
Specify keep count for canonical versions. Default to 3.

- --keep-snapshot [count]<br/>
Specify keep count for snapshot versions. Default to 1.

- --repository-type [type]<br/>
Specify docker image repository type. Can use 'docker' and 'ibmcr'. The 'docker' means docker image repository. The 'ibmcr' means IBM Cloud Registry. Default to 'docker'.

## IBM Cloud Registry

If you want to use IBM Cloud Registry, specify '--repository-type ibmcr'. In this case, you need to login IBM Cloud Registry in advance.

    $ ibmcloud login
    $ ibmcloud cr login
    $ remove_docker_images --repository-type ibmcr

If you want to execute it in non-interactive environment such as in CI server, you can use API key.

    $ export IBMCLOUD_API_KEY=YOUR_CONTAINER_REGISTRY_APIKEY
    $ ibmcloud login
    $ ibmcloud cr login
    $ remove_docker_images --repository-type ibmcr

You can create API key at web console:

- Manage => IAM => Service IDs => Create
- Input name and click Create.
- Click Access Policies.
- Click Assign access.
- Select IAM services.
- Click drop down 'No access' and select Container Registry.
- In 'Service access', check Reader and Writer.
- Click Add.
- Click Assign.
- Click API keys.
- Click Create.
- Input Name and click Create.
- Now, you can copy the API Key.

## Binary

Linux(x86_64):
[0.2.0](http://static.ruimo.com/release/remove_docker_images/0.2.0/remove_docker_images)

