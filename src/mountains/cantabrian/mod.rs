use crate::country::Country;
use crate::location::Location;
use crate::mountains::util::spanish_col;

pub fn alto_campoo() -> Location {
    Location::new("Alto Campoo".to_string(), Country::Spain, None)
}

pub fn alto_de_los_machucos() -> Location {
    spanish_col("Alto de Los Machucos", 921)
}
