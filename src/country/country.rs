use std::fmt;

#[derive(Copy,Clone,PartialEq, Debug)]
pub enum Country {
    Andorra,
    Austria,
    Belgium,
    Croatia,
    Denmark,
    Fiume,
    France,
    Germany,
    Greece,
    Israel,
    Ireland,
    Italy,
    Luxembourg,
    Monaco,
    Netherlands,
    NorthernIreland,
    Portugal,
    SanMarino,
    Slovenia,
    Spain,
    Switzerland,
    UnitedKingdom,
    VaticanCity,
    WestGermany,
    Yugoslavia,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
            Country::Andorra         => write!(f, "Andorra"),
            Country::Austria         => write!(f, "Austria"),
            Country::Belgium         => write!(f, "Belgium"),
            Country::Croatia         => write!(f, "Croatia"),
            Country::Denmark         => write!(f, "Denmark"),
            Country::Fiume           => write!(f, "Free State of Fiume"),
            Country::France          => write!(f, "France"),
            Country::Germany         => write!(f, "Germany"),
            Country::Greece          => write!(f, "Greece"),
            Country::Israel          => write!(f, "Israel"),
            Country::Ireland         => write!(f, "Ireland"),
            Country::Italy           => write!(f, "Italy"),
            Country::Luxembourg      => write!(f, "Luxembourg"),
            Country::Monaco          => write!(f, "Monaco"),
            Country::Netherlands     => write!(f, "Netherlands"),
            Country::NorthernIreland => write!(f, "Northern Ireland"),
            Country::Portugal        => write!(f, "Portugal"),
            Country::SanMarino       => write!(f, "San Marino"),
            Country::Slovenia        => write!(f, "Slovenia"),
            Country::Spain           => write!(f, "Spain"),
            Country::Switzerland     => write!(f, "Switzerland"),
            Country::UnitedKingdom   => write!(f, "United Kingdom"),
            Country::VaticanCity     => write!(f, "Vatican City"),
            Country::WestGermany     => write!(f, "West Germany"),
            Country::Yugoslavia      => write!(f, "Yugoslavia"),
        }
    }
}
