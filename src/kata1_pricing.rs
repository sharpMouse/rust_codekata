#![cfg(test)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

const STD_DENOMINATOR: u64 = 100;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
struct Money {
    value: u64,
}

impl Money {
    fn new(value: u64) -> Money {
        Money {
            value: value * STD_DENOMINATOR,
        }
    }

    fn raw(value: u64) -> Money {
        Money {
            value,
        }
    }
}

impl std::ops::AddAssign<Money> for Money {
    fn add_assign(&mut self, rhs: Money) {
        self.value += rhs.value;
    }
}

impl<'a> std::iter::Sum<&'a&'a Money> for Money {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = &'a&'a Money> {
        Money::raw(iter.map(|x| x.value).sum())
    }
}

impl std::ops::Mul<usize> for Money {
    type Output = Money;

    fn mul(self, rhs: usize) -> Money {
        let value = self.value * u64::try_from(rhs).ok().unwrap();
        Money::raw(value)
    }
}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value / STD_DENOMINATOR)?;
        let rest = self.value % STD_DENOMINATOR;
        if rest != 0 {
            write!(f, ".{:02}", rest)
        } else {
            write!(f, "")
        }
    }
}

#[test]
fn check_money() {
    assert_eq!(Money::new(100).to_string(), "100");
    assert_eq!(Money::raw(199).to_string(), "1.99");
    assert_eq!(Money::raw(5).to_string(), "0.05");
    assert_eq!([&Money::new(2), &Money::new(4)].iter().sum::<Money>(), Money::new(6));
}

///////////////////////////////////////////////////////////

#[derive(Eq, PartialEq, Hash)]
enum PricingStrategy {
    Add,
    Nth { count: usize, total: Money },
    OneFree { count: usize },
}

fn calculate_ps_add(moneys: &[&Money]) -> Money {
    moneys.iter().sum()
}

fn calculate_ps_nth(moneys: &[&Money], count: usize, total: &Money) -> Money {
    let nth_count = moneys.len() / count;
    let rest_index = nth_count * count;
    let mut result = *total * nth_count;
    result += moneys[rest_index..].iter().sum();
    result
}

fn calculate_ps_onefree(moneys: &[&Money], count: usize) -> Money {
    let free_count = moneys.len() / count;
    let mut money_copy = Vec::from(moneys);
    money_copy.select_nth_unstable(free_count);
    
    // Skip free pieces and calc the others
    money_copy[free_count..].iter().sum()
}

impl PricingStrategy {
    fn calculate(&self, moneys: &[&Money]) -> Money {
        match self {
            PricingStrategy::Add => calculate_ps_add(moneys),
            PricingStrategy::Nth { count, total } => calculate_ps_nth(moneys, *count, total),
            PricingStrategy::OneFree { count } => calculate_ps_onefree(moneys, *count),
        }
    }
}

#[test]
fn check_add() {
    let m1 = Money::new(10);
    let m2 = Money::new(15);
    let strat = PricingStrategy::Add;
    let total = strat.calculate(vec![&m1, &m2].as_slice());
    assert_eq!(total.to_string(), "25");
}

#[test]
fn check_nth() {
    let m1 = Money::new(10);
    let m2 = Money::new(10);
    let m3 = Money::new(10);
    let strat = PricingStrategy::Nth {
        count: 3,
        total: Money::new(20),
    };
    let total = strat.calculate(&[&m1, &m2, &m3]);
    assert_eq!(total.to_string(), "20");

    let total = strat.calculate(&[&m1; 3]);
    assert_eq!(total.to_string(), "20");
    let total = strat.calculate(&[&m1; 4]);
    assert_eq!(total.to_string(), "30");
    let total = strat.calculate(&[&m1; 5]);
    assert_eq!(total.to_string(), "40");
    let total = strat.calculate(&[&m1; 6]);
    assert_eq!(total.to_string(), "40");
    let total = strat.calculate(&[&m1; 7]);
    assert_eq!(total.to_string(), "50");
}

#[test]
fn check_onefree() {
    let m1 = Money::new(1);
    let m2 = Money::new(2);
    let m3 = Money::new(4);
    let strat = PricingStrategy::OneFree { count: 3 };
    let total = strat.calculate(&[&m1, &m2, &m3]);
    assert_eq!(total.to_string(), "6");

    let total = strat.calculate(&[&m1; 3]);
    assert_eq!(total.to_string(), "2");
    let total = strat.calculate(&[&m1; 4]);
    assert_eq!(total.to_string(), "3");
    let total = strat.calculate(&[&m1; 5]);
    assert_eq!(total.to_string(), "4");

    let m4 = Money::new(8);
    let total = strat.calculate(&[&m4, &m1, &m2, &m3]);
    assert_eq!(total.to_string(), "14");

    let total = strat.calculate(&[&m4, &m1, &m2, &m3, &m2, &m4]);
    assert_eq!(total.to_string(), "22");
    let total = strat.calculate(&[&m4, &m1, &m2, &m3, &m2, &m4, &m3]);
    assert_eq!(total.to_string(), "26");
}

/////////////////////////////////////////////////////////

struct Price {
    cost: Money,
    strategy: PricingStrategy,
}

impl Price {
    fn new(value: u64) -> Price {
        Price {
            cost: Money::new(value),
            strategy: PricingStrategy::Add,
        }
    }

    fn new_nth(value: u64, count: usize, total: u64) -> Price {
        Price {
            cost: Money::new(value),
            strategy: PricingStrategy::Nth{ count, total: Money::new(total) },
        }
    }

    fn new_onefree(value: u64, count: usize) -> Price {
        Price {
            cost: Money::new(value),
            strategy: PricingStrategy::OneFree{ count },
        }
    }
}

fn calculate_total_price(prices: &[&Price]) -> Money {
    let mut price_per_strat = HashMap::new();
    for &price in prices {
        let vc = &mut price_per_strat.entry(&price.strategy).or_insert(Vec::new());
        vc.push(&price.cost);
    }

    let mut result = Money::raw(0);
    for (strat, moneys) in &price_per_strat {
        result += strat.calculate(&moneys[..]);
    }
    result
}

#[test]
fn check_total() {
    let p1 = Price::new(1);
    let p2 = Price::new(2);

    let total = calculate_total_price(&[&p1, &p2]);
    assert_eq!(total.to_string(), "3");

    let p3 = Price::new_nth(100, 2, 4);
    let p4 = Price::new_nth(200, 2, 4);

    let total = calculate_total_price(&[&p1, &p3, &p2, &p4]);
    assert_eq!(total.to_string(), "7");

    let p5 = Price::new_onefree(8, 2);
    let p6 = Price::new_onefree(16, 2);

    let total = calculate_total_price(&[&p1, &p3, &p5, &p2, &p4, &p6]);
    assert_eq!(total.to_string(), "23");
}
