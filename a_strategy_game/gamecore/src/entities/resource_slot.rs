use crate::prelude::*;

/// An factory input or output slot. Can hold up to some maximum amount of resources.
/// E.g. `Leaf: 7 out of 100`
#[derive(Serialize, Deserialize)]
pub struct ResourceSlot {
    /// Type of Resource stored. E.g. Leaf, Rock.
    pub typ: ResourceTyp,
    /// Current amount stored. Always <= max.
    pub amount: Cel<u16>,
    /// Maximum amount stored.
    pub max: u16,
}

impl ResourceSlot {
    pub fn new(typ: ResourceTyp, max: u16) -> Self {
        debug_assert!(max > 0);
        Self { typ, max, amount: 0.cel() }
    }

    pub fn is_full(&self) -> bool {
        self.amount() >= self.max
    }

    pub fn fullness_pct(&self) -> u32 {
        (self.amount() as u32 * 100) / (self.max as u32)
    }

    pub fn is_empty(&self) -> bool {
        self.amount() == 0
    }

    /// Try to take one resource, return it if successful or None otherwise (slot was empty).
    pub fn try_take_one(&self) -> Option<ResourceTyp> {
        if self.amount() > 0 {
            self.amount.sub(1);
            Some(self.typ)
        } else {
            None
        }
    }

    /// Slot has at least `n` items. So we can successfully `take(n)`.
    pub fn has_at_least(&self, n: u16) -> bool {
        self.amount() >= n
    }

    pub fn take(&self, n: u16) -> Option<()> {
        debug_assert!(self.has_at_least(n));
        if self.has_at_least(n) {
            self.amount.sub(n);
            OK
        } else {
            FAIL
        }
    }

    pub fn can_accept(&self, n: u16) -> bool {
        self.amount() + n <= self.max
    }

    pub fn add_one(&self) -> Option<()> {
        if self.amount() < self.max {
            self.amount.inc(1);
            OK
        } else {
            FAIL
        }
    }

    #[inline]
    pub fn amount(&self) -> u16 {
        self.amount.get()
    }

    /// For internal use/debug only.
    pub(crate) fn get_amount(&self) -> &Cel<u16> {
        &self.amount
    }
}
