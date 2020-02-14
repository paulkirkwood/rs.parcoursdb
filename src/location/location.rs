use crate::country::Country;

#[derive(PartialEq, Debug)]
pub struct Location {
    name: String,
    country: Country,
    elevation: Option<i32>
}

impl Location {
    pub fn new(name: String, country: Country, elevation: Option<i32>) -> Location {
        match elevation {
            Some(e) => {
                if e < 0 {
                    panic!("elevation must be greater than zero");
                }
                Location { name: name, country: country, elevation: elevation }
            },
            None => Location { name: name, country: country, elevation: elevation }
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn country(&self) -> &Country {
        &self.country
    }

    pub fn elevation(&self) -> Option<i32> {
        self.elevation
    }

    pub fn qualified_place(&self) -> String {
        format!("{} ({})", self.name(), self.country())
    }

    pub fn format(&self, country: Country) -> String {
        if self.country == country {
            format!("{}", self.name())
        } else {
            format!("{} ({})", self.name, self.country)
        }
    }

    pub fn clone(&self) -> Location {
        Location::new(self.name().clone(), self.country().clone(), self.elevation())
    }

    pub fn commune(&self, name: &'static str) -> Location {
        Location::new(format!("{}/{}", self.name(), name.to_string()), self.country().clone(), self.elevation())
    }

    pub fn vicinity(&self, name: &'static str) -> Location {
        Location::new(format!("{} ({})", self.name(), name.to_string()), self.country().clone(), self.elevation())
    }
}
