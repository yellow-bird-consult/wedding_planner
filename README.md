# wedding_planner
This tool builds and runs dependencies for a Github repository. 

## Configuration
To declare you dependencies, you need 
to create a ```seating_plan.yml``` file in the root of your repository. 
This file should contain a list of dependencies, each with a name and a URL. 
The name is used to identify the dependency and the URL is used to clone the repository. 
The following is an example of a ```seating_plan.yml``` file:

```yaml
attendees:
  - name: institution
    url: https://github.com/yellow-bird-consult/institution.git
    branch: infrastructure

venue: ./sandbox/services/
```
This has one dependency, ```institution```, which is cloned from the ```infrastructure``` branch of 
the repository. The ```venue``` is the directory where the dependencies will be cloned to.

Each dependency needs to have a ```wedding_invite.yml``` file in the root of the repository which
contains the following:

```yaml
build_root: "."
runner_files:
  - runner_files/base.yml
  - runner_files/database.yml
build_files:
  x86_64: builds/Dockerfile.x86_64
  aarch64: builds/Dockerfile.aarch64
init_build:
  build_files:
    x86_64: database/builds/Dockerfile.x86_64
    aarch64: database/builds/Dockerfile.aarch64
  build_root: database
```
The wedding invite file has the following fields:

* ```build_root``` - The directory where the build will be run from.
* ```runner_files``` - A list of ```docker-compose``` files that will be used to run the dependency.
* ```build_files``` - A list of ```Dockerfile``` files that will be used to build the dependency
depending on the CPU that is running the program.
* ```init_build (optional)``` - A list of ```Dockerfile``` files that will be used to build the 
dependency's init build


## Usage
To run the program, you need to have ```docker``` and ```docker-compose``` installed. When we run the
program everything will run from the current working directory. The program will look for a 
```seating_plan.yml``` file in the current working directory. If you want to specifiy the path to the
```seating_plan.yml``` file, you can use the optional ```--f``` flag like the following command:

```bash
./wedp build -f /path/to/seating_plan.yml
```
The above command builds all the dependancies in the ```seating_plan.yml``` file. If you want to
run the dependencies, you can use the ```run``` command like the following:

```bash
./wedp run -f /path/to/seating_plan.yml
```

if you want to clone and install the dependencies, you can use the ```install``` command like the
following:

```bash
./wedp install -f /path/to/seating_plan.yml
```
Getting both outputs for both channels can be done with the following:

```bash
cargo run -- some test 1>out 2>err
```

If you want to setup the venue for the dependencies, you can use the ```setup``` command like the
following:

```bash
./wedp setup -f /path/to/seating_plan.yml
```

if you want to teardown the containers for the dependencies, you can use the ```teardown``` command
like the following:

```bash
./wedp teardown -f /path/to/seating_plan.yml
```

## Deploying a new release

Create the tag with the following:

```bash
git tag -a v0.0.6 -m "testing the deployment"
```

We then push the tag with the following:

```bash
git push origin v0.0.6 
```

This will then trigger the build and make a new release
