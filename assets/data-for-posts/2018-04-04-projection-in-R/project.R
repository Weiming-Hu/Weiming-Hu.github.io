# "`-''-/").___..--''"`-._
#  (`6_ 6  )   `-.  (     ).`-.__.`)   WE ARE ...
#  (_Y_.)'  ._   )  `._ `. ``-..-'    PENN STATE!
#    _ ..`--'_..-_/  /--'_.' ,'
#  (il),-''  (li),'  ((!.-'
# 
# Author: Weiming Hu <weiming@psu.edu>
#         Geoinformatics and Earth Observation Laboratory (http://geolab.psu.edu)
#         Department of Geography and Institute for CyberScience
#         The Pennsylvania State University

# This is the example code for the article posted here
# https://weiming-hu.github.io/geography/2018/04/04/projection-in-R.html

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

library(maps)
library(RColorBrewer) # this is only for color scale
# jpeg('map-original.jpeg', width = 600, height = 450)
plot(dat, main = 'Temperature Forecasts on 20141231 at 0600',
     col = brewer.pal(11, 'Spectral')[11:1])


# jpeg('coordinates-by-values.jpeg', width = 600, height = 450)
map()

# plot a sample of the points because this is faster
s <- sample(1:nrow(xyz), 5000)
points(xyz$x[s], xyz$y[s],
       cex = 0.05, pch = 19)


# specify CRS to be used
crs.latlon <- CRS("+proj=longlat +datum=WGS84")

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
# jpeg('map-with-lonlat.jpeg', width = 600, height = 450)
plot(rast, main = 'Temperature Forecasts on 20141231 at 0600',
     col = brewer.pal(11, 'Spectral')[11:1])
map(col = 'grey', add = T)
map('state', add = T)

# jpeg('map-no-na.jpeg', width = 600, height = 450)
source('fill-raster-NA.R')
rast.no.na <- fill.raster.NA(rast)
plot(rast.no.na, main = 'Temperature Forecasts on 20141231 at 0600',
     col = brewer.pal(11, 'Spectral')[11:1])
map(col = 'grey', add = T)
map('state', add = T)

################
# reprojection #
################
# reproject to lcc
crs.lcc <- CRS("+proj=lcc +lat_1=12.190 +lat_0=40 
               +lon_0=-97 +lat_2=45
               +ellps=WGS84 +datum=WGS84 +units=m +no_defs")
rast.ext <- projectExtent(rast.no.na, crs.lcc)
rast.lcc <- projectRaster(rast.no.na, rast.ext)

# overlay
library(sp)
library(maptools)
usa <- map("state", fill = TRUE, 
           col="transparent", plot=FALSE)
IDs <- sapply(strsplit(usa$names, ":"), function(x) x[1])
usa <- map2SpatialPolygons(
  usa, IDs=IDs, proj4string=CRS("+proj=longlat +datum=WGS84"))
usa.lcc <- spTransform(usa, CRSobj = crs.lcc)

world <- map(fill = TRUE, col="transparent", plot=FALSE)
IDs <- sapply(strsplit(world$names, ":"), function(x) x[1])
world <- map2SpatialPolygons(
  world, IDs=IDs, proj4string=CRS("+proj=longlat +datum=WGS84"))
world.lcc <- spTransform(world, CRSobj = crs.lcc)

# jpeg('map-lcc.jpeg', width = 600, height = 450)
plot(rast.lcc, main = 'Temperature Map on North America Lambert Conformal Conic',
     col = brewer.pal(11, 'Spectral')[11:1])
plot(world.lcc, add = T, border = 'grey')
plot(usa.lcc, add = T)

# reproject to laea
# let's choose a wild projection
#
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
