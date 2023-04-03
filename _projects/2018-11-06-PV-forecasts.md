---
layout: page
title: "High Resolution Forecasts of Photovoltaic Energy Production"
description: "Using Analog Ensemble to forecast photovoltaic energy production on a household level"
category: "Machine Learning"
---

## Key Points

This poster studies the temporal predictability of the small-scale photovoltaic energy production. The location studies is at State College, PA. The study compares the prediction performance between the Artificial Neural Network (ANN) and the [Analog Ensemble](https://weiming-hu.github.io/AnalogsEnsemble/) (AnEn) technique in forecasting short-term (3-day forecasts) photovoltaic energy production.

- The optimal ANN is trained using a brute-force search with 1-20 hidden nodes, and the best one with 10 nodes is chosen. At this specific location, AnEn outperforms ANN in respect to daily averaged errors. ANN tends to under-predict the peak photovoltaic (PV) energy production while AnEn shows good results during peak PV energy time period.
- AnEn can be used to downscale hourly PV forecasts to every 5-minute which, in this case, is the resolution of PV observations. Results show that the downscaling technique with AnEn is fairly accurate when the original forecasts are accurate.
- Both (ANN and AnEn) model are not able to perform well when the underlying meteorological model is inaccurate.

## Poster

<object data="{{ site.url }}{{ site.baseurl }}/assets/pdf/poster_pv.pdf" width="100%" height="800" type="application/pdf"></object>
