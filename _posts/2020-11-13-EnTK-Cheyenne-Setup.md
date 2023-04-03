---
layout: post
title: "HPC with RADICAL EnTK on NCAR Cheyenne"
description: "Ensemble toolkit for HPC workflows"
tags: Python HPC
giscus_comments: true
date: 2020-11-13T18:00:00
related_posts: false
---

- [Introduction](#introduction)
- [Setting Up](#setting-up)
  - [Step 4: Customization](#step-4-customization)
- [Debugging](#debugging)
  - [Where To Look](#where-to-look)
  - [Problem 1](#problem-1)
  - [Problem 2](#problem-2)
- [Final Remark](#final-remark)

## Introduction

```latex
tl;dr

Still using bash scripts to run jobs on HPC?
Too many bash script for your jobs on HPC?
Too many jobs to submit and the queue time is too long?

Use EnTK !!!
```

[Ensemble Toolkit](https://radicalentk.readthedocs.io/en/latest/index.html) (EnTK) is a Python framework for developing and executing applications comprised of multiple sets of tasks, a.k.a. ensembles, on High-Performance Computing (HPC) platforms. It is developed by the [RADICAL](http://radical.rutgers.edu/) group from Rutgers University.

Usually, running ensemble simulations on HPC platforms is done with some kind of scripting language like `bash`. It often involves a main script that does the actual computation and some helper scripts to submit jobs in batches. **The [queue time](https://link-springer-com.ezaccess.libraries.psu.edu/referenceworkentry/10.1007%2F1-4020-0612-8_789) can be a significant bottleneck** if you decide to submit hundreds or even thousands of tasks. If you have multiple stages in your simulation, for example, you need to first generate wind speed forecasts and then generate wind power simulation, you will need to **manually manage all the stages and the intermediate data**. This is not a flexible and scalable solution.

EnTK aims to address these issues and provide flexibility and scalability. It provides the following features:

- EnTK provides a framework to help you define your ensemble simulations in pipelines, stages, and tasks.
- Submit only one job for your entire ensemble simulation workflow.
- Allocation for each task is dynamically managed by EnTK. No more I-wish-I-had-five-more-minute moments.
- Low overhead

Think of it as a steward for your computational resources. Previously, you need to write specific script to handle simulations and job submissions. Now, you only ask once for the computational resources and EnTK manages it for you. You then tell EnTK how many cores to allocate for each task without specifying the acutal time for each task. That is one extra level of abstraction in the workflow.

This post is not to show tutorials on EnTK. Please refer to the [documentation](https://radicalentk.readthedocs.io/en/latest/index.html) for that matter. This post is dedicated to setting up an EnTK environment on the NCAR [Cheyenne](https://www2.cisl.ucar.edu/resources/computational-systems/cheyenne) supercomputer. And I personally use these instructions for my research projects on [weather analogs](https://weiming-hu.github.io/AnalogsEnsemble/) and [photovoltaic solar energy simulation](https://github.com/Weiming-Hu/RenewableSimulator). 

Please refer to the [allocation page](https://www2.cisl.ucar.edu/user-support/allocations/university-allocations) if you are interested in getting your allocation and start your research on Cheyenne at scale!

## Setting Up

Setting up an environemnt for EnTK on Cheyenne involves the following steps:

1. Create a virtual environment for head nodes
2. Install EnTK
3. Prepare a second virtual environment for compute nodes
4. Customize configuration

The following script shows a step-by-step guide to setting up EnTK on Cheyenne. It is assumed that you are already logged onto a head node of Cheyenne.

```bash
##########
# Step 1 #
##########

# Load modules
#
# Avoid using Intel compilers due to possible linking errors. Use GNU compilers instead.
#
module purge
module load ncarenv gnu mpt netcdf cmake ncarcompilers python git

# Prepare virtual environment
virtualenv -p python3.7 ~/venv

# Activate the environment
source ~/venv/bin/activate

##########
# Step 2 #
##########

# Instal EnTK
#
# Add --no-cache-dir if you have disabled your pip cache
#
pip install radical.entk

##########
# Step 3 #
##########

# Prepare a second virtual environment for compute nodes
# -d installs default modules.
#
radical-pilot-create-static-ve -p ~/ve.pilot -d
```

> As on 2021/2/1, I received the error, `ERROR: pip's dependency resolver does not currently take into account all the packages that are installed. This behaviour is the source of the following dependency conflicts.
requests 2.25.1 requires idna<3,>=2.5, but you have idna 3.1 which is incompatible.`, while creating the static virtual environment. As far as I'm concerned, this won't affect the process.

At this point, we have prepared all the environments needed by EnTK. Step 3 is optional but I highly recommend doing it on Cheyenne. EnTK can automatically create the environment for compute nodes while the job the running but this will take time from your acutal computation. In reality, the Internet connection on Cheyenne compute nodes is very slow. Setting up the environment effectively can take half an hour or even longer. So to save time, we prepare the environment ahead of time for EnTK.

### Step 4: Customization

**Prepare the computing environment ahead of time**. We need to tell EnTK to use the virtual environment prepared, instead of creating a new one on compute nodes. This can be done by creating a file at `~/.radical/pilot/configs/resource_ncar.json`, and write the following content.

```
{
    "cheyenne_mpt": {
        "virtenv" : "/glade/u/home/wuh20/ve.pilot"
    },
    
    "cheyenne": {
        "virtenv" : "/glade/u/home/wuh20/ve.pilot"
    }
}
```

Note that we have changed the `virtenv` field for two resource labels, `cheyenne_mpt` and `cheyenne`. We need to use these resource labels when running simulations on Cheyenne. For example, my resource configuration looks like the following.

```yaml
resource-desc:
  name: 'ncar.cheyenne_mpt'
  walltime: 300
  cpus: 72
  gpus: 0
  queue: 'regular'
  project: '<Your project account>'
  schema: 'local'
```

**Set up a message database**. To instantiate an `AppManager`, it needs RabbitMQ for message transfer. Please refer to the [documentation](https://radicalentk.readthedocs.io/en/latest/install.html#rabbitmq) for options you have to set up RabbitMQ. What is an `AppManager`, it is explained [here](https://radicalentk.readthedocs.io/en/latest/user_guide/get_started.html#creating-the-appmanager) on what it is and how to create one. But the different is, the documentation creates an `AppManager` to run locally, but we are running on supercomputers. After setting up RabbitMQ, you will have your own set of configurations for `hostname`, `port`, `username`, and `password`.

**Set up a temporary folder for logs**. Point the temporary file directory to scratch folder is also a good practice because EnTK with a verbose level will create a lot of messages. For me, I have the following setting in my `~/.bash_profile`.

```bash
export TMPDIR=/glade/scratch/<your NCAR account>/tmp
```

**Trun off heartbeat**. EnTK uses `pika` to maintain connections. However, session might be terminated if they become too long. I added the following customization into my main python script to specifically turn off heartbeat so that servers will not close any connections unless the client does so.

```python
import pika

# Make sure server does not close any connection unless the client does so
pika.connection.Parameters.DEFAULT_HEARTBEAT_INTERVAL = 0
pika.connection.Parameters.DEFAULT_HEARTBEAT_TIMEOUT = 0
```

Finally, I usually set the following in my main python script to enable verbose output and to log profiling information.

```python
# Enable profiling information
os.environ['RADICAL_PROFILE'] = 'True'

# Set output verbosity
os.environ['RADICAL_LOG_LVL'] = 'DEBUG'
```

## Debugging

### Where To Look

Errors are generated from two sources, the client side errors and the server side errrors.

The client side errors can be found where you run your main python script. It looks similar to `re.session.cheyenne1.wuh20.018579.0020` and `018579` is the job ID.

The server side errors are stored under `$TMPDIR/radical.pilot.sandbox`. You can match names of the client side and server side folders.

I usually use the following commands to look for possible error messages. Sometimes, you need to sift through the results just to make sure the matches are actual error messages, rather than source code.

```
grep -R Error *.log
```

### Problem 1

The program is stuck with the following output for more than half an hour.

```bash
EnTK session: re.session.cheyenne1.wuh20.018579.0019
Creating AppManagerSetting up RabbitMQ system                                 ok
                                                                              ok
Validating and assigning resource manager                                     ok
Setting up RabbitMQ system                                                   n/a
new session: [re.session.cheyenne1.wuh20.018579.0019]                          \
database   : [mongodb://hpcw-psu:****@129.114.17.185:27017/hpcw-psu]          ok
create pilot manager                                                          ok
submit 1 pilot(s)
        pilot.0000   ncar.cheyenne            72 cores       0 gpus           ok
```

This could be caused by EnTK spending a long time on preparing the virtual environment directly on compute nodes. Compute nodes have slow internet connection.

Make sure you have created a configuration file as in step 4. And also make sure that the resource label you are using is consistent with the one present in the configuration file you have created.

### Problem 2

The program returns similar errors to the following one before the successful creation of a pilot manager.

```python
Exception in thread Thread-12:
Traceback (most recent call last):
  File "/glade/u/apps/ch/opt/python/3.7.5/gnu/8.3.0/lib/python3.7/threading.py", line 926, in _bootstrap_inner
    self.run()
  File "/glade/u/apps/ch/opt/python/3.7.5/gnu/8.3.0/lib/python3.7/threading.py", line 870, in run
    self._target(*self._args, **self._kwargs)
  File "/glade/u/home/wuh20/venv/lib/python3.7/site-packages/radical/entk/execman/rp/task_manager.py", line 280, in _process_tasks
    umgr = rp.UnitManager(session=rmgr._session)
  File "/glade/u/home/wuh20/venv/lib/python3.7/site-packages/radical/pilot/unit_manager.py", line 136, in __init__
    self._cmgr.start_components()
  File "/glade/u/home/wuh20/venv/lib/python3.7/site-packages/radical/pilot/utils/component.py", line 231, in start_components
    raise RuntimeError('bridge startup failed')
RuntimeError: bridge startup failed
```

This is probably caused by left over processes. We need to first confirm this by looking for those processes and then terminate them manually.

```bash
wuh20@cheyenne4:~/github/pv-workflow/02_WeightOptimization> ps -u wuh20 | grep -e python -e rp
  519 ?        09:36:10 rp.pmgr_launchi
21919 ?        00:00:07 rp.pmgr_launchi
27693 ?        10:14:41 rp.pmgr_launchi
30741 ?        10:13:24 rp.pmgr_launchi
36305 ?        10:16:11 rp.pmgr_launchi
46109 ?        09:18:37 rp.pmgr_launchi
49351 ?        10:24:34 rp.pmgr_launchi
58516 ?        10:23:10 rp.pmgr_launchi
58906 ?        09:38:10 rp.pmgr_launchi
72261 ?        00:00:06 rp.pmgr_launchi
72596 ?        10:16:12 rp.pmgr_launchi
wuh20@cheyenne4:~/github/pv-workflow/02_WeightOptimization> pkill -f rp.pmgr_launchi
wuh20@cheyenne4:~/github/pv-workflow/02_WeightOptimization> ps -u wuh20 | grep -e python -e rp
wuh20@cheyenne4:~/github/pv-workflow/02_WeightOptimization> 
```

## Final Remark

This solution has been tested on February 1, 2021. Feel free to contact me if you have problems.

