---
layout: page
title: "Unstructured Grid Refinement for Numeric Weather Prediction"
description: "Using an unstructured grid for better weather predictions"
category: "Machine Learning"
---

## Abstract

The [Analog Ensemble](https://weiming-hu.github.io/CAnalogsV2/) is a statistical technique to generate probabilistic forecasts. This is a computationally efficient solution to ensemble modeling because it does not require multiple NWP simulations, but a single model realization. However, the required computation can grow very large because atmospheric models are routinely run with increasing resolutions. For example, the NAM contains over 262,792 grids to generate a 12 km prediction. NWP models generally use a structured grid to represent the domain, despite the fact that certain physical changes occur non-uniformly across space and time. For example, temperature changes tend to occur more rapidly in mountains than plateaus.

A new machine learning based algorithm is proposed to dynamically and automatically learn the optimal unstructured grid pattern. This iterative algorithm is guided by machine learning rule generation and instantiation to identify grid vertices. Analog computations are performed only at vertices, therefore minimizing the number of vertices. Identifying their locations are paramount to optimize the available computational resources, minimize queue time, and ultimately achieve better results. The optimal unstructured grid is then used to perform probabilistic forecasts for a variety of applications like uncertainty quantification or renewable energy prediction. In this work, the short-term temperature is used as a study case.

## Poster

<object data="{{ site.url }}{{ site.baseurl }}/assets/pdf/poster_doug.pdf" width="100%" height="800" type="application/pdf"></object>

<br> 

## Presentation

<iframe width="100%" height="600" src="https://www.youtube.com/embed/UYFDw9J2wPo" frameborder="0" allow="autoplay; encrypted-media" allowfullscreen></iframe>

<br> 

## References

1. Delle Monache, Luca, et al. "Probabilistic weather prediction with an analog ensemble." Monthly Weather Review 141.10 (2013): 3498-3516.
2. Alessandrini, S., et al. "An analog ensemble for short-term probabilistic solar power forecast." Applied energy 157 (2015): 95-110.
3. Junk, Constantin, et al. "Predictor-weighting strategies for probabilistic wind power forecasting with an analog ensemble." Meteorologische Zeitschrift 24.4 (2015): 361-379.
4. Cervone, Guido, et al. "Short-term photovoltaic power forecasting using Artificial Neural Networks and an Analog Ensemble." Renewable Energy 108 (2017): 274-286.
