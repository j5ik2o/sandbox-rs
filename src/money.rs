use bigdecimal::{BigDecimal, One, Zero};
use std::ops::{Add, Mul, Div, Rem, Sub};
use std::cmp::Ordering;

use std::fmt::Formatter;
use crate::currency::Currency;

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
  currency: Currency,
  amount: BigDecimal,
}

impl std::fmt::Display for Money {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "(currency = {}, amount = {:>.0})",
      self.currency, self.amount
    )
  }
}

impl Money {
  pub fn one(currency: Currency) -> Self {
    Self::of(BigDecimal::one(), currency)
  }

  pub fn zero(currency: Currency) -> Self {
    Self::of(BigDecimal::zero(), currency)
  }

  pub fn negated(self) -> Self {
    Self::of(self.amount.mul(BigDecimal::from(-1)), self.currency)
  }

  pub fn abs(self) -> Self {
    Self::of(self.amount.abs(), self.currency)
  }

  pub fn rem(self, n: BigDecimal) -> Self {
    Self::of(self.amount.rem(n), self.currency)
  }

  pub fn is_zero(&self) -> bool {
    self.amount.is_zero()
  }

  pub fn is_non_zero(&self) -> bool {
    !self.is_zero()
  }
}

impl Add for Money {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    assert_eq!(self.currency, rhs.currency);
    let result = self.amount.add(rhs.amount);
    Self::of(result, rhs.currency)
  }
}

impl Sub for Money {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    self + rhs.negated()
  }
}

impl Mul<BigDecimal> for Money {
  type Output = Self;

  fn mul(self, rhs: BigDecimal) -> Self::Output {
    Self::of(self.amount.mul(rhs), self.currency)
  }
}

impl Div<BigDecimal> for Money {
  type Output = Self;

  fn div(self, rhs: BigDecimal) -> Self::Output {
    Self::of(self.amount.div(rhs), self.currency)
  }
}

impl PartialOrd for Money {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    assert_eq!(self.currency, other.currency);
    self.amount.partial_cmp(&other.amount)
  }
}

pub trait Factories<T> {
  fn of(_: T, _: Currency) -> Self;
}

impl Factories<i64> for Money {
  fn of(amount: i64, currency: Currency) -> Self {
    Money {
      amount: BigDecimal::from(amount),
      currency,
    }
  }
}

impl Factories<BigDecimal> for Money {
  fn of(amount: BigDecimal, currency: Currency) -> Self {
    Money { amount, currency }
  }
}
