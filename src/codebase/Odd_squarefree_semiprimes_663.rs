// Rosetta Code task: Odd squarefree semiprimes
// Source: https://rosettacode.org/wiki/Odd_squarefree_semiprimes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//  15  21  33  39  51  57  69  87  93 111 123 129 141 159 177
// 183 201 213 219 237 249 267 291 303 309 321 327 339 381 393
// 411 417 447 453 471 489 501 519 537 543 573 579 591 597 633
// 669 681 687 699 717 723 753 771 789 807 813 831 843 849 879
// 921 933 939 951 993  35  55  65  85  95 115 145 155 185 205
// 215 235 265 295 305 335 355 365 395 415 445 485 505 515 535
// 545 565 635 655 685 695 745 755 785 815 835 865 895 905 955
// 965 985 995  77  91 119 133 161 203 217 259 287 301 329 371
// 413 427 469 497 511 553 581 623 679 707 721 749 763 791 889
// 917 959 973 143 187 209 253 319 341 407 451 473 517 583 649
// 671 737 781 803 869 913 979 221 247 299 377 403 481 533 559
// 611 689 767 793 871 923 949 323 391 493 527 629 697 731 799
// 901 437 551 589 703 779 817 893 667 713 851 943 989 899
// =======================

use std::ops::Div;

fn is_prime(limit: u32) -> impl Fn(&u32) -> bool {
    let mut sieve = vec![true; limit as usize + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=limit.isqrt() {
        if sieve[i as usize] {
            (i * i..=limit).step_by(i as usize).for_each(|j| {
                sieve[j as usize] = false;
            });
        }
    }
    move |n| sieve[*n as usize]
}

fn main() {
    let is_prime = is_prime(333);
    (3..33)
        .filter(|p| is_prime(p))
        .flat_map(|p| {
            (p + 1..=1000_u32.div(p))
                .filter(|q| is_prime(q))
                .map(move |q| p * q)
        })
        .enumerate()
        .for_each(|(idx, value)| {
            if (idx + 1) % 15 == 0 {
                print!("{:3}\n", value);
            } else {
                print!("{:3} ", value);
            }
        });
    println!();
}
