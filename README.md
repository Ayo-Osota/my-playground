# Drawing Awesome Shapes with Elliptical Border Radius in CSS

Most are familiar with the basic usage of `border-radius` to create circular or rounded shapes, many are unaware of its potential for creating more complex and interesting shapes using elliptical `border-radius`.

## Understanding Border-Radius

By setting a uniform value, you can achieve simple rounded corners or circles.



<div class="even-columns">

```css
<!-- Simple rounded corners -->
.circle {
    width: 100px;
    height: 100px;
    background-color: #3498db;
    border-radius: 50%; /* Creates a circle */
}
```
<div style="width: 100px; height: 100px; background-color: #3498db; border-radius: 50%;"></div>
</div>

## Elliptical Border-Radius

 To create an elliptical `border-radius`, you can use two values separated by a slash (`/`):

<div class="even-columns">

```css
.elliptical {
    width: 200px;
    height: 100px;
    background-color: #e74c3c;
    border-radius: 50% / 25%;
}
```
<div style="width: 200px;
    height: 100px;
    background-color: #e74c3c;
    border-radius: 50% / 25%;"></div>
</div>


### By adjusting the values of the elliptical `border-radius`, you can create a variety of unique shapes. Here are a few examples:

## Pill Shape

<div class="even-columns">


```css
.pill {
    width: 300px;
    height: 100px;
    background-color: #2ecc71;
    border-radius: 50px / 50%;
}
```
<div style="width: 300px;
    height: 100px;
    background-color: #2ecc71;
    border-radius: 50px / 50%;"></div>
</div>



<style>
.even-columns {
  display: grid;
  gap: 1rem;
  margin: 2rem 0;
}



@media (min-width: 768px) {
    .even-columns > *:first-child {
    width: 50%;
}
  .even-columns {
    display: flex;
    align-items: center;
  }
}
</style>