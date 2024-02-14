use std::{f64::consts::PI, ops::Range};

use crate::{
    fft::{evaluation_point, XyScatter},
    utils::norm,
};

pub trait FiltreTrait {
    fn new(h0: f64, f0: f64, q: f64) -> Self;
    fn gain_graph(&self, n: usize, width: Range<f64>) -> XyScatter {
        evaluation_point(n, width, &Box::new(move |f| self.gain_at(f)))
    }
    fn phase_graph(&self, n: usize, width: Range<f64>) -> XyScatter {
        evaluation_point(n, width, &Box::new(move |f| self.phase_at(f)))
    }

    fn gain_at(&self, f: f64) -> f64;
    fn phase_at(&self, f: f64) -> f64;
}

pub struct FiltrePasseHaut {
    h0: f64,
    /// pulsation de coupure
    omega0: f64,
    q: f64,
}

impl FiltreTrait for FiltrePasseHaut {
    fn new(h0: f64, f0: f64, q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q,
        }
    }

    fn gain_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        (self.h0.abs() * x.powi(2)) / norm(1.0 - x.powi(2), x * (1.0 / self.q))
    }

    fn phase_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        PI - if pulsation < self.omega0 {
            -(x * (1.0 / self.q) / (1.0 - x)).atan()
        } else if pulsation > self.omega0 {
            -PI - (x * (1.0 / self.q) / (1.0 - x)).atan()
        } else if pulsation == self.omega0 {
            -PI / 2.0
        } else {
            unreachable!()
        }
    }
}

pub struct FiltrePasseBas {
    h0: f64,
    /// pulsation de coupure
    omega0: f64,
    q: f64,
}
impl FiltreTrait for FiltrePasseBas {
    fn new(h0: f64, f0: f64, q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q,
        }
    }

    fn gain_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        self.h0 / norm(1.0 - x.powi(2), x * (1.0 / self.q))
    }

    fn phase_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        if pulsation < self.omega0 {
            -(x * (1.0 / self.q) / (1.0 - x)).atan()
        } else if pulsation > self.omega0 {
            -PI - (x * (1.0 / self.q) / (1.0 - x)).atan()
        } else if pulsation == self.omega0 {
            -PI / 2.0
        } else {
            unreachable!()
        }
    }
}

pub struct FiltrePasseBande {
    h0: f64,
    /// pulsation non coupé
    omega0: f64,
    q: f64,
}
impl FiltrePasseBande {
    /// bp: bande passante
    pub fn q_from_bp(h0: f64, f0: f64, bp: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q: 2.0 * PI * f0 / bp,
        }
    }
}
impl FiltreTrait for FiltrePasseBande {
    fn new(h0: f64, f0: f64, q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q,
        }
    }

    fn gain_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        self.h0 / norm(1.0, self.q * (x - (1.0 / x)))
    }

    fn phase_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        -(self.q * (x - (1.0 / x))).atan()
    }
}

pub struct FiltreRejecteur {
    h0: f64,
    /// pulsation rejetté
    omega0: f64,
    q: f64,
}
impl FiltreRejecteur {
    /// bp: bande passante
    pub fn q_from_bp(h0: f64, f0: f64, bp: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q: 2.0 * PI * f0 / bp,
        }
    }
}
impl FiltreTrait for FiltreRejecteur {
    fn new(h0: f64, f0: f64, q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q,
        }
    }

    fn gain_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        (self.h0 * (1.0 - x.powi(2))).abs() / norm(1.0 - x.powi(2), x * (1.0 / self.q))
    }

    fn phase_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        if pulsation < self.omega0 {
            -(x * (1.0 / self.q) / (1.0 - x)).atan()
        } else if pulsation > self.omega0 {
            -PI - (x * (1.0 / self.q) / (1.0 - x)).atan()
        } else if pulsation == self.omega0 {
            -PI / 2.0
        } else {
            unreachable!()
        }
    }
}
