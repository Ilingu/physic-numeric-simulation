use std::{f64::consts::PI, ops::Range};

use crate::{
    fft::{evaluation_point, XyScatter},
    utils::norm,
};

#[derive(Debug)]
pub enum FiltreType {
    PasseHaut2nd,
    PasseBas2nd,
    PasseHaut1er,
    PasseBas1er,
    PasseBande,
    CoupeBande,
}

pub struct FiltreCaracteristique {
    pub filtre_type: FiltreType,
    pub h0: f64,
    pub omega0: f64,
    pub q: f64,
}

pub trait FiltreTrait {
    fn new(h0: f64, f0: f64, q: f64) -> Self;
    fn get_caracteristique(&self) -> FiltreCaracteristique;

    fn gain_graph(&self, n: usize, width: &Range<f64>) -> XyScatter {
        evaluation_point(n, width, &Box::new(move |f| self.gain_at(f)))
    }
    fn phase_graph(&self, n: usize, width: &Range<f64>) -> XyScatter {
        evaluation_point(n, width, &Box::new(move |f| self.phase_at(f)))
    }

    fn gain_at(&self, f: f64) -> f64;
    fn phase_at(&self, f: f64) -> f64;
}

pub struct FiltrePasseHaut1er {
    h0: f64,
    /// pulsation de coupure
    omega0: f64,
}

impl FiltreTrait for FiltrePasseHaut1er {
    fn new(h0: f64, f0: f64, _q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
        }
    }

    fn get_caracteristique(&self) -> FiltreCaracteristique {
        FiltreCaracteristique {
            filtre_type: FiltreType::PasseHaut1er,
            h0: self.h0,
            omega0: self.omega0,
            q: -1.0,
        }
    }

    fn gain_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        self.h0 / norm(1.0, 1.0 / x)
    }

    fn phase_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        (1.0 / x).atan()
    }
}

pub struct FiltrePasseBas1er {
    h0: f64,
    /// pulsation de coupure
    omega0: f64,
}

impl FiltreTrait for FiltrePasseBas1er {
    fn new(h0: f64, f0: f64, _q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
        }
    }

    fn get_caracteristique(&self) -> FiltreCaracteristique {
        FiltreCaracteristique {
            filtre_type: FiltreType::PasseBas1er,
            h0: self.h0,
            omega0: self.omega0,
            q: -1.0,
        }
    }

    fn gain_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        self.h0 / norm(1.0, x)
    }

    fn phase_at(&self, f: f64) -> f64 {
        let pulsation = 2.0 * PI * f;
        let x = pulsation / self.omega0;
        x.atan()
    }
}

pub struct FiltrePasseHaut2nd {
    h0: f64,
    /// pulsation de coupure
    omega0: f64,
    q: f64,
}

impl FiltreTrait for FiltrePasseHaut2nd {
    fn new(h0: f64, f0: f64, q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q,
        }
    }

    fn get_caracteristique(&self) -> FiltreCaracteristique {
        FiltreCaracteristique {
            filtre_type: FiltreType::PasseHaut2nd,
            h0: self.h0,
            omega0: self.omega0,
            q: self.q,
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
        let argh0 = if self.h0 < 0.0 {
            0.0
        } else if self.h0 > 0.0 {
            PI
        } else {
            unreachable!("Arg(0) is not defined")
        };
        let phic = if pulsation < self.omega0 {
            -(x * (1.0 / self.q) / (1.0 - x.powi(2))).atan()
        } else if pulsation > self.omega0 {
            -PI - (x * (1.0 / self.q) / (1.0 - x.powi(2))).atan()
        } else if pulsation == self.omega0 {
            -PI / 2.0
        } else {
            unreachable!("w must be >0")
        };
        argh0 + phic
    }
}

pub struct FiltrePasseBas2nd {
    h0: f64,
    /// pulsation de coupure
    omega0: f64,
    q: f64,
}
impl FiltreTrait for FiltrePasseBas2nd {
    fn new(h0: f64, f0: f64, q: f64) -> Self {
        Self {
            h0,
            omega0: 2.0 * PI * f0,
            q,
        }
    }

    fn get_caracteristique(&self) -> FiltreCaracteristique {
        FiltreCaracteristique {
            filtre_type: FiltreType::PasseBas2nd,
            h0: self.h0,
            omega0: self.omega0,
            q: self.q,
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
            -(x * (1.0 / self.q) / (1.0 - x.powi(2))).atan()
        } else if pulsation > self.omega0 {
            -PI - (x * (1.0 / self.q) / (1.0 - x.powi(2))).atan()
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

    fn get_caracteristique(&self) -> FiltreCaracteristique {
        FiltreCaracteristique {
            filtre_type: FiltreType::PasseBande,
            h0: self.h0,
            omega0: self.omega0,
            q: self.q,
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

    fn get_caracteristique(&self) -> FiltreCaracteristique {
        FiltreCaracteristique {
            filtre_type: FiltreType::CoupeBande,
            h0: self.h0,
            omega0: self.omega0,
            q: self.q,
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
        if pulsation != self.omega0 {
            -(x * (1.0 / self.q) / (1.0 - x.powi(2))).atan()
        } else if pulsation == self.omega0 {
            -PI / 2.0
        } else {
            unreachable!()
        }
    }
}
