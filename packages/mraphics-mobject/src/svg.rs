use mraphics_core::MraphicsID;

pub struct SVGMobject {
    identifier: MraphicsID,

    pub code: String,
}

impl SVGMobject {
    pub fn new(code: String) -> Self {
        Self {
            identifier: MraphicsID::acquire(),

            code,
        }
    }
}
