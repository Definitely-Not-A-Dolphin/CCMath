#set page(
  height: auto,
)

#let inv(n) = math.frac([1],n);

= Multiply

$
(a+b i)(c + d i)=\
a c + a d i + b c i + b d i^2=\
a c + (a d + b c)i - b d=\
a c - b d + (a d + b c)i
$

= Divide
$
frac(a + b i, c + d i)=\
frac((a + b i)(c - d i), (c + d i)(c - d i))=\
frac(a c + b d + (b c - a d)i, c^2 - d^2)=\
frac(a c + b d, c^2 - d^2) + frac(b c - a d, c^2 - d^2)i=\
$

= Square root
If $b$ is positive:
$
sqrt(a + b i)=sqrt(frac(a+sqrt(a^2+b^2),2))+sqrt(frac(-a+sqrt(a^2+b^2),2))i
$

If $b$ is negative:
$
sqrt(a + b i)=sqrt(frac(a+sqrt(a^2+b^2),2))-sqrt(frac(-a+sqrt(a^2+b^2),2))i
$

= Inverse
$
inv(a + b i) =\
frac(a - b i, a^2 + b^2)=\
frac(a, a^2 + b^2) - frac(b, a^2 + b^2) i
$

= Natural Logarithm
$
ln(z) =\
ln(|z|e^(i theta))=\
ln|z|+ln(e^(i theta))=\
ln|z|+i theta
$

= Exponentiation

$
z_1^(z_2)=\
z_1^(c + d i)=\
z_1^(c) z_1^(d i)=\
z_1^(c) e^(ln(z_1)d i)=\
$

= Trig

$
sin(a+b i)&=sin(a) cosh(b) + cos(a) sinh(b) i\
cos(a + b i)&=cos(a) cosh(b) - sin(a) sinh(b) i\
$

= Inverse Trig

$
arcsin(z)&=-i ln(sqrt(1-z^2) + i z)\
arccos(z)&=-i ln(sqrt(z^2-1) + z)\
$

= Hyperbolic Trig

$
sinh(a + b i)&=sinh(a)cos(b)+cosh(a)sin(b)i\
cosh(a + b i)&=cosh(a)cos(b)+sinh(a)sin(b)i
$
