---
layout: page
permalink: /publications/
title: Pubs
description: A selected list of publications in reversed chronological order
nav: true
nav_order: 3
---

<div class="publications">

{% assign thisyear = "now" | date: "%Y" | plus: 0 %}

<!-- Itterate on all page years -->
{% for y in (2018..thisyear) reversed %}
  <!-- fetch the number of objects for this year -->
  {%- capture refs -%}{%- bibliography_count -f papers -q @*[year={{y}}]* -%}{%- endcapture -%}
  {% assign refs = refs | plus: 0 %}
  <!-- if we have a bibliography reference for this year -->
  {% if refs > 0 %}
    <!-- Create a year heading -->
    <h2 class="year">{{y}}</h2>
    <!-- create the bibliography card -->
    {% bibliography -f papers -q @*[year={{y}}]* %}
  {% endif %}
{% endfor %}

</div>
