//! Defines the operations and data definitions for a top bar program.

use std::ops::Deref;
use rustwlc::WlcView;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bar {
    view: WlcView
}

impl Bar {
    pub fn new(view: WlcView) -> Self {
        Bar { view: view }
    }

    /// Gets the view that is associated with the bar.
    pub fn view(&self) -> WlcView {
        self.view
    }
}


impl Deref for Bar {
    type Target = WlcView;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}
