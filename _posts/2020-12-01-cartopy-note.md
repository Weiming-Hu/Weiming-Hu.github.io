---
layout: post
title: "Cartopy Study Notes (on NCAR Cheyenne)"
description: "Study notes for Cartopy"
tags: Python
giscus_comments: true
date: 2020-12-01T11:00:00
related_posts: false
---

<!-- vim-markdown-toc GFM -->

- [Setup](#setup)
  - [Environment](#environment)
  - [Map Data](#map-data)
  - [Other Settings](#other-settings)
- [Examples](#examples)
  - [Working with Projections](#working-with-projections)
- [Resources](#resources)

<!-- vim-markdown-toc -->

## Setup

### Environment

Since I'm using `cartopy` on supercomputers, I usually do not have root access for installation. I need to make sure everything I do is within my user space.

> To prepare an environment in user space, this approach might no longer work. Please see my more recent post on this matter [here](https://weiming-hu.github.io/user-NPL-jupyter/).

```shell
module load python

# To make conda accessible, we need to activate a base environment
source /ncar/usr/jupyterhub/20190905/bin/activate

# If you have not yet created these directories
mkdir $HOME/conda_env
mkdir $HOME/conda_pkg

# Specify system environment
export CONDA_ENVS_PATH=$HOME/conda_env
export CONDA_PKGS_DIRS=$HOME/conda_pkg

# Create a new environment
# conda create -n "myenv" python=3.7.5 ipython
#
# Or clone from a pre-existing environment
conda create --name venvConda --clone /ncar/usr/jupyterhub/20190905

# Activate the environment
conda activate venvConda

# Install additional modules
conda install -y --channel conda-forge cartopy
conda install -y ipykernel

# Add the environment to jupyter kernals
python -m ipykernel install --user --name venvConda
```

The above settings of `CONDA_ENVS_PATH` and `CONDA_PKGS_DIRS` are meant for Linux OS. For other OS, please refer to the [official documentation](https://conda.io/projects/conda/en/latest/user-guide/configuration/use-condarc.html#specify-environment-directories-envs-dirs).

If you want to remove any kernels from jupyter notebook, you can use the following command.

```shell
# Get a list of available kernels
jupyter kernelspec list

# Remove an unwanted kernel
jupyter kernelspec uninstall <kernel name to remove> 
```

### Map Data

Compute nodes on Cheyenne have very slow Internet connection. This would potentially slow down our visualization process. Let's download all the data for visualization ahead of time. We will use [this script](https://github.com/SciTools/cartopy/blob/master/tools/feature_download.py) to download all data that `cartopy` uses.

```shell
# Create a directory for storing visualization data
mkdir ~/scratch/data/Cartopy/
cd ~/scratch/data/Cartopy/

# Download the script
wget https://raw.githubusercontent.com/SciTools/cartopy/master/tools/feature_download.py

# If you have not yet activated the environment we have just created
# run the following
#
# conda activate venvConda

# Run the script to download all data
python feature_download.py --output ~/scratch/data/Cartopy/ \
    gshhs physical cultural cultural-extra
```

The total size of the downloaded files is about 600 MB (as of 2020/11/30). So please be patient. Later on during the development before using `cartopy`, we will set the data directory accordingly.

```python
import os
import cartopy
cartopy.config['pre_existing_data_dir'] = os.path.expanduser('~/scratch/data/Cartopy/')

```

However, the downloaded script still does not download absolutely all the data needed. When I tried to add the `ax.add_feature(cf.STATES)` layer, the downloader was again called. So I have to manually download some files after all.

```shell
# Change to the data directory
cd ~/scratch/data/Cartopy/shapefiles/natural_earth/cultural

# The URL will be displayed when the downloaded was initialized. You can discover 
# new datasets to download from there. Here is an example.
#
wget http://naciscdn.org/naturalearth/110m/cultural/ne_110m_admin_1_states_provinces_lakes.zip

# Unzip
unzip ne_110m_admin_1_states_provinces_lakes.zip
```

### Other Settings

I have had mysterious segmentation fault when using `cartopy.feature` and projections. I following the steps to reinstall `shaply` to avoid segmentation faults as suggested [here](https://github.com/SciTools/cartopy/issues/879).

```shell
# Make sure you are using the proper python environment
pip uninstall shapely; pip install --no-binary :all: shapely
```

## Examples

Here are some basic imports.

```python
import os
import cartopy
import warnings

import xarray as xr
import matplotlib as mpl
import cartopy.crs as ccrs
import cartopy.feature as cf
import matplotlib.pyplot as plt

# Ignore matplotlib deprecation warnings
warnings.filterwarnings("ignore", category=mpl.cbook.mplDeprecation)
```

### Working with Projections

The dataset `ds` is an `xarray.core.dataset.Dataset` based on my own data. It would work other data too.

```python
projection_visual = ccrs.LambertConformal(central_longitude=-96, central_latitude=45)
# projection_visual = ccrs.Orthographic(central_longitude=-96, central_latitude=45)
# projection_visual = ccrs.Mercator()
# projection_visual = ccrs.PlateCarree()

# My data coordinates are un-projected coordinates in latitudes and longitudes.
# So I'm using Geodetic. It is an unprojected coordinate system.
# You cannot use this as the visualization projection.
#
data_crs = ccrs.Geodetic()

fig, ax = plt.subplots(figsize=(16, 12), subplot_kw={'projection': projection_visual})
ds.plot.scatter(x='Xs', y='Ys', hue='StationState', transform=data_crs, ax=ax, s=0.7)

ax.add_feature(cf.COASTLINE)
ax.add_feature(cf.STATES)

# ax.set_global()
fig.show()
```

## Resources

Here is a list of resources:

- [Deal with projections with `xarray`](http://xarray.pydata.org/en/stable/examples/visualization_gallery.html?highlight=projection#Multiple-plots-and-map-projections)
- [Cartopy projection list](https://scitools.org.uk/cartopy/docs/v0.15/crs/projections.html)
- [Coordinate reference systems in Cartopy](https://scitools.org.uk/cartopy/docs/v0.15/crs/index.html#coordinate-reference-systems-in-cartopy)
