# Drawing Awesome Shapes with Elliptical Border Radius in CSS

Most are familiar with the basic usage of `border-radius` to create circular or rounded shapes, many are unaware of its potential for creating more complex and interesting shapes using elliptical `border-radius`.

## Table of Contents
1. [Understanding Border-Radius](#understanding-border-radius)
2. [Elliptical Border-Radius](#elliptical-border-radius)
3. [Examples](#examples)
    - [Pill Shape](#pill-shape)
    - [Leaf Shape](#leaf)
    - [Egg Shape](#egg)
    - [Moon Shape](#moon)
    - [Speech Bubble](#speech-bubble)
4. [Connect with Me](#connect-with-me)

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

<p class="even-columns">

```css
.elliptical {
    width: 200px;
    height: 100px;
    background-color: #e74c3c;
    border-radius: 50% / 25%;
}
```
<p style="width: 200px;
    height: 100px;
    background-color: #e74c3c;
    border-radius: 50% / 25%;">Hiii
    </p>
</p>

<p>Hello1</p>
<p style="width: 200px;
    height: 100px;
    background-color: #e74c3c;
    border-radius: 50% / 25%;"">Hello2</p>



## Examples
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
    border-radius: 50px / 50%;">
    </div>
</div>

## Leaf 

<div class="even-columns">


```css
.leaf {
    width: 200px;
    height: 100px;
    background-color: #f39c12;
    border-radius: 60% 30% / 30% 60%;
}
```
<div style="width: 200px;
  height: 100px;
  background-color: #f39c12;
  border-radius: 60% 30% / 30% 60%;">
    </div>
</div>

## Egg 

<div class="even-columns">


```css
.egg {
    width: 126px;
    height: 180px;
    background-color: #FFD79A;
    border-radius: 50% 50% 50% 50% / 60% 60% 40% 40%;
}
```
<div style="width: 126px;
      height: 180px;
      background-color: #FFD79A;
      border-radius: 50% 50% 50% 50% / 60% 60% 40% 40%;">
    </div>
</div>

## Moon 

<div class="even-columns">


```css
.moon {
    width: 160px;
    height: 160px;
    border-radius: 50%;
    rotate: 45deg;
    background: #F6F1D5;
    position: relative;
    overflow: hidden;
    box-shadow: 0 0 15px rgba(255, 255, 255, 0.8);
}

.moon > *:first-child {
    position: absolute;
    top: 0;
    right: 25px;
    width: 160px;
    height: 160px;
    border-radius: 50%;
    background: #333037; 
    box-shadow: 0 0 20px rgba(255, 255, 255, 0.8);  
}
```
<div style="
    width: 160px;
    height: 160px;
    border-radius: 50%;
    rotate: 45deg;
    background: #F6F1D5;
    position: relative;
    overflow: hidden;
    box-shadow: 0 0 15px rgba(255, 255, 255, 0.8);
">
    <div style="
        position: absolute;
        top: 0;
        right: 25px;
        width: 160px;
        height: 160px;
        border-radius: 50%;
        background: #333037; 
        box-shadow: 0 0 20px rgba(255, 255, 255, 0.8); 
    "></div>
</div>
</div>

## Speech bubble 

<div class="even-columns">


```css
.speech-bubble {
  width: 200px;
  height: 100px;
  background-color: #8e44ad;
  border-radius: 0 15% 15% / 0 30% 30%;
  position: relative;
}

.speech-bubble::before {
  content: "";
  position: absolute;
  top: -40px;
  left: 0;
  height: 40px;
  width: 40px;
  border-bottom-left-radius: 50%;
  background-color: transparent;
  box-shadow: 0 20px 0 0 #8e44ad;
}


```
<div class="speech-bubble">
    </div>
</div>


### Whether you’re designing buttons, icons, or unique decorative elements, elliptical border-radius is a valuable tool in your CSS toolkit.

[View on CodePen](https://codepen.io/johndoe/pen/abc123)
<svg style="font-size: 1.5rem; margin-top: 1rem;" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="m8.21 12l-1.33.89v-1.78zm3.26-2.18V7.34l-4.16 2.78l1.85 1.24zm5.23.3l-4.17-2.78v2.48l2.31 1.54zm-9.39 3.76l4.16 2.78v-2.48l-2.31-1.54zm5.22.3v2.48l4.17-2.78l-1.86-1.24zM12 10.74L10.12 12L12 13.26L13.88 12zM22 12c0 5.5-4.5 10-10 10S2 17.5 2 12S6.5 2 12 2s10 4.5 10 10m-3.82-1.88v-.07l-.01-.05l-.01-.05c-.01-.01-.01-.02-.02-.04l-.01-.02l-.02-.04l-.01-.02l-.02-.03l-.02-.03l-.03-.03l-.03-.02V9.7l-.04-.02l-.01-.01l-5.65-3.76a.53.53 0 0 0-.59 0L6.05 9.67v.01L6 9.7v.02l-.03.02l-.03.03l-.01.03l-.03.03l-.01.02l-.02.04l-.01.02l-.02.04V10h-.01l-.01.05v3.9l.01.05h.01v.05c.01.01.01.02.02.04l.01.02l.02.04l.01.02l.02.03l.02.03l.03.03l.03.02v.02l.04.02l.01.01l5.66 3.77c.08.06.19.08.29.08s.21-.03.3-.08l5.65-3.77l.01-.01l.04-.02v-.02l.03-.02l.03-.03l.02-.03l.02-.03l.01-.02l.02-.04l.01-.02l.02-.04V14h.01l.01-.05zm-1.06 2.77v-1.78l-1.33.89z"/></svg>

---
### ***If you believe it, you can achieve it***
*The only limit to your success is your imagination*



**osot💤**
---
## Connect with me

<div style="font-size: 2rem; display: flex; gap: 2rem;">
<a class="social-link" href="https://www.linkedin.com/in/ayo-osota/">
<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M22 3.47059V20.5294C22 20.9194 21.8451 21.2935 21.5693 21.5693C21.2935 21.8451 20.9194 22 20.5294 22H3.47059C3.08056 22 2.70651 21.8451 2.43073 21.5693C2.15494 21.2935 2 20.9194 2 20.5294V3.47059C2 3.08056 2.15494 2.70651 2.43073 2.43073C2.70651 2.15494 3.08056 2 3.47059 2H20.5294C20.9194 2 21.2935 2.15494 21.5693 2.43073C21.8451 2.70651 22 3.08056 22 3.47059ZM7.88235 9.64706H4.94118V19.0588H7.88235V9.64706ZM8.14706 6.41177C8.14861 6.18929 8.10632 5.96869 8.02261 5.76255C7.93891 5.55642 7.81542 5.36879 7.65919 5.21039C7.50297 5.05198 7.31708 4.92589 7.11213 4.83933C6.90718 4.75277 6.68718 4.70742 6.46471 4.70588H6.41177C5.95934 4.70588 5.52544 4.88561 5.20552 5.20552C4.88561 5.52544 4.70588 5.95934 4.70588 6.41177C4.70588 6.86419 4.88561 7.29809 5.20552 7.61801C5.52544 7.93792 5.95934 8.11765 6.41177 8.11765C6.63426 8.12312 6.85565 8.0847 7.06328 8.00458C7.27092 7.92447 7.46074 7.80422 7.62189 7.65072C7.78304 7.49722 7.91237 7.31346 8.00248 7.10996C8.09259 6.90646 8.14172 6.6872 8.14706 6.46471V6.41177ZM19.0588 13.3412C19.0588 10.5118 17.2588 9.41177 15.4706 9.41177C14.8851 9.38245 14.3021 9.50715 13.7799 9.77345C13.2576 10.0397 12.8143 10.4383 12.4941 10.9294H12.4118V9.64706H9.64706V19.0588H12.5882V14.0529C12.5457 13.5403 12.7072 13.0315 13.0376 12.6372C13.3681 12.2429 13.8407 11.9949 14.3529 11.9471H14.4647C15.4 11.9471 16.0941 12.5353 16.0941 14.0176V19.0588H19.0353L19.0588 13.3412Z" fill="currentColor"/>
</svg>
</a>
  <a class="social-link" href="https://twitter.com/ayo_osota/">
  <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 16 16"><path fill="currentColor" d="M12.6.75h2.454l-5.36 6.142L16 15.25h-4.937l-3.867-5.07l-4.425 5.07H.316l5.733-6.57L0 .75h5.063l3.495 4.633L12.601.75Zm-.86 13.028h1.36L4.323 2.145H2.865z"/></svg>
  </a>
  <a class="social-link" href="mailto:osotaayomikun@gmail.com">
  <svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
<path fill-rule="evenodd" clip-rule="evenodd" d="M20 4C21.6569 4 23 5.34315 23 7V17C23 18.6569 21.6569 20 20 20H4C2.34315 20 1 18.6569 1 17V7C1 5.34315 2.34315 4 4 4H20ZM19.2529 6H4.74718L11.3804 11.2367C11.7437 11.5236 12.2563 11.5236 12.6197 11.2367L19.2529 6ZM3 7.1688V17C3 17.5523 3.44772 18 4 18H20C20.5523 18 21 17.5523 21 17V7.16882L13.8589 12.8065C12.769 13.667 11.231 13.667 10.1411 12.8065L3 7.1688Z" fill="currentColor"/>
</svg>
  </a>
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
    max-width: 460px;
}
  .even-columns {
    display: flex;
    align-items: center;
  }
}

.speech-bubble {
  width: 200px;
  height: 100px;
  background-color: #8e44ad;
  border-radius: 0 15% 15% / 0 30% 30%;
  position: relative;
}

.speech-bubble::before {
  content: "";
  position: absolute;
  top: -40px;
  left: 0;
  height: 40px;
  width: 40px;
  border-bottom-left-radius: 50%;
  background-color: transparent;
  box-shadow: 0 20px 0 0 #8e44ad;
}

a {
  color: #FFCC00;
}

a:hover {
  color: #E8D01F;
}
</style>
