use crate::point::{Point2, Point3};
use crate::vector::{Vec2, Vec3, Vec4, InnerSpace};

// ##################################################
// #                   CONSTANTS                    #
// ##################################################

static facVals: [i64; 21] = [
    1,1,2,6,24,120,720,5040,40320,362880,3628800,
    39916800,479001600,6227020800,87178291200,1307674368000,
    20922789888000,3556877428096000,6402373705728000,
    121645100408832000,2432902008176640000
]; 

static invFac: [f32; 6] = [
    1.0,1.0,0.5,0.1666666666666666667,0.04166666666666666667,0.00833333333333333334
];

static PI: f32 = std::f32::consts::PI;

// ##################################################
// #                GENERAL PURPOSE                 #
// ##################################################

pub fn fac(n: i64) -> i64 {
    match n {
        0 => 1,
        _ => n * fac(n-1)
    }
}

pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

pub fn ln(x: f32) -> f32 {
    x.ln()
}

pub fn log(x: f32) -> f32 {
    ln(x)/2.30258509299
}

pub fn log2(x: f32) -> f32 {
    ln(x)/0.69314718056
}

pub fn sin(x: f32) -> f32 {
    x.sin()
}

pub fn asin(x: f32) -> f32 {
    x.asin()
}

pub fn cos(x: f32) -> f32 {
    x.cos()
}

pub fn acos(x: f32) -> f32 {
    x.acos()
}

pub fn tan(x: f32) -> f32 {
    x.tan()
}

pub fn atan(x: f32) -> f32 {
    x.atan()
}

pub fn atan2(p: Point2) -> f32 {
    p.y.atan2(p.x)
}

pub fn sinh(x: f32) -> f32 {
    x.sinh()
}

pub fn cosh(x: f32) -> f32 {
    x.cosh()
}

pub fn tanh(x: f32) -> f32 {
    x.tanh()
}

pub fn clamp(start: f32, end: f32, value: f32) -> f32 {
    match value {
        _ if value > end => end,
        _ if start > value => start,
        _ => value
    }
}

pub fn pointDerivative(func: fn(f32) -> f32, x: f32, h: f32) -> f32 {
    (func(x+h) - func(x-h))/(2.0 * h)
}

// ##################################################
// #                 INTERPOLATION                  #
// ##################################################

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

pub fn invLerp(a: f32, b:f32, value: f32) -> f32 {
    (value - a) / (b - a)
}

pub fn lerp2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a * (1.0 - t) + b * t
}

pub fn invLerp2(a: Vec2, b: Vec2, value: Vec2) -> Option<f32> {
    let tx = (value.x - a.x) / (b.x - a.x);
    let ty = (value.y - a.y) / (b.y - a.y);

    if tx == ty {
        return Some(tx);
    }
    None
}

pub fn lerp3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a * (1.0 - t) + b * t
}

pub fn invLerp3(a: Vec3, b: Vec3, value: Vec3) -> Option<f32> {
    let tx = (value.x - a.x)/(b.x - a.x);
    let ty = (value.y - a.y)/(b.y - a.y);
    let tz = (value.z - a.z)/(b.z - a.z);

    if (tx == ty) && (ty == tz) {
        return Some(tx);
    }
    None
}

// ##################################################
// #                  BEZIER CURVES                 #
// ##################################################

/// Cubic Bézier Curve in R² 
pub fn bezier2(p0: Point2, p1: Point2, p2: Point2, p3: Point2, t: f32) -> Point2 {
    let tSquared = t * t;
    let tCubed = tSquared * t;
    let vP0 = Vec2::fromPoint(p0);
    let vP1 = Vec2::fromPoint(p1);
    let vP2 = Vec2::fromPoint(p2);
    let vP3 = Vec2::fromPoint(p3);

    Point2::fromVec(vP0 * (-tCubed + 3.0 * tSquared - 3.0 * t + 1.0 ) +
                    vP1 * (3.0 * tCubed - 6.0 * tSquared + 3.0 * t ) +
                    vP2 * (-3.0 * tCubed + 3.0 * tSquared ) + 
                    vP3 * tCubed)
} 

/// Cubic Bézier Curve in R³
pub fn bezier3(p0: Point3, p1: Point3, p2: Point3, p3: Point3, t: f32) -> Point3 {
    let tSquared = t * t;
    let tCubed = tSquared * t;
    let vP0 = Vec3::fromPoint(p0);
    let vP1 = Vec3::fromPoint(p1);
    let vP2 = Vec3::fromPoint(p2);
    let vP3 = Vec3::fromPoint(p3);

    Point3::fromVec(vP0 * (-tCubed + 3.0 * tSquared - 3.0 * t + 1.0 ) +
                    vP1 * (3.0 * tCubed - 6.0 * tSquared + 3.0 * t ) +
                    vP2 * (-3.0 * tCubed + 3.0 * tSquared ) + 
                    vP3 * tCubed)
}

// ##################################################
// #                    SPLINES                     #
// ##################################################


// ##################################################
// #                EASING FUNCTIONS                #
// ##################################################

pub fn easeInSine(x: f32) -> f32 {
    1.0 - cos((x * PI) / 2.0)
}

pub fn easeOutSine(x: f32) -> f32 {
    sin((x * PI) / 2.0)
}

pub fn easeInOutSine(x: f32) -> f32 {
    -(cos(PI * x) - 1.0) / 2.0
}

pub fn easeInQuad(x: f32) -> f32 {
    x * x
}

pub fn easeOutQuad(x: f32) -> f32 {
    1.0 - (1.0 - x) * (1.0 - x)
}

pub fn easeInOutQuad(x: f32) -> f32 {
    if x < 0.5 { 2.0 * x * x } else { 1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0 }
}

pub fn easeInCubic(x: f32) -> f32 {
    x * x * x
}

pub fn easeOutCubic(x: f32) -> f32 {
    1.0 - (1.0 - x).powf(3.0)
}

pub fn easeInOutCubic(x: f32) -> f32 {
    if x < 0.5 { 4.0 * x * x * x } else { 1.0 - (-2.0 * x + 2.0).powf(3.0) / 2.0 }
}

pub fn easeInQuart(x: f32) -> f32 {
    x * x * x * x
}

pub fn easeOutQuart(x: f32) -> f32 {
    1.0 - (1.0 - x).powf(4.0)
}

pub fn easeInOutQuart(x: f32) -> f32 {
    if x < 0.5 { 8.0 * x * x * x * x } else { 1.0 - (-2.0 * x + 2.0).powf(4.0) / 2.0 }
}

pub fn easeInQuint(x: f32) -> f32 {
    x * x * x * x * x
}

pub fn easeOutQuint(x: f32) -> f32 {
    1.0 - (1.0 - x).powf(5.0)
}

pub fn easeInOutQuint(x: f32) -> f32 {
    if x < 0.5 { 16.0 * x * x * x * x * x } else { 1.0 - (-2.0 * x + 2.0).powf(5.0) / 2.0 }
}

pub fn easeInExpo(x: f32) -> f32 {
    if x == 0.0 { 0.0 } else { 2.0_f32.powf(10.0 * x - 10.0) }
}

pub fn easeOutExpo(x: f32) -> f32 {
    if x == 1.0 { 1.0 } else { 1.0 - 2.0_f32.powf(-10.0 * x) }
}

pub fn easeInOutExpo(x: f32) -> f32 {
    if x == 0.0 { 0.0 } else if x == 1.0 { 1.0 } else if x < 0.5 { 2.0_f32.powf(20.0 * x - 10.0) / 2.0 } else { (2.0 - 2.0_f32.powf(-20.0 * x + 10.0)) / 2.0 }
}

pub fn easeInCirc(x: f32) -> f32 {
    1.0 - sqrt(1.0 - x * x)
}

pub fn easeOutCirc(x: f32) -> f32 {
    sqrt(1.0 - (x - 1.0).powf(2.0))
}

pub fn easeInOutCirc(x: f32) -> f32 {
    if x < 0.5 { sqrt(1.0 - (1.0 - 2.0 * x).powf(2.0)) / 2.0 } else { (sqrt(1.0 - (-2.0 * x + 2.0).powf(2.0)) + 1.0) / 2.0 }
}

pub fn easeInBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    c3 * x * x * x - c1 * x * x
}

pub fn easeOutBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (x - 1.0).powf(3.0) + c1 * (x - 1.0).powf(2.0)
}

pub fn easeInOutBack(x: f32) -> f32 {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;
    if x < 0.5 { (2.0 * x).powf(2.0) * ((c2 + 1.0) * 2.0 * x - c2) / 2.0 } else { ((2.0 * x - 2.0).powf(2.0) * ((c2 + 1.0) * (2.0 * x - 2.0) + c2) + 2.0) / 2.0 }
}

pub fn easeInElastic(x: f32) -> f32 {
    let c4 = (2.0 * PI) / 3.0;
    if x == 0.0 { 0.0 } else if x == 1.0 { 1.0 } else { -2.0_f32.powf(10.0 * x - 10.0) * sin((x * 10.0 - 10.75) * c4) }
}

pub fn easeOutElastic(x: f32) -> f32 {
    let c4 = (2.0 * PI) / 3.0;
    if x == 0.0 { 0.0 } else if x == 1.0 { 1.0 } else { 2.0_f32.powf(-10.0 * x) * sin((x * 10.0 - 0.75) * c4) + 1.0 }
}

pub fn easeInOutElastic(x: f32) -> f32 {
    let c5 = (2.0 * PI) / 4.5;
    if x == 0.0 { 0.0 } else if x == 1.0 { 1.0 } else if x < 0.5 { -(2.0_f32.powf(20.0 * x - 10.0) * sin((20.0 * x - 11.125) * c5)) / 2.0 } else { (2.0_f32.powf(-20.0 * x + 10.0) * sin((20.0 * x - 11.125) * c5)) / 2.0 + 1.0 }
}

pub fn easeInBounce(x: f32) -> f32 {
    1.0 - easeOutBounce(1.0 - x)
}

pub fn easeOutBounce(x: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if x < 1.0 / d1 { n1 * x * x } else if x < 2.0 / d1 { n1 * (x - 1.5 / d1) * (x - 1.5 / d1) + 0.75 } else if x < 2.5 / d1 { n1 * (x - 2.25 / d1) * (x - 2.25 / d1) + 0.9375 } else { n1 * (x - 2.625 / d1) * (x - 2.625 / d1) + 0.984375 }
}

pub fn easeInOutBounce(x: f32) -> f32 {
    if x < 0.5 { (1.0 - easeOutBounce(1.0 - 2.0 * x)) / 2.0 } else { (1.0 + easeOutBounce(2.0 * x - 1.0)) / 2.0 }
}
