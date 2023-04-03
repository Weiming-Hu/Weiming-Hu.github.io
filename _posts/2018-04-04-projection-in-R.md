---
layout: post
title: "Deal with Raster and Projection in R"
description: "Coordinate systems and reprojection in R"
tags: R
giscus_comments: true
date: 2018-04-14T21:53:00
related_posts: false
---

<!-- vim-markdown-toc GitLab -->

- [Introduction](#introduction)
- [Data](#data)
- [Georectified Raster with Lat-Lon](#georectified-raster-with-lat-lon)
- [Raster Reprojection](#raster-reprojection)
  - [Projection 1: North America Lambert Conformal Conic](#projection-1-north-america-lambert-conformal-conic)
  - [Projection 2: Lambert Azimuthal Equal Area](#projection-2-lambert-azimuthal-equal-area)
- [Everything Together](#everything-together)
- [References](#references)
- [Update](#update)

<!-- vim-markdown-toc -->

## Introduction

When dealing with data with coordinates, we usually have the longitudes and the latitudes of each data value, and we want to plot the georectified map overlayed with other map layers, or simply we just want to plot the georectified map in a different coordinate system. For example, if you have a data table like below.

```
> # x for longitudes
> # y for latitudes
> # z for values
> #
> head(xyz)
          x        y        z
1 -152.8786 54.56534 278.4887
2 -152.7311 54.60416 278.3637
3 -152.5835 54.64287 278.2387
4 -152.4358 54.68147 278.1137
5 -152.2878 54.71995 278.1137
6 -152.1398 54.75833 278.1137
```

And you want to plot the data using a raster with the correct projection. There might be several problems:

1. The x and y grids are irregular. It is very common that locations by latitudes and longitudes do not lie perfectly on a regular grid. Some locations are closer to each other and the others are further out.
2. The domain of the dataset is not rectangular. In other words, the shape of the domain can be complicate.
3. Interpolation method of the whole dataset is not preferred. Interpolation is very time-consuming, and interpolation might bring errors.

After search, I compiled the information online, and recommend the following steps to efficiently create a georectified raster.


## Data

You can find the `Rdata` file [here](https://weiming-hu.github.io/assets/data-for-posts/2018-04-04-projection-in-R/NAM-temperature.Rdata) which will be used in this post. The data are subset from [North American Mesoscale Forecast Model](https://www.ncdc.noaa.gov/data-access/model-data/model-datasets/north-american-mesoscale-forecast-system-nam). You can download the full data by your own option from the [NCAR Research Data Archive](https://rda.ucar.edu/datasets/ds335.0/).


## Georectified Raster with Lat-Lon

First, you have the variable `xyz`. NAM model output has 428 rows and 614 cols, so I hard coded these attributes. We have longitudes, latitudes, and values for each of the grid. **Please note that although the grid is regular originally, lat-lon grid is irregular since we want to plot the map in lat-lon.**

```
library(raster)
load('NAM-temperature.Rdata')

# NAM forecast output has 428 rows and 614 columns
nrow <- 428
ncol <- 614

# if you download the original netcdf file
# and use raster to read the variable
# you will get similar results like this
#
dat <- raster(
  xmn = 1, xmx = ncol,
  ymn = 1, ymx = nrow,
  nrows = nrow, ncols = ncol,
  vals = xyz$z)
```

Let's take a simple look at the `dat` which is the temperature in Kelvin and the actual locations of lat-lon points.

```
library(maps)
library(RColorBrewer) # this is only for color scale
plot(dat, main = 'Temperature Forecasts on 20141231 at 0600',
     col = brewer.pal(11, 'Spectral')[11:1])
```

<div class="row mt-6">
    <div class="col-sm mt-6 mt-md-0">
        {% include figure.html path="assets/data-for-posts/2018-04-04-projection-in-R/map-original.jpeg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

You can see the xs and ys are off, and the shape of America looks weird. What should our domain look like? Let's plot the coordinates by values.

```
map()

# plot a sample of the points because this is faster
s <- sample(1:nrow(xyz), 5000)
points(xyz$x[s], xyz$y[s],
       cex = 0.05, pch = 19)
```

<div class="row mt-6">
    <div class="col-sm mt-6 mt-md-0">
        {% include figure.html path="assets/data-for-posts/2018-04-04-projection-in-R/coordinates-by-values.jpeg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

The domain has a bit of a curvature rather than being a perfect rectangle.

Therefore, we are going to do the following steps:

1. Get the extent (bounding box) of the domain together with the number of rows and columns.
2. Rasterize data with longitudes and latitudes.
3. Deal with NA values.

```
# For color schemes
library(RColorBrewer)

# specify CRS to be used
crs.latlon <- CRS("+proj=longlat +datum=WGS84")

# get nrow and ncol
# this is only an approximation
#
nrow <- dim(dat)[1]
ncol <- dim(dat)[2]

# get extent of the domain
ext <- extent(xyz[, c('x', 'y')])

# create a raster for extent with nrow and ncol
rast <- raster(
  ext, nrow = nrow, ncol = ncol,
  crs = crs.latlon)

# rasterize the data values
rast <- rasterize(
  xyz[, c('x', 'y')], rast,
  xyz[, 'z'], fun=mean)

# you have the map
print(rast)
plot(rast, main = 'Temperature Forecasts on 20141231 at 0600',
     col = brewer.pal(11, 'Spectral')[11:1])
map(col = 'grey', add = T)
map('state', add = T)
```

<div class="row mt-6">
    <div class="col-sm mt-6 mt-md-0">
        {% include figure.html path="assets/data-for-posts/2018-04-04-projection-in-R/map-with-lonlat.jpeg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

At last we deal with NA values. Popular ways to replace NAs include nearest neighbor and interpolation. If you have multiple raster layers, you can also try to extract values for the NA locations from other layers using the R function [`raster::approxNA`](https://rdrr.io/cran/raster/man/approxNA.html). But this function only works with `RasterStack` or `RasterBrick`. As mentioned in the beginning, interpolation is slow and might bring errors. Here, because we are only interpolating for NA locations which should not be too many, we constrain the negative impact. The function provided [here](https://gist.github.com/Weiming-Hu/ee8981bef06c6512327e4c4d9a91fecb) only interpolates the NA locations where the up, down, right, and left values are all valid.

```
source('fill-raster-NA.R')
rast.no.na <- fill.raster.NA(rast)
plot(rast.no.na, main = 'Temperature Forecasts on 20141231 at 0600',
     col = brewer.pal(11, 'Spectral')[11:1])
map(col = 'grey', add = T)
map('state', add = T)
```

<div class="row mt-6">
    <div class="col-sm mt-6 mt-md-0">
        {% include figure.html path="assets/data-for-posts/2018-04-04-projection-in-R/map-no-na.jpeg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

## Raster Reprojection

Now that we have a georectified raster, if you want to reproject the raster to a different projection, you can easily do it.

### Projection 1: North America Lambert Conformal Conic

I chose this projection because this is the projection used by the [NAM model](https://rda.ucar.edu/datasets/ds335.0/).

When I plot the overlay map, I didn't use the function `maps::map` directly because I just couldn't get it working. If you have an idea please comment. Instead I extract the spatial objects from `map` and reproject them myself.

```
# this are the parameters I chose. I had a hard time selecting
# these parameters. See explanations after this code for why
# I chose these parameters.
#
crs.lcc <- CRS("+proj=lcc +lat_1=12.190 +lat_0=40 
               +lon_0=-97 +lat_2=45
               +ellps=WGS84 +datum=WGS84 +units=m +no_defs")

# reproject to lcc
rast.ext <- projectExtent(rast.no.na, crs.lcc)
rast.lcc <- projectRaster(rast.no.na, rast.ext)

# overlay
library(sp)
library(maptools)

# extract usa spatial polygons
usa <- map("state", fill = TRUE, 
           col="transparent", plot=FALSE)
IDs <- sapply(strsplit(usa$names, ":"), function(x) x[1])
usa <- map2SpatialPolygons(
  usa, IDs=IDs, proj4string=CRS("+proj=longlat +datum=WGS84"))
usa.lcc <- spTransform(usa, CRSobj = crs.lcc)

# extract world spatial polygons
world <- map(fill = TRUE, col="transparent", plot=FALSE)
IDs <- sapply(strsplit(world$names, ":"), function(x) x[1])
world <- map2SpatialPolygons(
  world, IDs=IDs, proj4string=CRS("+proj=longlat +datum=WGS84"))
world.lcc <- spTransform(world, CRSobj = crs.lcc)

# plot them together
plot(rast.lcc, main = 'Temperature Map on North America Lambert Conformal Conic',
     col = brewer.pal(11, 'Spectral')[11:1])
plot(world.lcc, add = T, border = 'grey')
plot(usa.lcc, add = T)
```

<div class="row mt-6">
    <div class="col-sm mt-6 mt-md-0">
        {% include figure.html path="assets/data-for-posts/2018-04-04-projection-in-R/map-lcc.jpeg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>

**Explanation for the prjoection parameters**

- You can get a approximately correct projection from [ESRI:102009 North America Lambert Conformal Conic](https://epsg.io/102009). However you need to fine tune the parameters.
- `+proj=lcc` is the name of the projection. You can also search the name in [EPSG](https://epsg.io/).
- `+lat_0=40 +lon_0=-97` is the location where you want the projection to be centered. For example, here I chose the center at Penn State. You can find coordinates using [Google Map](https://www.google.com/maps/)
- `+lat_1=12.190` is the start of Lambert Conformal projection specified by the model. I found this in [NAM model](https://rda.ucar.edu/datasets/ds335.0/) `Spatial Coverage -> Detailed coverage information -> NAM 12km CONUS files`.
- `+lat_2=45` is the curvature of the domain as my understanding. I haven't looked into this. So please correct me if I'm wrong.
- `+ellps=WGS84 +datum=WGS84` is the datum used with the projection.
- `+units=m` is the unit.

I have to admit that finding the correct parameters for a projection is not easy, and I didn't find good tutorial on this. If you are having the same issue or you have more experience on this, let's create a general guide for using different projections.


### Projection 2: Lambert Azimuthal Equal Area

Now we have the skills, let's choose a weird projection and try it out.

```
crs.laea <- CRS("+proj=laea +lat_0=90 +lon_0=0
               +x_0=0 +y_0=0 +ellps=WGS84 
               +datum=WGS84 +units=m +no_defs")
rast.ext <- projectExtent(rast.no.na, crs.laea)
rast.laea <- projectRaster(rast.no.na, rast.ext)

# overlay
usa.laea <- spTransform(usa, CRSobj = crs.laea)
world.laea <- spTransform(world, CRSobj = crs.laea)

# jpeg('map-laea.jpeg', width = 600, height = 450)
plot(rast.laea, main = 'Temperature Map on Lambert Azimuthal Equal Area',
     col = brewer.pal(11, 'Spectral')[11:1])
plot(world.laea, add = T, border = 'grey')
plot(usa.laea, add = T)
```

<div class="row mt-6">
    <div class="col-sm mt-6 mt-md-0">
        {% include figure.html path="assets/data-for-posts/2018-04-04-projection-in-R/map-laea.jpeg" class="img-fluid rounded z-depth-1" zoomable=true %}
    </div>
</div>


## Everything Together

To put every thing together, you can download the R script [here](https://weiming-hu.github.io/assets/data-for-posts/2018-04-04-projection-in-R/project.R).


## References

- [ESRI:102009 North America Lambert Conformal Conic](https://epsg.io/102009)
- [ESRI:102017 North Pole Lambert Azimuthal Equal Area](https://epsg.io/102017)
- [Specifying Grids, Ellipsoids, and Map Projections](http://www.cmascenter.org/sa-tools/documentation/4.2/html/grids_ellipsoids_map_proj.html)
- [Mapmate R package for 3D latlon visualization](https://leonawicz.github.io/mapmate/articles/mapmate.html)
- [Overview of coordinate reference systems in R](https://www.nceas.ucsb.edu/~frazier/RSpatialGuides/OverviewCoordinateReferenceSystems.pdf)
- [Georectification](https://lincolnmullen.com/projects/spatial-workshop/georectification.html)
- [maps package R](https://cran.r-project.org/web/packages/maps/maps.pdf)
- [How to make raster from irregular point data without interpolation](https://gis.stackexchange.com/questions/79062/how-to-make-raster-from-irregular-point-data-without-interpolation/79074)
- [Projecting sp objects in R](https://gis.stackexchange.com/questions/31743/projecting-sp-objects-in-r)
- [Converting a 'map' object to a 'SpatialPolygon' object](https://stackoverflow.com/questions/26062280/converting-a-map-object-to-a-spatialpolygon-object)
- [map2SpatialPolygons: Convert map objects to sp classes](https://rdrr.io/cran/maptools/man/map2SpatialPolygons.html)

## Update

**Mar. 25, 2019** Although `RColorBrewer` schemes are limited by the number of colors in each scheme, you can easily interpolate as many colors as you want using the colors provided and the `colorRampPalette` function. Note the `(100)` at the end which specifies the number of colors to generate.

```
plot(rast, col = colorRampPalette(brewer.pal(11, 'Spectral')[11:1])(100))
```
