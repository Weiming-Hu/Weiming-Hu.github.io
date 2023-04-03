---
layout: post
title: "Creating User Clone of NCAR Package Library for JupyterHub"
description: "User clone of NPL for JupyterHub"
tags: Python HPC
giscus_comments: true
date: 2021-02-09T12:00:00
related_posts: false
---

## Motivation

This posts documented how I created a user clone of NCAR package library (NPL) for use with JupyterHub. I want to do this because:

1. I want to use the [JupyterHub](https://jupyterhub.ucar.edu) interface
2. I want to be able to install packages to kernels. This would prevent me from using any of the pre-existing kernels.

Most of the steps are already documented [here](https://www2.cisl.ucar.edu/resources/python-%E2%80%93-ncar-package-library#clone). But JupyterHub environment was probably built with a different setup with the cloned NPL package. So there are extra steps to make sure the same version sare installed.

> This post was last tested on Feb. 24, 2021.

## Clone NPL

```bash
# Load the required modules
module load python ncarenv
	
# Check available modules
ncar_pylib -l

# Clone the module to your user space
cd $HOME
ncar_pylib -c 20201220 venv_Jupyter
```

You can find more details on this matter [here](https://www2.cisl.ucar.edu/resources/python-%E2%80%93-ncar-package-library#clone). The clone process took about 3 minutes for me and it printed the following output.

```bash
Cloning 20201220 into directory venv_Jupyter...

    Python 3.7.9-20201220 library on system cheyenne
    Modules used to build this package library:

        ncarenv/1.3
        gnu/9.1.0
        ncarcompilers/0.5.0
        python/3.7.9

    When installing packages into your personal clone,
    use the aforementioned modules to avoid compiler
    and library errors.

    We also recommend using the "--no-cache-dir" option
    to the pip install command, which will prevent pip from
    installing previously compiled package installs that
    may have used different library versions. Example:

        module purge
        module load ncarenv gnu/9.1.0 ncarcompilers python/3.7.9
        ncar_pylib -c default my_clone
        ncar_pylib my_clone
        pip --no-cache-dir install PACKAGE_NAME
```

And then, let's add the cloned module as a kernel so that we can use it in Jupyter notebooks.

```bash
ncar_pylib --kernel venv_Jupyter
```

## Module Consistency

The following python code check should be done after requesting a session from JupyterHub and in a Jupyter notebook. You should toggle the kernel between `Python 3` and `venv_Jupyter` to see the different versions if any.

```python
import bokeh
import tornado
import distributed
import dask_jobqueue
```

At the time when I wrote this post, `Python 3` had the following configuration.

```python
print(distributed.__version__)    # 2.3.2
print(dask_jobqueue.__version__)  # 0.6.3
print(tornado.version)            # 6.0.3
print(bokeh.__version__)          # 1.3.4
print(msgpack.__version__)        # 0.6.1
```

`venv_Jupyter` had the following configuration.

```python
print(distributed.__version__)    # 2020.12.0
print(dask_jobqueue.__version__)  # 0.7.2
print(tornado.version)            # 6.1
print(bokeh.__version__)          # 2.2.3
print(msgpack.__version__)        # 1.0.2
```

Our next step is to install version specific modules into `venv_Jupyter`.

```bash
# These lines come from the previous output when you cloned NPL
module purge
module load ncarenv gnu/9.1.0 ncarcompilers python/3.7.9
ncar_pylib venv_Jupyter

# Install version specific modules
pip install --no-cache-dir \
  distributed==2.3.2 \
  dask_jobqueue==0.6.3 \
  tornado==6.0.3 \
  bokeh==1.3.4 \
  msgpack==0.6.1
```

## Issues

Please feel free to comment if you have found other modules that should be checked for version consistency. Thank you!

