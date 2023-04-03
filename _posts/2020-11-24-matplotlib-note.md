---
layout: post
title: "Matplotlib Study Note"
description: "Study notes for Matplotlib"
tags: Python
giscus_comments: true
date: 2020-11-24T14:00:00
related_posts: false
---

<!-- vim-markdown-toc GFM -->

- [Resources](#resources)
- [My First Confusion](#my-first-confusion)
- [Exploration of Plotting](#exploration-of-plotting)
- [Performance Considerations](#performance-considerations)

<!-- vim-markdown-toc -->

## Resources

All of the reading comes from the [Tutorial](https://matplotlib.org/tutorials/index.html) page of `matplotlib`. I have read the following pages:

1. [Usage Guide](https://matplotlib.org/tutorials/introductory/usage.html#sphx-glr-tutorials-introductory-usage-py)
2. [The Lifecycle of a Plot](https://matplotlib.org/tutorials/introductory/lifecycle.html#the-lifecycle-of-a-plot)
3. [Customizing Matplotlib with style sheets and rcParams](https://matplotlib.org/tutorials/introductory/usage.html#the-object-oriented-interface-and-the-pyplot-interface)
4. [Artist Tutorial](https://matplotlib.org/tutorials/intermediate/artists.html#sphx-glr-tutorials-intermediate-artists-py)
5. [Legend Guide](https://matplotlib.org/tutorials/intermediate/legend_guide.html#sphx-glr-tutorials-intermediate-legend-guide-py)
6. [Customizing Figure Layouts Using GridSpec and Other Functions](https://matplotlib.org/tutorials/intermediate/gridspec.html#customizing-figure-layouts-using-gridspec-and-other-functions)

## My First Confusion

**Which one should I use,** `matplotlib.pyplot` or `figure.Figure` ?

`matplotlib` has two interfaces:

1. `Figure` being the object-oriented interface
2. `pyplot` being the state-based interface.

They are equally powerful and both are illustrated in `matplotlib` docmentation. But it is better to stick to the one and do no mix them. Some suggestions are:

1. `pyplot` behaves like base plot in [R](https://www.r-project.org/) and is [suggested](https://matplotlib.org/tutorials/introductory/usage.html#the-object-oriented-interface-and-the-pyplot-interface) when generating simple plots in an interactive environment (e.g. Jupyter notebook).
2. In general, it is [suggested](https://matplotlib.org/tutorials/introductory/lifecycle.html#a-note-on-the-object-oriented-api-vs-pyplot) to use `figure.Figure`.

I will focus on using the `figure.Figure` interface throughout my study notes.

## Exploration of Plotting

Some common imports are as follow:

```python
import matplotlib as mpl
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
```

I have compiled the following list of examples for generating figures. Each example provides some information on figure customization.

1. [Generating a multi-line plot](https://matplotlib.org/tutorials/introductory/usage.html#the-object-oriented-interface-and-the-pyplot-interface) with a legend and axis labels

2. [Generating a bar plot](https://matplotlib.org/tutorials/introductory/lifecycle.html#combining-multiple-visualizations) with customizations of the automatic layout, the figure style, the x-axis, figure saving, and combining multiple visualizations

3. [An example matplotlibrc file](https://matplotlib.org/tutorials/introductory/customizing.html#a-sample-matplotlibrc-file) with customization settings for all kinds of properties. Typically, we can change them by using the following code. Actually, `mpl.rcParams` is just a dictionary that controls the default aesthetics. Changing its values will change the default setting.
   - `mpl.rcParams['lines.linewidth'] = 2`
   - `mpl.rcParams.update({'font.size': 22})`
   - `mpl.rc('lines', linewidth=4, linestyle='-.')`
   - `mpl.rc('ytick.minor', size=0.33333)`
   - `ax.plot([1, 2, 3], [1, 3, 2], linewidth=4)`
   
4. [Changing properties of axis containers](https://matplotlib.org/tutorials/intermediate/artists.html#sphx-glr-tutorials-intermediate-artists-py) including ticks and labels
   
   > There are two types of Artists: *primitives* and *containers*. The primitives represent the standard graphical objects we want to paint onto our canvas (Line2D, Rectangle, Text, AxesImage, etc.), and the containers are places to put them (Tick, Axis, Axes and Figure).

5. [Fine-tuning legend position](https://matplotlib.org/api/_as_gen/matplotlib.pyplot.legend.html#matplotlib.pyplot.legend) using `bbox_to_anchor` and `loc`

   > Those artists with an empty string as label or with a label starting with "_" will be ignored.

6. [Creating more complex legends](https://matplotlib.org/tutorials/intermediate/legend_guide.html#legend-handlers) with legend handler

7. [Creating complex grid layout](https://matplotlib.org/tutorials/intermediate/gridspec.html#fine-adjustments-to-a-gridspec-layout) with `Gridspec` and adjusting margins between panels

## Performance Considerations

All of the following points and more are available from the [Performance](https://matplotlib.org/tutorials/introductory/usage.html#performance) section.

1. **Lines**: Set `rcParams['path.simplify']` and `rcParams['path.simplify_threshold']` to speed up line segment redering.
2. **Legends**: Specifically provide a location for legends to avoid the expensive default operation of finding the best location.
3. **Presets**: Use the `fast` style for automatic simplification.

