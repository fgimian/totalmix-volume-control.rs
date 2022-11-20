// Creates an exponential curve with a given steepness (a) for x values between 0.0 and 1.0.
pub fn curve(x: f32, a: f32) -> f32 {
    // Curve algorithm pulled from the following post:
    // https://math.stackexchange.com/questions/384613/exponential-function-with-values-between-0-and-1-for-x-values-between-0-and-1

    // Values must be greater than 1.0 to work in this algorithm so we assume a linear curve for
    // 1.0 or below.
    if a <= 1.0 {
        return x;
    }

    (a.powf(x) - 1.0) / (a - 1.0)
}
