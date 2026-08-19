// Rosetta Code task: P-value correction
// Source: https://rosettacode.org/wiki/P-value_correction#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//
// BenjaminiHochberg:
// [ 0]: 0.6126681081, 0.8521710465, 0.1987205200, 0.1891595417, 0.3217789286
// [ 5]: 0.9301450000, 0.4870370000, 0.9301450000, 0.6049730556, 0.6826752564
// [10]: 0.6482628947, 0.7253722500, 0.5280972727, 0.8769925556, 0.4705703448
// [15]: 0.9241867391, 0.6049730556, 0.7856107317, 0.4887525806, 0.1136717045
// [20]: 0.4991890625, 0.8769925556, 0.9991834000, 0.3217789286, 0.9301450000
// [25]: 0.2304957692, 0.5832475000, 0.0389954722, 0.8521710465, 0.1476842609
// [30]: 0.0168363750, 0.0025629017, 0.0351608437, 0.0625018947, 0.0036365888
// [35]: 0.0025629017, 0.0294688286, 0.0061660636, 0.0389954722, 0.0026889914
// [40]: 0.0004502862, 0.0000125223, 0.0788155476, 0.0314261300, 0.0048465270
// [45]: 0.0025629017, 0.0048465270, 0.0011017083, 0.0725203250, 0.0220595769
//
// BenjaminiYekutieli:
// [ 0]: 1.0000000000, 1.0000000000, 0.8940844244, 0.8510676197, 1.0000000000
// [ 5]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [10]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [15]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 0.5114323399
// [20]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [25]: 1.0000000000, 1.0000000000, 0.1754486368, 1.0000000000, 0.6644618149
// [30]: 0.0757503083, 0.0115310209, 0.1581958559, 0.2812088585, 0.0163617595
// [35]: 0.0115310209, 0.1325863108, 0.0277423864, 0.1754486368, 0.0120983246
// [40]: 0.0020259303, 0.0000563403, 0.3546073326, 0.1413926119, 0.0218055202
// [45]: 0.0115310209, 0.0218055202, 0.0049568120, 0.3262838334, 0.0992505663
//
// Bonferroni:
// [ 0]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [ 5]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [10]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [15]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [20]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [25]: 1.0000000000, 1.0000000000, 0.7019185000, 1.0000000000, 1.0000000000
// [30]: 0.2020365000, 0.0151667450, 0.5625735000, 1.0000000000, 0.0290927100
// [35]: 0.0153774100, 0.4125636000, 0.0678267000, 0.6803480000, 0.0188229400
// [40]: 0.0009005725, 0.0000125223, 1.0000000000, 0.4713919500, 0.0439557650
// [45]: 0.0108891550, 0.0484652700, 0.0033051250, 1.0000000000, 0.2867745000
//
// Hochberg:
// [ 0]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [ 5]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [10]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [15]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [20]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [25]: 0.9991834000, 0.9991834000, 0.4632662100, 0.9991834000, 0.9991834000
// [30]: 0.1575884700, 0.0138396690, 0.3938014500, 0.7600230400, 0.0250197306
// [35]: 0.0138396690, 0.3052970640, 0.0542613600, 0.4626366400, 0.0165641872
// [40]: 0.0008825610, 0.0000125223, 0.9930759000, 0.3394022040, 0.0369228426
// [45]: 0.0102358057, 0.0397415214, 0.0031729200, 0.8992520300, 0.2179486200
//
// Holm:
// [ 0]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [ 5]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [10]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [15]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [20]: 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000, 1.0000000000
// [25]: 1.0000000000, 1.0000000000, 0.4632662100, 1.0000000000, 1.0000000000
// [30]: 0.1575884700, 0.0139534054, 0.3938014500, 0.7600230400, 0.0250197306
// [35]: 0.0139534054, 0.3052970640, 0.0542613600, 0.4626366400, 0.0165641872
// [40]: 0.0008825610, 0.0000125223, 0.9930759000, 0.3394022040, 0.0369228426
// [45]: 0.0102358057, 0.0397415214, 0.0031729200, 0.8992520300, 0.2179486200
//
// Sidak:
// [ 0]: 1.0000000000, 1.0000000000, 0.9946598274, 0.9914285749, 0.9999515274
// [ 5]: 1.0000000000, 0.9999999688, 1.0000000000, 1.0000000000, 1.0000000000
// [10]: 1.0000000000, 1.0000000000, 0.9999999995, 1.0000000000, 0.9999998801
// [15]: 1.0000000000, 1.0000000000, 1.0000000000, 0.9999999855, 0.9231179729
// [20]: 0.9999999956, 1.0000000000, 1.0000000000, 0.9999317605, 1.0000000000
// [25]: 0.9983109511, 1.0000000000, 0.5068253940, 1.0000000000, 0.9703301333
// [30]: 0.1832692440, 0.0150545753, 0.4320729669, 0.6993672225, 0.0286818157
// [35]: 0.0152621104, 0.3391808707, 0.0656206307, 0.4959194266, 0.0186503726
// [40]: 0.0009001752, 0.0000125222, 0.8142104886, 0.3772612062, 0.0430222116
// [45]: 0.0108312558, 0.0473319661, 0.0032997780, 0.7705015898, 0.2499384839
//
// Hommel:
// [ 0]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9987623800, 0.9991834000
// [ 5]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [10]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [15]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9595180000
// [20]: 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000, 0.9991834000
// [25]: 0.9991834000, 0.9991834000, 0.4351894700, 0.9991834000, 0.9766522500
// [30]: 0.1414255500, 0.0130434007, 0.3530936533, 0.6887708800, 0.0238560222
// [35]: 0.0132245726, 0.2722919760, 0.0542613600, 0.4218157600, 0.0158112696
// [40]: 0.0008825610, 0.0000125223, 0.8743649143, 0.3016908480, 0.0351646120
// [45]: 0.0095824564, 0.0387722160, 0.0031729200, 0.8122276400, 0.1950066600
// =======================

use std::iter;

#[rustfmt::skip]
const PVALUES:[f64;50] = [
4.533_744e-01, 7.296_024e-01, 9.936_026e-02, 9.079_658e-02, 1.801_962e-01,
8.752_257e-01, 2.922_222e-01, 9.115_421e-01, 4.355_806e-01, 5.324_867e-01,
4.926_798e-01, 5.802_978e-01, 3.485_442e-01, 7.883_130e-01, 2.729_308e-01,
8.502_518e-01, 4.268_138e-01, 6.442_008e-01, 3.030_266e-01, 5.001_555e-02,
3.194_810e-01, 7.892_933e-01, 9.991_834e-01, 1.745_691e-01, 9.037_516e-01,
1.198_578e-01, 3.966_083e-01, 1.403_837e-02, 7.328_671e-01, 6.793_476e-02,
4.040_730e-03, 3.033_349e-04, 1.125_147e-02, 2.375_072e-02, 5.818_542e-04,
3.075_482e-04, 8.251_272e-03, 1.356_534e-03, 1.360_696e-02, 3.764_588e-04,
1.801_145e-05, 2.504_456e-07, 3.310_253e-02, 9.427_839e-03, 8.791_153e-04,
2.177_831e-04, 9.693_054e-04, 6.610_250e-05, 2.900_813e-02, 5.735_490e-03
];

#[derive(Debug)]
enum CorrectionType {
    BenjaminiHochberg,
    BenjaminiYekutieli,
    Bonferroni,
    Hochberg,
    Holm,
    Hommel,
    Sidak,
}

enum SortDirection {
    Increasing,
    Decreasing,
}

/// orders **input** vector by value and multiplies with **multiplier** vector
/// Finally returns the multiplied values in the original order of **input**
fn ordered_multiply(input: &[f64], multiplier: &[f64], direction: &SortDirection) -> Vec<f64> {
    let order_by_value = match direction {
        SortDirection::Increasing => {
            |a: &(f64, usize), b: &(f64, usize)| b.0.partial_cmp(&a.0).unwrap()
        }
        SortDirection::Decreasing => {
            |a: &(f64, usize), b: &(f64, usize)| a.0.partial_cmp(&b.0).unwrap()
        }
    };

    let cmp_minmax = match direction {
        SortDirection::Increasing => |a: f64, b: f64| a.gt(&b),
        SortDirection::Decreasing => |a: f64, b: f64| a.lt(&b),
    };

    // add original order index
    let mut input_indexed = input
        .iter()
        .enumerate()
        .map(|(idx, &p_value)| (p_value, idx))
        .collect::<Vec<_>>();

    // order by value desc/asc
    input_indexed.sort_unstable_by(order_by_value);

    // do the multiplication in place, clamp it at 1.0,
    // keep the original index in place
    for i in 0..input_indexed.len() {
        input_indexed[i] = (
            f64::min(1.0, input_indexed[i].0 * multiplier[i]),
            input_indexed[i].1,
        );
    }

    // make vector strictly monotonous increasing/decreasing in place
    for i in 1..input_indexed.len() {
        if cmp_minmax(input_indexed[i].0, input_indexed[i - 1].0) {
            input_indexed[i] = (input_indexed[i - 1].0, input_indexed[i].1);
        }
    }

    // re-sort back to original order
    input_indexed.sort_unstable_by(|a: &(f64, usize), b: &(f64, usize)| a.1.cmp(&b.1));

    // remove ordering index
    let (resorted, _): (Vec<_>, Vec<_>) = input_indexed.iter().cloned().unzip();
    resorted
}

#[allow(clippy::cast_precision_loss)]
fn hommel(input: &[f64]) -> Vec<f64> {
    // using algorith described:
    // http://stat.wharton.upenn.edu/~steele/Courses/956/ResourceDetails/MultipleComparision/Writght92.pdf

    // add original order index
    let mut input_indexed = input
        .iter()
        .enumerate()
        .map(|(idx, &p_value)| (p_value, idx))
        .collect::<Vec<_>>();

    // order by value asc
    input_indexed
        .sort_unstable_by(|a: &(f64, usize), b: &(f64, usize)| a.0.partial_cmp(&b.0).unwrap());

    let (p_values, order): (Vec<_>, Vec<_>) = input_indexed.iter().cloned().unzip();

    let n = input.len();

    // initial minimal n*p/i values
    // get the smalles of these values
    let min_result = (0..n)
        .map(|i| ((p_values[i] * n as f64) / (i + 1) as f64))
        .fold(1. / 0. /* -inf */, f64::min);

    // // initialize result vector with minimal values
    let mut result = iter::repeat(min_result).take(n).collect::<Vec<_>>();

    for m in (2..n).rev() {
        let cmin: f64;
        let m_as_float = m as f64;
        let mut a = p_values.clone();
        // println!("\nn: {}", m);
        {
            // split p-values into two group
            let (_, second) = p_values.split_at(n - m + 1);

            // calculate minumum of m*p/i for this second group
            cmin = second
                .iter()
                .zip(2..=m)
                .map(|(p, i)| (m_as_float * p) / i as f64)
                .fold(1. / 0. /* inf */, f64::min);
        }

        // replace p values if p<cmin in the second group
        ((n - m + 1)..n).for_each(|i| a[i] = a[i].max(cmin));

        // replace p values if min(cmin, m*p) > p
        (0..=(n - m)).for_each(|i| a[i] = a[i].max(f64::min(cmin, m_as_float * p_values[i])));

        // store in the result vector if any adjusted p is higher than the current one
        (0..n).for_each(|i| result[i] = result[i].max(a[i]));
    }

    // re-sort into the original order
    let mut result = result
        .into_iter()
        .zip(order.into_iter())
        .map(|(p, idx)| (p, idx))
        .collect::<Vec<_>>();
    result.sort_unstable_by(|a: &(f64, usize), b: &(f64, usize)| a.1.cmp(&b.1));
    let (result, _): (Vec<_>, Vec<_>) = result.iter().cloned().unzip();
    result
}
#[allow(clippy::cast_precision_loss)]
fn p_value_correction(p_values: &[f64], ctype: &CorrectionType) -> Vec<f64> {
    let p_vec = p_values.to_vec();
    if p_values.is_empty() {
        return p_vec;
    }

    let fsize = p_values.len() as f64;

    match ctype {
        CorrectionType::BenjaminiHochberg => {
            let multiplier = (0..p_values.len())
                .map(|index| fsize / (fsize - index as f64))
                .collect::<Vec<_>>();

            ordered_multiply(&p_vec, &multiplier, &SortDirection::Increasing)
        }
        CorrectionType::BenjaminiYekutieli => {
            let q: f64 = (1..=p_values.len()).map(|index| 1. / index as f64).sum();
            let multiplier = (0..p_values.len())
                .map(|index| q * fsize / (fsize - index as f64))
                .collect::<Vec<_>>();

            ordered_multiply(&p_vec, &multiplier, &SortDirection::Increasing)
        }
        CorrectionType::Bonferroni => p_vec
            .iter()
            .map(|p| f64::min(p * fsize, 1.0))
            .collect::<Vec<_>>(),
        CorrectionType::Hochberg => {
            let multiplier = (0..p_values.len())
                .map(|index| 1. + index as f64)
                .collect::<Vec<_>>();
            ordered_multiply(&p_vec, &multiplier, &SortDirection::Increasing)
        }
        CorrectionType::Holm => {
            let multiplier = (0..p_values.len())
                .map(|index| fsize - index as f64)
                .collect::<Vec<_>>();

            ordered_multiply(&p_vec, &multiplier, &SortDirection::Decreasing)
        }
        CorrectionType::Sidak => p_vec
            .iter()
            .map(|x| 1. - (1. - x).powf(fsize))
            .collect::<Vec<_>>(),
        CorrectionType::Hommel => hommel(&p_vec),
    }
}

// prints array into a nice table, max 5 floats/row
fn array_to_string(a: &[f64]) -> String {
    a.chunks(5)
        .enumerate()
        .map(|(index, e)| {
            format!(
                "[{:>2}]: {}",
                index * 5,
                e.iter()
                    .map(|x| format!("{:>1.10}", x))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
fn main() {
    let ctypes = [
        CorrectionType::BenjaminiHochberg,
        CorrectionType::BenjaminiYekutieli,
        CorrectionType::Bonferroni,
        CorrectionType::Hochberg,
        CorrectionType::Holm,
        CorrectionType::Sidak,
        CorrectionType::Hommel,
    ];

    for ctype in &ctypes {
        println!("\n{:?}:", ctype);
        println!("{}", array_to_string(&p_value_correction(&PVALUES, ctype)));
    }
}
