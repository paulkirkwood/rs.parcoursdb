extern crate parcoursdb;
extern crate chrono;

#[cfg(test)]
mod test {
    use parcoursdb::tour_de_france::repository::*;

    #[test]
    fn test_tour_de_france_1903() {
        let route = [
            "1,1 July,Paris to Lyon,467.0 km,Road stage",
            "2,5 July,Lyon to Marseille,374.0 km,Road stage",
            "3,8 July,Marseille to Toulouse,423.0 km,Road stage",
            "4,12 July,Toulouse to Bordeaux,268.0 km,Road stage",
            "5,13 July,Bordeaux to Nantes,425.0 km,Road stage",
            "6,18 July,Nantes to Paris,471.0 km,Road stage"
        ];

        let edition = tour_de_france_1903();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "6 stages");
    }

    #[test]
    fn test_tour_de_france_1904() {
        let route = [
            "1,2 July,Montgeron to Lyon,467.0 km,Road stage",
            "2,9 July,Lyon to Marseille,374.0 km,Road stage",
            "3,13 July,Marseille to Toulouse,424.0 km,Road stage",
            "4,17 July,Toulouse to Bordeaux,268.0 km,Road stage",
            "5,20 July,Bordeaux to Nantes,425.0 km,Road stage",
            "6,23 July,Nantes to Paris,471.0 km,Road stage"
        ];

        let edition = tour_de_france_1904();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "6 stages");
    }

    #[test]
    fn test_tour_de_france_1905() {
        let route = [
            "1,9 July,Paris to Nancy,340.0 km,Road stage",
            "2,11 July,Nancy to Besançon,299.0 km,Road stage",
            "3,14 July,Besançon to Grenoble,327.0 km,Road stage",
            "4,16 July,Grenoble to Toulon,348.0 km,Road stage",
            "5,18 July,Toulon to Nîmes,192.0 km,Road stage",
            "6,20 July,Nîmes to Toulouse,307.0 km,Road stage",
            "7,22 July,Toulouse to Bordeaux,268.0 km,Road stage",
            "8,24 July,Bordeaux to La Rochelle,257.0 km,Road stage",
            "9,26 July,La Rochelle to Rennes,263.0 km,Road stage",
            "10,28 July,Rennes to Caen,167.0 km,Road stage",
            "11,29 July,Caen to Paris,253.0 km,Road stage"
        ];

        let edition = tour_de_france_1905();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "11 stages");
    }

    #[test]
    fn test_tour_de_france_1906() {
        let route = [
            "1,4 July,Paris to Lille,275.0 km,Road stage",
            "2,6 July,Douai to Nancy,400.0 km,Road stage",
            "3,8 July,Nancy to Dijon,416.0 km,Road stage",
            "4,10 July,Dijon to Grenoble,311.0 km,Road stage",
            "5,12 July,Grenoble to Nice,345.0 km,Road stage",
            "6,14 July,Nice to Marseille,292.0 km,Road stage",
            "7,16 July,Marseille to Toulouse,480.0 km,Road stage",
            "8,18 July,Toulouse to Bayonne,300.0 km,Road stage",
            "9,20 July,Bayonne to Bordeaux,338.0 km,Road stage",
            "10,22 July,Bordeaux to Nantes,391.0 km,Road stage",
            "11,24 July,Nantes to Brest,321.0 km,Road stage",
            "12,26 July,Brest to Caen,415.0 km,Road stage",
            "13,29 July,Caen to Paris,259.0 km,Road stage"
        ];

        let edition = tour_de_france_1906();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "13 stages");
    }

    #[test]
    fn test_tour_de_france_1907() {
        let route = [
            "1,8 July,Paris to Roubaix,272.0 km,Road stage",
            "2,10 July,Roubaix to Metz,398.0 km,Road stage",
            "3,12 July,Metz to Belfort,259.0 km,Road stage",
            "4,14 July,Belfort to Lyon,309.0 km,Road stage",
            "5,16 July,Lyon to Grenoble,311.0 km,Road stage",
            "6,18 July,Grenoble to Nice,345.0 km,Road stage",
            "7,20 July,Nice to Nîmes,345.0 km,Road stage",
            "8,22 July,Nîmes to Toulouse,303.0 km,Road stage",
            "9,24 July,Toulouse to Bayonne,299.0 km,Road stage",
            "10,26 July,Bayonne to Bordeaux,269.0 km,Road stage",
            "11,28 July,Bordeaux to Nantes,391.0 km,Road stage",
            "12,30 July,Nantes to Brest,321.0 km,Road stage",
            "13,1 August,Brest to Caen,415.0 km,Road stage",
            "14,4 August,Caen to Paris,251.0 km,Road stage"
        ];

        let edition = tour_de_france_1907();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "14 stages");
    }

    #[test]
    fn test_tour_de_france_1908() {
        let route = [
            "1,13 July,Paris to Roubaix,272.0 km,Road stage",
            "2,15 July,Roubaix to Metz,398.0 km,Road stage",
            "3,17 July,Metz to Belfort,259.0 km,Road stage",
            "4,19 July,Belfort to Lyon,309.0 km,Road stage",
            "5,21 July,Lyon to Grenoble,311.0 km,Road stage",
            "6,23 July,Grenoble to Nice,345.0 km,Road stage",
            "7,25 July,Nice to Nîmes,354.0 km,Road stage",
            "8,27 July,Nîmes to Toulouse,303.0 km,Road stage",
            "9,29 July,Toulouse to Bayonne,299.0 km,Road stage",
            "10,31 July,Bayonne to Bordeaux,269.0 km,Road stage",
            "11,2 August,Bordeaux to Nantes,391.0 km,Road stage",
            "12,4 August,Nantes to Brest,321.0 km,Road stage",
            "13,6 August,Brest to Caen,415.0 km,Road stage",
            "14,9 August,Caen to Paris,251.0 km,Road stage"
        ];

        let edition = tour_de_france_1908();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "14 stages");
    }

    #[test]
    fn test_tour_de_france_1909() {
        let route = [
            "1,5 July,Paris to Roubaix,272.0 km,Road stage",
            "2,7 July,Roubaix to Metz,398.0 km,Road stage",
            "3,9 July,Metz to Belfort,259.0 km,Road stage",
            "4,11 July,Belfort to Lyon,309.0 km,Road stage",
            "5,13 July,Lyon to Grenoble,311.0 km,Road stage",
            "6,15 July,Grenoble to Nice,346.0 km,Road stage",
            "7,17 July,Nice to Nîmes,345.0 km,Road stage",
            "8,19 July,Nîmes to Toulouse,303.0 km,Road stage",
            "9,21 July,Toulouse to Bayonne,299.0 km,Road stage",
            "10,23 July,Bayonne to Bordeaux,269.0 km,Road stage",
            "11,25 July,Bordeaux to Nantes,391.0 km,Road stage",
            "12,27 July,Nantes to Brest,321.0 km,Road stage",
            "13,29 July,Brest to Caen,424.0 km,Road stage",
            "14,1 August,Caen to Paris,250.0 km,Road stage"
        ];

        let edition = tour_de_france_1909();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "14 stages");
    }

    #[test]
    fn test_tour_de_france_1910() {
        let route = [
            "1,3 July,Paris to Roubaix,269.0 km,Road stage",
            "2,5 July,Roubaix to Metz,398.0 km,Road stage",
            "3,7 July,Metz to Belfort,259.0 km,Road stage",
            "4,9 July,Belfort to Lyon,309.0 km,Road stage",
            "5,11 July,Lyon to Grenoble,311.0 km,Road stage",
            "6,13 July,Grenoble to Nice,345.0 km,Road stage",
            "7,17 July,Nice to Nîmes,345.0 km,Road stage",
            "8,19 July,Nîmes to Perpignan,216.0 km,Road stage",
            "9,21 July,Perpignan to Luchon,289.0 km,Road stage",
            "10,23 July,Bayonne to Bordeaux,269.0 km,Road stage",
            "11,25 July,Bordeaux to Nantes,391.0 km,Road stage",
            "12,27 July,Nantes to Brest,321.0 km,Road stage",
            "13,29 July,Brest to Caen,424.0 km,Road stage",
            "14,31 July,Caen to Paris,262.0 km,Road stage"
        ];

        let edition = tour_de_france_1910();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "14 stages");
    }

    #[test]
    fn test_tour_de_france_1911() {
        let route = [
            "1,2 July,Paris to Dunkerque,351.0 km,Road stage",
            "2,4 July,Dunkerque to Longwy,388.0 km,Road stage",
            "3,6 July,Longwy to Belfort,331.0 km,Road stage",
            "4,8 July,Belfort to Chamonix,344.0 km,Road stage",
            "5,10 July,Chamonix to Grenoble,366.0 km,Road stage",
            "6,12 July,Grenoble to Nice,348.0 km,Road stage",
            "7,14 July,Nice to Marseille,334.0 km,Road stage",
            "8,16 July,Marseille to Perpignan,335.0 km,Road stage",
            "9,18 July,Perpignan to Luchon,289.0 km,Road stage",
            "10,20 July,Luchon to Bayonne,326.0 km,Road stage",
            "11,22 July,Bayonne to La Rochelle,379.0 km,Road stage",
            "12,23 July,La Rochelle to Brest,470.0 km,Road stage",
            "13,26 July,Brest to Cherbourg,405.0 km,Road stage",
            "14,28 July,Cherbourg to Le Havre,361.0 km,Road stage",
            "15,30 July,Le Havre to Paris,317.0 km,Road stage"
        ];

        let edition = tour_de_france_1911();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1912() {
        let route = [
            "1,30 June,Paris to Dunkerque,351.0 km,Road stage",
            "2,2 July,Dunkerque to Longwy,388.0 km,Road stage",
            "3,4 July,Longwy to Belfort,331.0 km,Road stage",
            "4,6 July,Belfort to Chamonix,344.0 km,Road stage",
            "5,8 July,Chamonix to Grenoble,366.0 km,Road stage",
            "6,10 July,Grenoble to Nice,323.0 km,Road stage",
            "7,12 July,Nice to Marseille,334.0 km,Road stage",
            "8,14 July,Marseille to Perpignan,335.0 km,Road stage",
            "9,16 July,Perpignan to Luchon,289.0 km,Road stage",
            "10,18 July,Luchon to Bayonne,326.0 km,Road stage",
            "11,20 July,Bayonne to La Rochelle,379.0 km,Road stage",
            "12,21 July,La Rochelle to Brest,470.0 km,Road stage",
            "13,24 July,Brest to Cherbourg,405.0 km,Road stage",
            "14,26 July,Cherbourg to Le Havre,361.0 km,Road stage",
            "15,28 July,Le Havre to Paris,317.0 km,Road stage"
        ];

        let edition = tour_de_france_1912();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1913() {
        let route = [
            "1,29 June,Paris to Le Havre,388.0 km,Road stage",
            "2,1 July,Le Havre to Cherbourg,364.0 km,Road stage",
            "3,3 July,Cherbourg to Brest,405.0 km,Road stage",
            "4,5 July,Brest to La Rochelle,470.0 km,Road stage",
            "5,7 July,La Rochelle to Bayonne,379.0 km,Road stage",
            "6,9 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,11 July,Luchon to Perpignan,324.0 km,Road stage",
            "8,13 July,Perpignan to Aix-en-Provence,325.0 km,Road stage",
            "9,15 July,Aix-en-Provence to Nice,356.0 km,Road stage",
            "10,17 July,Nice to Grenoble,333.0 km,Road stage",
            "11,19 July,Grenoble to Geneva (Switzerland),325.0 km,Road stage",
            "12,21 July,Geneva (Switzerland) to Belfort,335.0 km,Road stage",
            "13,23 July,Belfort to Longwy,325.0 km,Road stage",
            "14,25 July,Longwy to Dunkerque,393.0 km,Road stage",
            "15,27 July,Dunkerque to Paris,340.0 km,Road stage"
        ];

        let edition = tour_de_france_1913();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1914() {
        let route = [
            "1,28 June,Paris to Le Havre,388.0 km,Road stage",
            "2,30 June,Le Havre to Cherbourg,364.0 km,Road stage",
            "3,2 July,Cherbourg to Brest,405.0 km,Road stage",
            "4,4 July,Brest to La Rochelle,470.0 km,Road stage",
            "5,6 July,La Rochelle to Bayonne,376.0 km,Road stage",
            "6,8 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,10 July,Luchon to Perpignan,323.0 km,Road stage",
            "8,12 July,Perpignan to Marseille,370.0 km,Road stage",
            "9,14 July,Marseille to Nice,338.0 km,Road stage",
            "10,16 July,Nice to Grenoble,323.0 km,Road stage",
            "11,18 July,Grenoble to Geneva (Switzerland),325.0 km,Road stage",
            "12,20 July,Geneva (Switzerland) to Belfort,325.0 km,Road stage",
            "13,22 July,Belfort to Longwy,325.0 km,Road stage",
            "14,24 July,Longwy to Dunkerque,390.0 km,Road stage",
            "15,26 July,Dunkerque to Paris,340.0 km,Road stage"
        ];

        let edition = tour_de_france_1914();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1919() {
        let route = [
            "1,29 June,Paris to Le Havre,388.0 km,Road stage",
            "2,1 July,Le Havre to Cherbourg,364.0 km,Road stage",
            "3,3 July,Cherbourg to Brest,405.0 km,Road stage",
            "4,5 July,Brest to Les Sables-d'Olonne,412.0 km,Road stage",
            "5,7 July,Les Sables-d'Olonne to Bayonne,482.0 km,Road stage",
            "6,9 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,11 July,Luchon to Perpignan,323.0 km,Road stage",
            "8,13 July,Perpignan to Marseille,370.0 km,Road stage",
            "9,15 July,Marseille to Nice,338.0 km,Road stage",
            "10,17 July,Nice to Grenoble,333.0 km,Road stage",
            "11,19 July,Grenoble to Geneva (Switzerland),325.0 km,Road stage",
            "12,21 July,Geneva (Switzerland) to Strasbourg,371.0 km,Road stage",
            "13,23 July,Strasbourg to Metz,315.0 km,Road stage",
            "14,25 July,Metz to Dunkerque,468.0 km,Road stage",
            "15,27 July,Dunkerque to Paris,340.0 km,Road stage"
        ];

        let edition = tour_de_france_1919();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1920() {
        let route = [
            "1,27 June,Paris to Le Havre,388.0 km,Road stage",
            "2,29 June,Le Havre to Cherbourg,364.0 km,Road stage",
            "3,1 July,Cherbourg to Brest,405.0 km,Road stage",
            "4,3 July,Brest to Les Sables-d'Olonne,412.0 km,Road stage",
            "5,5 July,Les Sables-d'Olonne to Bayonne,482.0 km,Road stage",
            "6,7 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,9 July,Luchon to Perpignan,323.0 km,Road stage",
            "8,11 July,Perpignan to Aix-en-Provence,325.0 km,Road stage",
            "9,14 July,Aix-en-Provence to Nice,356.0 km,Road stage",
            "10,16 July,Nice to Grenoble,333.0 km,Road stage",
            "11,18 July,Grenoble to Gex,362.0 km,Road stage",
            "12,20 July,Gex to Strasbourg,354.0 km,Road stage",
            "13,22 July,Strasbourg to Metz,300.0 km,Road stage",
            "14,24 July,Metz to Dunkerque,433.0 km,Road stage",
            "15,27 July,Dunkerque to Paris,340.0 km,Road stage"
        ];

        let edition = tour_de_france_1920();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1921() {
        let route = [
            "1,26 June,Paris to Le Havre,388.0 km,Road stage",
            "2,28 June,Le Havre to Cherbourg,364.0 km,Road stage",
            "3,30 June,Cherbourg to Brest,405.0 km,Road stage",
            "4,2 July,Brest to Les Sables-d'Olonne,412.0 km,Road stage",
            "5,4 July,Les Sables-d'Olonne to Bayonne,482.0 km,Road stage",
            "6,6 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,8 July,Luchon to Perpignan,323.0 km,Road stage",
            "8,10 July,Perpignan to Toulon,411.0 km,Road stage",
            "9,12 July,Toulon to Nice,272.0 km,Road stage",
            "10,14 July,Nice to Grenoble,333.0 km,Road stage",
            "11,16 July,Grenoble to Geneva (Switzerland),325.0 km,Road stage",
            "12,18 July,Geneva (Switzerland) to Strasbourg,371.0 km,Road stage",
            "13,20 July,Strasbourg to Metz,300.0 km,Road stage",
            "14,22 July,Metz to Dunkerque,433.0 km,Road stage",
            "15,24 July,Dunkerque to Paris,340.0 km,Road stage",
        ];

        let edition = tour_de_france_1921();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1922() {
        let route = [
            "1,25 June,Paris to Le Havre,388.0 km,Road stage",
            "2,27 June,Le Havre to Cherbourg,364.0 km,Road stage",
            "3,29 June,Cherbourg to Brest,405.0 km,Road stage",
            "4,1 July,Brest to Les Sables-d'Olonne,412.0 km,Road stage",
            "5,3 July,Les Sables-d'Olonne to Bayonne,482.0 km,Road stage",
            "6,5 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,7 July,Luchon to Perpignan,323.0 km,Road stage",
            "8,9 July,Perpignan to Toulon,411.0 km,Road stage",
            "9,11 July,Toulon to Nice,284.0 km,Road stage",
            "10,13 July,Nice to Briançon,274.0 km,Road stage",
            "11,15 July,Briançon to Geneva (Switzerland),260.0 km,Road stage",
            "12,17 July,Geneva (Switzerland) to Strasbourg,371.0 km,Road stage",
            "13,19 July,Strasbourg to Metz,300.0 km,Road stage",
            "14,21 July,Metz to Dunkerque,433.0 km,Road stage",
            "15,23 July,Dunkerque to Paris,340.0 km,Road stage"
        ];

        let edition = tour_de_france_1922();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1923() {
        let route = [
            "1,24 June,Paris to Le Havre,381.0 km,Road stage",
            "2,26 June,Le Havre to Cherbourg,371.0 km,Road stage",
            "3,28 June,Cherbourg to Brest,405.0 km,Road stage",
            "4,30 June,Brest to Les Sables-d'Olonne,412.0 km,Road stage",
            "5,2 July,Les Sables-d'Olonne to Bayonne,482.0 km,Road stage",
            "6,4 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,6 July,Luchon to Perpignan,323.0 km,Road stage",
            "8,8 July,Perpignan to Toulon,427.0 km,Road stage",
            "9,10 July,Toulon to Nice,281.0 km,Road stage",
            "10,12 July,Nice to Briançon,275.0 km,Road stage",
            "11,14 July,Briançon to Geneva (Switzerland),260.0 km,Road stage",
            "12,16 July,Geneva (Switzerland) to Strasbourg,377.0 km,Road stage",
            "13,18 July,Strasbourg to Metz,300.0 km,Road stage",
            "14,20 July,Metz to Dunkerque,433.0 km,Road stage",
            "15,22 July,Dunkerque to Paris,343.0 km,Road stage"
        ];

        let edition = tour_de_france_1923();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1924() {
        let route = [
            "1,22 June,Paris to Le Havre,381.0 km,Road stage",
            "2,24 June,Le Havre to Cherbourg,371.0 km,Road stage",
            "3,26 June,Cherbourg to Brest,405.0 km,Road stage",
            "4,28 June,Brest to Les Sables-d'Olonne,412.0 km,Road stage",
            "5,30 June,Les Sables-d'Olonne to Bayonne,482.0 km,Road stage",
            "6,2 July,Bayonne to Luchon,326.0 km,Road stage",
            "7,4 July,Luchon to Perpignan,323.0 km,Road stage",
            "8,6 July,Perpignan to Toulon,427.0 km,Road stage",
            "9,8 July,Toulon to Nice,280.0 km,Road stage",
            "10,10 July,Nice to Briançon,275.0 km,Road stage",
            "11,12 July,Briançon to Gex,307.0 km,Road stage",
            "12,14 July,Gex to Strasbourg,360.0 km,Road stage",
            "13,16 July,Strasbourg to Metz,300.0 km,Road stage",
            "14,18 July,Metz to Dunkerque,433.0 km,Road stage",
            "15,20 July,Dunkerque to Paris,343.0 km,Road stage"
        ];

        let edition = tour_de_france_1924();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "15 stages");
    }

    #[test]
    fn test_tour_de_france_1925() {
        let route = [
            "1,21 June,Paris to Le Havre,340.0 km,Road stage",
            "2,23 June,Le Havre to Cherbourg,371.0 km,Road stage",
            "3,25 June,Cherbourg to Brest,405.0 km,Road stage",
            "4,26 June,Brest to Vannes,208.0 km,Road stage",
            "5,27 June,Vannes to Les Sables-d'Olonne,204.0 km,Road stage",
            "6,28 June,Les Sables-d'Olonne to Bordeaux,293.0 km,Road stage",
            "7,29 June,Bordeaux to Bayonne,189.0 km,Road stage",
            "8,1 July,Bayonne to Luchon,326.0 km,Road stage",
            "9,3 July,Luchon to Perpignan,323.0 km,Road stage",
            "10,4 July,Perpignan to Nîmes,215.0 km,Road stage",
            "11,5 July,Nîmes to Toulon,215.0 km,Road stage",
            "12,7 July,Toulon to Nice,280.0 km,Road stage",
            "13,9 July,Nice to Briançon,275.0 km,Road stage",
            "14,11 July,Briançon to Evian,303.0 km,Road stage",
            "15,13 July,Evian to Mulhouse,373.0 km,Road stage",
            "16,15 July,Mulhouse to Metz,334.0 km,Road stage",
            "17,17 July,Metz to Dunkerque,433.0 km,Road stage",
            "18,19 July,Dunkerque to Paris,343.0 km,Road stage"
        ];

        let edition = tour_de_france_1925();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages");
    }

    #[test]
    fn test_tour_de_france_1926() {
        let route = [
            "1,20 June,Evian to Mulhouse,373.0 km,Road stage",
            "2,22 June,Mulhouse to Metz,334.0 km,Road stage",
            "3,24 June,Metz to Dunkerque,433.0 km,Road stage",
            "4,26 June,Dunkerque to Le Havre,361.0 km,Road stage",
            "5,28 June,Le Havre to Cherbourg,357.0 km,Road stage",
            "6,30 June,Cherbourg to Brest,405.0 km,Road stage",
            "7,2 July,Brest to Les Sables-d'Olonne,412.0 km,Road stage",
            "8,3 July,Les Sables-d'Olonne to Bordeaux,285.0 km,Road stage",
            "9,4 July,Bordeaux to Bayonne,189.0 km,Road stage",
            "10,6 July,Bayonne to Luchon,326.0 km,Road stage",
            "11,8 July,Luchon to Perpignan,323.0 km,Road stage",
            "12,10 July,Perpignan to Toulon,427.0 km,Road stage",
            "13,12 July,Toulon to Nice,280.0 km,Road stage",
            "14,14 July,Nice to Briançon,275.0 km,Road stage",
            "15,16 July,Briançon to Evian,303.0 km,Road stage",
            "16,17 July,Evian to Dijon,321.0 km,Road stage",
            "17,18 July,Dijon to Paris,341.0 km,Road stage"
        ];

        let edition = tour_de_france_1926();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "17 stages");
    }

    #[test]
    fn test_tour_de_france_1927() {
        let route = [
            "1,19 June,Paris to Dieppe,180.0 km,Team time trial",
            "2,20 June,Dieppe to Le Havre,103.0 km,Team time trial",
            "3,21 June,Le Havre to Caen,225.0 km,Team time trial",
            "4,22 June,Caen to Cherbourg,140.0 km,Team time trial",
            "5,23 June,Cherbourg to Dinan,199.0 km,Team time trial",
            "6,24 June,Dinan to Brest,206.0 km,Team time trial",
            "7,25 June,Brest to Vannes,207.0 km,Team time trial",
            "8,26 June,Vannes to Les Sables-d'Olonne,204.0 km,Team time trial",
            "9,27 June,Les Sables-d'Olonne to Bordeaux,285.0 km,Team time trial",
            "10,28 June,Bordeaux to Bayonne,189.0 km,Road stage",
            "11,30 June,Bayonne to Luchon,326.0 km,Road stage",
            "12,2 July,Luchon to Perpignan,323.0 km,Road stage",
            "13,4 July,Perpignan to Marseille,360.0 km,Road stage",
            "14,5 July,Marseille to Toulon,120.0 km,Team time trial",
            "15,6 July,Toulon to Nice,220.0 km,Road stage",
            "16,8 July,Nice to Briançon,275.0 km,Road stage",
            "17,9 July,Briançon to Evian,283.0 km,Road stage",
            "18,11 July,Evian to Pontarlier,213.0 km,Team time trial",
            "19,12 July,Pontarlier to Belfort,119.0 km,Team time trial",
            "20,13 July,Belfort to Strasbourg,145.0 km,Team time trial",
            "21,14 July,Strasbourg to Metz,165.0 km,Team time trial",
            "22,15 July,Metz to Charleville,159.0 km,Team time trial",
            "23,16 July,Charleville to Dunkerque,270.0 km,Team time trial",
            "24,17 July,Dunkerque to Paris,344.0 km,Road stage"
        ];

        let edition = tour_de_france_1927();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "24 stages");
    }

    #[test]
    fn test_tour_de_france_1928() {
        let route = [
            "1,17 June,Paris to Caen,207.0 km,Team time trial",
            "2,18 June,Caen to Cherbourg,140.0 km,Team time trial",
            "3,19 June,Cherbourg to Dinan,199.0 km,Team time trial",
            "4,20 June,Dinan to Brest,206.0 km,Team time trial",
            "5,21 June,Brest to Vannes,208.0 km,Team time trial",
            "6,22 June,Vannes to Les Sables-d'Olonne,204.0 km,Team time trial",
            "7,23 June,Les Sables-d'Olonne to Bordeaux,285.0 km,Team time trial",
            "8,24 June,Bordeaux to Hendaye,225.0 km,Team time trial",
            "9,26 June,Hendaye to Luchon,387.0 km,Road stage",
            "10,28 June,Luchon to Perpignan,323.0 km,Road stage",
            "11,30 June,Perpignan to Marseille,363.0 km,Road stage",
            "12,2 July,Marseille to Nice,330.0 km,Road stage",
            "13,4 July,Nice to Grenoble,333.0 km,Road stage",
            "14,6 July,Grenoble to Evian,329.0 km,Road stage",
            "15,8 July,Evian to Pontarlier,213.0 km,Team time trial",
            "16,9 July,Pontarlier to Belfort,119.0 km,Team time trial",
            "17,10 July,Belfort to Strasbourg,145.0 km,Team time trial",
            "18,11 July,Strasbourg to Metz,165.0 km,Team time trial",
            "19,12 July,Metz to Charleville,159.0 km,Team time trial",
            "20,13 July,Charleville to Malo-les-Bains,271.0 km,Team time trial",
            "21,14 July,Malo-les-Bains to Dunkerque,234.0 km,Team time trial",
            "22,15 July,Dieppe to Paris,331.0 km,Road stage"
        ];

        let edition = tour_de_france_1928();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_de_france_1929() {
        let route = [
            "1,30 June,Paris to Caen,206.0 km,Road stage",
            "2,1 July,Caen to Cherbourg,140.0 km,Road stage",
            "3,2 July,Cherbourg to Dinan,199.0 km,Road stage",
            "4,3 July,Dinan to Brest,206.0 km,Road stage",
            "5,4 July,Brest to Vannes,208.0 km,Road stage",
            "6,5 July,Vannes to Les Sables-d'Olonne,206.0 km,Road stage",
            "7,6 July,Les Sables-d'Olonne to Bordeaux,285.0 km,Road stage",
            "8,7 July,Bordeaux to Bayonne,182.0 km,Road stage",
            "9,9 July,Bayonne to Luchon,363.0 km,Road stage",
            "10,11 July,Luchon to Perpignan,323.0 km,Road stage",
            "11,13 July,Perpignan to Marseille,366.0 km,Road stage",
            "12,15 July,Marseille to Cannes,191.0 km,Team time trial",
            "13,16 July,Cannes to Nice,133.0 km,Road stage",
            "14,18 July,Nice to Grenoble,333.0 km,Road stage",
            "15,20 July,Grenoble to Evian,329.0 km,Road stage",
            "16,22 July,Evian to Belfort,283.0 km,Road stage",
            "17,23 July,Belfort to Strasbourg,145.0 km,Road stage",
            "18,24 July,Strasbourg to Metz,165.0 km,Road stage",
            "19,25 July,Metz to Charleville,159.0 km,Team time trial",
            "20,26 July,Charleville to Malo-les-Bains,270.0 km,Team time trial",
            "21,27 July,Malo-les-Bains to Dunkerque,234.0 km,Road stage",
            "22,28 July,Dieppe to Paris,332.0 km,Road stage"
        ];

        let edition = tour_de_france_1929();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_de_france_1930() {
        let route = [
            "1,2 July,Paris to Caen,206.0 km,Road stage",
            "2,3 July,Caen to Dinan,203.0 km,Road stage",
            "3,4 July,Dinan to Brest,206.0 km,Road stage",
            "4,5 July,Brest to Vannes,210.0 km,Road stage",
            "5,6 July,Vannes to Les Sables-d'Olonne,202.0 km,Road stage",
            "6,7 July,Les Sables-d'Olonne to Bordeaux,285.0 km,Road stage",
            "7,8 July,Bordeaux to Hendaye,222.0 km,Road stage",
            "8,9 July,Hendaye to Pau,146.0 km,Road stage",
            "9,10 July,Pau to Luchon,231.0 km,Road stage",
            "10,12 July,Luchon to Perpignan,322.0 km,Road stage",
            "11,14 July,Perpignan to Montpellier,164.0 km,Road stage",
            "12,15 July,Montpellier to Marseille,209.0 km,Road stage",
            "13,16 July,Marseille to Cannes,181.0 km,Road stage",
            "14,17 July,Cannes to Nice,132.0 km,Road stage",
            "15,19 July,Nice to Grenoble,333.0 km,Road stage",
            "16,21 July,Grenoble to Evian,331.0 km,Road stage",
            "17,23 July,Evian to Belfort,282.0 km,Road stage",
            "18,24 July,Belfort to Metz,223.0 km,Road stage",
            "19,25 July,Metz to Charleville,159.0 km,Road stage",
            "20,26 July,Charleville to Malo-les-Bains,271.0 km,Road stage",
            "21,28 July,Malo-les-Bains to Paris,300.0 km,Road stage"
        ];

        let edition = tour_de_france_1930();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_1931() {
        let route = [
            "1,30 June,Paris to Caen,208.0 km,Road stage",
            "2,1 July,Caen to Dinan,212.0 km,Road stage",
            "3,2 July,Dinan to Brest,206.0 km,Road stage",
            "4,3 July,Brest to Vannes,211.0 km,Road stage",
            "5,4 July,Vannes to Les Sables-d'Olonne,202.0 km,Road stage",
            "6,5 July,Les Sables-d'Olonne to Bordeaux,338.0 km,Road stage",
            "7,6 July,Bordeaux to Bayonne,180.0 km,Road stage",
            "8,7 July,Bayonne to Pau,106.0 km,Road stage",
            "9,8 July,Pau to Luchon,231.0 km,Road stage",
            "10,10 July,Luchon to Perpignan,322.0 km,Road stage",
            "11,12 July,Perpignan to Montpellier,164.0 km,Road stage",
            "12,13 July,Montpellier to Marseille,207.0 km,Road stage",
            "13,14 July,Marseille to Cannes,181.0 km,Road stage",
            "14,15 July,Cannes to Nice,132.0 km,Road stage",
            "15,17 July,Nice to Gap,233.0 km,Road stage",
            "16,18 July,Gap to Grenoble,102.0 km,Road stage",
            "17,19 July,Grenoble to Aix-les-Bains,230.0 km,Road stage",
            "18,20 July,Aix-les-Bains to Evian,204.0 km,Road stage",
            "19,21 July,Evian to Belfort,282.0 km,Road stage",
            "20,22 July,Belfort to Colmar,209.0 km,Road stage",
            "21,23 July,Colmar to Metz,192.0 km,Road stage",
            "22,24 July,Metz to Charleville,159.0 km,Road stage",
            "23,26 July,Charleville to Malo-les-Bains,271.0 km,Road stage",
            "24,28 July,Malo-les-Bains to Paris,313.0 km,Road stage"
        ];

        let edition = tour_de_france_1931();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "24 stages");
    }

    #[test]
    fn test_tour_de_france_1932() {
        let route = [
            "1,6 July,Paris to Caen,208.0 km,Road stage",
            "2,7 July,Caen to Nantes,300.0 km,Road stage",
            "3,9 July,Nantes to Bordeaux,387.0 km,Road stage",
            "4,11 July,Bordeaux to Pau,206.0 km,Road stage",
            "5,12 July,Pau to Luchon,229.0 km,Road stage",
            "6,14 July,Luchon to Perpignan,322.0 km,Road stage",
            "7,16 July,Perpignan to Montpellier,168.0 km,Road stage",
            "8,17 July,Montpellier to Marseille,206.0 km,Road stage",
            "9,18 July,Marseille to Cannes,191.0 km,Road stage",
            "10,19 July,Cannes to Nice,132.0 km,Road stage",
            "11,21 July,Nice to Gap,233.0 km,Road stage",
            "12,22 July,Gap to Grenoble,102.0 km,Road stage",
            "13,23 July,Grenoble to Aix-les-Bains,230.0 km,Road stage",
            "14,24 July,Aix-les-Bains to Evian,204.0 km,Road stage",
            "15,25 July,Evian to Belfort,291.0 km,Road stage",
            "16,26 July,Belfort to Strasbourg,145.0 km,Road stage",
            "17,27 July,Strasbourg to Metz,165.0 km,Road stage",
            "18,28 July,Metz to Charleville,159.0 km,Road stage",
            "19,29 July,Charleville to Malo-les-Bains,271.0 km,Road stage",
            "20,30 July,Malo-les-Bains to Amiens,212.0 km,Road stage",
            "21,31 July,Amiens to Paris,159.0 km,Road stage"
        ];

        let edition = tour_de_france_1932();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_1933() {
        let route = [
            "1,27 June,Paris to Lille,262.0 km,Road stage",
            "2,28 June,Lille to Charleville,192.0 km,Road stage",
            "3,29 June,Charleville to Metz,166.0 km,Road stage",
            "4,30 June,Metz to Belfort,220.0 km,Road stage",
            "5,1 July,Belfort to Evian,293.0 km,Road stage",
            "6,3 July,Evian to Aix-les-Bains,207.0 km,Road stage",
            "7,4 July,Aix-les-Bains to Grenoble,229.0 km,Road stage",
            "8,5 July,Grenoble to Gap,102.0 km,Road stage",
            "9,6 July,Gap to Digne,227.0 km,Road stage",
            "10,7 July,Digne to Nice,156.0 km,Road stage",
            "11,9 July,Nice to Cannes,128.0 km,Road stage",
            "12,10 July,Cannes to Marseille,208.0 km,Road stage",
            "13,11 July,Marseille to Montpellier,168.0 km,Road stage",
            "14,12 July,Montpellier to Perpignan,166.0 km,Road stage",
            "15,14 July,Perpignan to Ax-les-Thermes,158.0 km,Road stage",
            "16,15 July,Ax-les-Thermes to Luchon,165.0 km,Road stage",
            "17,16 July,Luchon to Tarbes,91.0 km,Road stage",
            "18,17 July,Tarbes to Pau,185.0 km,Road stage",
            "19,19 July,Pau to Bordeaux,233.0 km,Road stage",
            "20,20 July,Bordeaux to La Rochelle,183.0 km,Road stage",
            "21,21 July,La Rochelle to Rennes,266.0 km,Road stage",
            "22,22 July,Rennes to Caen,169.0 km,Road stage",
            "23,23 July,Caen to Paris,222.0 km,Road stage"
        ];

        let edition = tour_de_france_1933();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "23 stages");
    }

    #[test]
    fn test_tour_de_france_1934() {
        let route = [
            "1,3 July,Paris to Lille,262.0 km,Road stage",
            "2,4 July,Lille to Charleville,192.0 km,Road stage",
            "3,5 July,Charleville to Metz,161.0 km,Road stage",
            "4,6 July,Metz to Belfort,220.0 km,Road stage",
            "5,7 July,Belfort to Evian,293.0 km,Road stage",
            ",8 July,Rest day,Evian",
            "6,9 July,Evian to Aix-les-Bains,207.0 km,Road stage",
            "7,10 July,Aix-les-Bains to Grenoble,229.0 km,Road stage",
            "8,11 July,Grenoble to Gap,102.0 km,Road stage",
            "9,12 July,Gap to Digne,227.0 km,Road stage",
            "10,13 July,Digne to Nice,156.0 km,Road stage",
            ",14 July,Rest day,Nice",
            "11,15 July,Nice to Cannes,126.0 km,Road stage",
            "12,16 July,Cannes to Marseille,195.0 km,Road stage",
            "13,17 July,Marseille to Montpellier,172.0 km,Road stage",
            "14,18 July,Montpellier to Perpignan,177.0 km,Road stage",
            ",19 July,Rest day,Perpignan",
            "15,20 July,Perpignan to Ax-les-Thermes,158.0 km,Road stage",
            "16,21 July,Ax-les-Thermes to Luchon,165.0 km,Road stage",
            "17,22 July,Luchon to Tarbes,91.0 km,Road stage",
            "18,23 July,Tarbes to Pau,172.0 km,Road stage",
            ",24 July,Rest day,Pau",
            "19,25 July,Pau to Bordeaux,215.0 km,Road stage",
            "20,26 July,Bordeaux to La Rochelle,183.0 km,Road stage",
            "21a,27 July,La Rochelle to La Roche-sur-Yon,81.0 km,Road stage",
            "21b,27 July,La Roche-sur-Yon to Nantes,90.0 km,Individual time trial",
            "22,28 July,Nantes to Caen,275.0 km,Road stage",
            "23,29 July,Caen to Paris,221.0 km,Road stage"
        ];

        let edition = tour_de_france_1934();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "23 stages including 1 split stage");
    }

    #[test]
    fn test_tour_de_france_1935() {
        let route = [
            "1,4 July,Paris to Lille,262.0 km,Road stage",
            "2,5 July,Lille to Charleville,192.0 km,Road stage",
            "3,6 July,Charleville to Metz,161.0 km,Road stage",
            "4,7 July,Metz to Belfort,220.0 km,Road stage",
            "5a,8 July,Belfort to Geneva (Switzerland),262.0 km,Road stage",
            "5b,8 July,Geneva (Switzerland) to Evian,58.0 km,Individual time trial",
            ",9 July,Rest day,Evian",
            "6,10 July,Evian to Aix-les-Bains,207.0 km,Road stage",
            "7,11 July,Aix-les-Bains to Grenoble,229.0 km,Road stage",
            "8,12 July,Grenoble to Gap,102.0 km,Road stage",
            "9,13 July,Gap to Digne,227.0 km,Road stage",
            "10,14 July,Digne to Nice,156.0 km,Road stage",
            ",15 July,Rest day,Nice",
            "11,16 July,Nice to Cannes,126.0 km,Road stage",
            "12,17 July,Cannes to Marseille,195.0 km,Road stage",
            "13a,18 July,Marseille to Nîmes,112.0 km,Road stage",
            "13b,18 July,Nîmes to Montpellier,56.0 km,Individual time trial",
            "14a,19 July,Montpellier to Narbonne,103.0 km,Road stage",
            "14b,19 July,Narbonne to Perpignan,63.0 km,Individual time trial",
            "15,20 July,Perpignan to Luchon,325.0 km,Road stage",
            ",21 July,Rest day,Luchon",
            "16,22 July,Luchon to Pau,194.0 km,Road stage",
            ",23 July,Rest day,Pau",
            "17,24 July,Pau to Bordeaux,224.0 km,Road stage",
            "18a,25 July,Bordeaux to Rochefort,103.0 km,Road stage",
            "18b,25 July,Rochefort to La Rochelle,33.0 km,Individual time trial",
            "19a,26 July,La Rochelle to La Roche-sur-Yon,81.0 km,Road stage",
            "19b,26 July,La Roche-sur-Yon to Nantes,90.0 km,Individual time trial",
            "20a,27 July,Nantes to Vire,220.0 km,Road stage",
            "20b,27 July,Vire to Caen,55.0 km,Individual time trial",
            "21,28 July,Caen to Paris,221.0 km,Road stage",
        ];

        let edition = tour_de_france_1935();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 6 split stages");
    }

    #[test]
    fn test_tour_de_france_1936() {
        let mut route: Vec<String> = Vec::new();
        route.push("1,7 July,Paris to Lille,258.0 km,Road stage".to_string());
        route.push("2,8 July,Lille to Charleville,192.0 km,Road stage".to_string());
        route.push("3,9 July,Charleville to Metz,161.0 km,Road stage".to_string());
        route.push("4,10 July,Metz to Belfort,220.0 km,Road stage".to_string());
        route.push("5,11 July,Belfort to Evian-les-Bains,298.0 km,Road stage".to_string());
        route.push(",12 July,Rest day,Evian-les-Bains".to_string());
        route.push("6,13 July,Evian-les-Bains to Aix-les-Bains,212.0 km,Road stage".to_string());
        route.push("7,14 July,Aix-les-Bains to Grenoble,230.0 km,Road stage".to_string());
        route.push("8,15 July,Grenoble to Briançon,194.0 km,Road stage".to_string());
        route.push("9,16 July,Briançon to Digne,220.0 km,Road stage".to_string());
        route.push(",17 July,Rest day,Digne".to_string());
        route.push("10,18 July,Digne to Nice,156.0 km,Road stage".to_string());
        route.push("11,19 July,Nice to Cannes,126.0 km,Road stage".to_string());
        route.push(",20 July,Rest day,Cannes".to_string());
        route.push("12,21 July,Cannes to Marseille,195.0 km,Road stage".to_string());
        route.push("13a,22 July,Marseille to Nîmes,112.0 km,Road stage".to_string());
        route.push("13b,22 July,Nîmes to Montpellier,52.0 km,Individual time trial".to_string());
        route.push("14a,23 July,Montpellier to Narbonne,103.0 km,Road stage".to_string());
        route.push("14b,23 July,Narbonne to Perpignan,63.0 km,Individual time trial".to_string());
        route.push(",24 July,Rest day,Perpignan".to_string());
        route.push("15,25 July,Perpignan to Luchon,325.0 km,Road stage".to_string());
        route.push(",26 July,Rest day,Luchon".to_string());
        route.push("16,27 July,Luchon to Pau,194.0 km,Road stage".to_string());
        route.push(",28 July,Rest day,Pau".to_string());
        route.push("17,29 July,Pau to Bordeaux,229.0 km,Road stage".to_string());
        route.push("18a,30 July,Bordeaux to Saintes,117.0 km,Road stage".to_string());
        route.push("18b,30 July,Saintes to La Rochelle,75.0 km,Individual time trial".to_string());
        route.push("19a,31 July,La Rochelle to La Roche-sur-Yon,81.0 km,Road stage".to_string());
        route.push("19b,31 July,La Roche-sur-Yon to Cholet,65.0 km,Individual time trial".to_string());
        route.push("19c,31 July,Cholet to Angers,67.0 km,Road stage".to_string());
        route.push("20a,1 August,Angers to Vire,204.0 km,Road stage".to_string());
        route.push("20b,1 August,Vire to Caen,55.0 km,Individual time trial".to_string());
        route.push("21,2 August,Caen to Paris,234.0 km,Road stage".to_string());

        let edition = tour_de_france_1936();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 5 split stages");
    }

    #[test]
    fn test_tour_de_france_1937() {
        let mut route: Vec<String> = Vec::new();
        route.push("1,30 June,Paris to Lille,263.0 km,Road stage".to_string());
        route.push("2,1 July,Lille to Charleville,192.0 km,Road stage".to_string());
        route.push("3,2 July,Charleville to Metz,161.0 km,Road stage".to_string());
        route.push("4,3 July,Metz to Belfort,220.0 km,Road stage".to_string());
        route.push("5a,4 July,Belfort to Lons-le-Saunier,175.0 km,Road stage".to_string());
        route.push("5b,4 July,Lons-le-Saunier to Champagnole,34.0 km,Team time trial".to_string());
        route.push("5c,4 July,Champagnole to Geneva (Switzerland),93.0 km,Road stage".to_string());
        route.push(",5 July,Rest day,Geneva (Switzerland)".to_string());
        route.push("6,6 July,Geneva (Switzerland) to Aix-les-Bains,180.0 km,Road stage".to_string());
        route.push("7,7 July,Aix-les-Bains to Grenoble,228.0 km,Road stage".to_string());
        route.push("8,8 July,Grenoble to Briançon,194.0 km,Road stage".to_string());
        route.push("9,9 July,Briançon to Digne,220.0 km,Road stage".to_string());
        route.push(",10 July,Rest day,Digne".to_string());
        route.push("10,11 July,Digne to Nice,251.0 km,Road stage".to_string());
        route.push(",12 July,Rest day,Nice".to_string());
        route.push("11a,13 July,Nice to Toulon,169.0 km,Road stage".to_string());
        route.push("11b,13 July,Toulon to Marseille,65.0 km,Team time trial".to_string());
        route.push("12a,14 July,Marseille to Nîmes,112.0 km,Road stage".to_string());
        route.push("12b,14 July,Nîmes to Montpellier,51.0 km,Road stage".to_string());
        route.push("13a,15 July,Montpellier to Narbonne,51.0 km,Road stage".to_string());
        route.push("13b,15 July,Narbonne to Perpignan,63.0 km,Road stage".to_string());
        route.push(",16 July,Rest day,Perpignan".to_string());
        route.push("14a,17 July,Perpignan to Bourg-Madame,99.0 km,Road stage".to_string());
        route.push("14b,17 July,Bourg-Madame to Ax-les-Thermes,59.0 km,Road stage".to_string());
        route.push("14c,17 July,Ax-les-Thermes to Luchon,167.0 km,Road stage".to_string());
        route.push(",18 July,Rest day,Luchon".to_string());
        route.push("15,19 July,Luchon to Pau,194.0 km,Road stage".to_string());
        route.push(",20 July,Rest day,Pau".to_string());
        route.push("16,21 July,Pau to Bordeaux,235.0 km,Road stage".to_string());
        route.push("17a,22 July,Bordeaux to Royan,123.0 km,Road stage".to_string());
        route.push("17b,22 July,Royan to Saintes,37.0 km,Road stage".to_string());
        route.push("17c,22 July,Saintes to La Rochelle,37.0 km,Road stage".to_string());
        route.push("18a,23 July,La Rochelle to La Roche-sur-Yon,82.0 km,Team time trial".to_string());
        route.push("18b,23 July,La Roche-sur-Yon to Rennes,172.0 km,Road stage".to_string());
        route.push("19a,24 July,Rennes to Vire,114.0 km,Road stage".to_string());
        route.push("19b,24 July,Vire to Caen,59.0 km,Individual time trial".to_string());
        route.push("20,25 July,Caen to Paris,234.0 km,Road stage".to_string());
 
        let edition = tour_de_france_1937();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages including 8 split stages");
    }

    #[test]
    fn test_tour_de_france_1938() {
        let mut route: Vec<String> = Vec::new();
        route.push("1,5 July,Paris to Caen,215.0 km,Road stage".to_string());
        route.push("2,6 July,Caen to Saint-Brieuc,237.0 km,Road stage".to_string());
        route.push("3,7 July,Saint-Brieuc to Nantes,238.0 km,Road stage".to_string());
        route.push("4a,8 July,Nantes to La Roche-sur-Yon,62.0 km,Road stage".to_string());
        route.push("4b,8 July,La Roche-sur-Yon to La Rochelle,83.0 km,Road stage".to_string());
        route.push("4c,8 July,La Rochelle to Royan,83.0 km,Road stage".to_string());
        route.push(",9 July,Rest day,Royan".to_string());
        route.push("5,10 July,Royan to Bordeaux,198.0 km,Road stage".to_string());
        route.push("6a,11 July,Bordeaux to Arcachon,53.0 km,Road stage".to_string());
        route.push("6b,11 July,Arcachon to Bayonne,171.0 km,Road stage".to_string());
        route.push("7,12 July,Bayonne to Pau,115.0 km,Road stage".to_string());
        route.push(",13 July,Rest day,Pau".to_string());
        route.push("8,14 July,Pau to Luchon,193.0 km,Road stage".to_string());
        route.push(",15 July,Rest day,Luchon".to_string());
        route.push("9,16 July,Luchon to Perpignan,260.0 km,Road stage".to_string());
        route.push("10a,17 July,Perpignan to Narbonne,63.0 km,Road stage".to_string());
        route.push("10b,17 July,Narbonne to Beziers,27.0 km,Individual time trial".to_string());
        route.push("10c,17 July,Beziers to Montpellier,73.0 km,Road stage".to_string());
        route.push("11,18 July,Montpellier to Marseille,223.0 km,Road stage".to_string());
        route.push("12,19 July,Marseille to Cannes,199.0 km,Road stage".to_string());
        route.push(",20 July,Rest day,Cannes".to_string());
        route.push("13,21 July,Cannes to Digne,284.0 km,Road stage".to_string());
        route.push("14,22 July,Digne to Briançon,219.0 km,Road stage".to_string());
        route.push("15,23 July,Briançon to Aix-les-Bains,311.0 km,Road stage".to_string());
        route.push(",24 July,Rest day,Aix-les-Bains".to_string());
        route.push("16,25 July,Aix-les-Bains to Besançon,284.0 km,Road stage".to_string());
        route.push("17a,26 July,Besançon to Belfort,89.0 km,Road stage".to_string());
        route.push("17b,26 July,Belfort to Strasbourg,143.0 km,Road stage".to_string());
        route.push("18,27 July,Strasbourg to Metz,186.0 km,Road stage".to_string());
        route.push("19,28 July,Metz to Reims,196.0 km,Road stage".to_string());
        route.push(",29 July,Rest day,Reims".to_string());
        route.push("20a,30 July,Reims to Laon,48.0 km,Road stage".to_string());
        route.push("20b,30 July,Laon to Saint-Quentin,42.0 km,Individual time trial".to_string());
        route.push("20c,30 July,Saint-Quentin to Lille,107.0 km,Road stage".to_string());
        route.push("21,31 July,Lille to Paris,279.0 km,Road stage".to_string());
 
        let edition = tour_de_france_1938();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 5 split stages");
    }

    #[test]
    fn test_tour_de_france_1939() {
        let route = [
            "1,10 July,Paris to Caen,215.0 km,Road stage",
            "2a,11 July,Caen to Vire,64.0 km,Individual time trial",
            "2b,11 July,Vire to Rennes,119.0 km,Road stage",
            "3,12 July,Rennes to Brest,244.0 km,Road stage",
            "4,13 July,Brest to Lorient,174.0 km,Road stage",
            "5,14 July,Lorient to Nantes,207.0 km,Road stage",
            "6a,15 July,Nantes to La Rochelle,144.0 km,Road stage",
            "6b,15 July,La Rochelle to Royan,107.0 km,Road stage",
            ",16 July,Rest day,Royan",
            "7,17 July,Royan to Bordeaux,198.0 km,Road stage",
            "8a,18 July,Bordeaux to Salies-de-Bearn,210.0 km,Road stage",
            "8b,18 July,Salies-de-Bearn to Pau,69.0 km,Individual time trial",
            "9,19 July,Pau to Toulouse,311.0 km,Road stage",
            ",20 July,Rest day,Toulouse",
            "10a,21 July,Toulouse to Narbonne,149.0 km,Road stage",
            "10b,21 July,Narbonne to Beziers,27.0 km,Individual time trial",
            "10c,21 July,Beziers to Montpellier,70.0 km,Road stage",
            "11,22 July,Montpellier to Marseille,212.0 km,Road stage",
            "12a,23 July,Marseille to Saint-Raphael,157.0 km,Road stage",
            "12b,23 July,Saint-Raphael to Monaco (Monaco),122.0 km,Road stage",
            "13,24 July,Monaco (Monaco),101.0 km,Road stage",
            "14,25 July,Monaco (Monaco) to Digne,175.0 km,Road stage",
            "15,26 July,Digne to Briançon,219.0 km,Road stage",
            "16a,27 July,Briançon,126.0 km,Road stage",
            "16b,27 July,Bonneval to Bourg-Saint-Maurice,64.0 km,Individual time trial",
            "16c,27 July,Bourg-Saint-Maurice to Annecy,104.0 km,Road stage",
            ",28 July,Rest day,Annecy",
            "17a,29 July,Annecy to Dole,226.0 km,Road stage",
            "17b,29 July,Dole to Dijon,59.0 km,Individual time trial",
            "18a,30 July,Dijon to Troyes,151.0 km,Road stage",
            "18b,30 July,Troyes to Paris,201.0 km,Road stage",
        ];
        let edition = tour_de_france_1939();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "18 stages including 8 split stages");
    }

    #[test]
    fn test_tour_de_france_1947() {
        let route = [
            "1,25 June,Paris to Lille,236.0 km,Road stage",
            "2,26 June,Lille to Brussels (Belgium),182.0 km,Road stage",
            "3,27 June,Brussels (Belgium) to Luxembourg City (Luxembourg),314.0 km,Road stage",
            "4,28 June,Luxembourg City (Luxembourg) to Strasbourg,223.0 km,Road stage",
            "5,29 June,Strasbourg to Besançon,248.0 km,Road stage",
            ",30 June,Rest day,Besançon",
            "6,1 July,Besançon to Lyon,249.0 km,Road stage",
            "7,2 July,Lyon to Grenoble,172.0 km,Road stage",
            "8,3 July,Grenoble to Briançon,185.0 km,Road stage",
            ",4 July,Rest day,Briançon",
            "9,5 July,Briançon to Digne,217.0 km,Road stage",
            "10,6 July,Digne to Nice,255.0 km,Road stage",
            ",7 July,Rest day,Nice",
            "11,8 July,Nice to Marseille,230.0 km,Road stage",
            "12,9 July,Marseille to Montpellier,165.0 km,Road stage",
            "13,10 July,Montpellier to Carcassonne,172.0 km,Road stage",
            "14,11 July,Carcassonne to Luchon,253.0 km,Road stage",
            ",12 July,Rest day,Luchon",
            "15,13 July,Luchon to Pau,195.0 km,Road stage",
            "16,14 July,Pau to Bordeaux,195.0 km,Road stage",
            "17,15 July,Bordeaux to Les Sables-d'Olonne,272.0 km,Road stage",
            "18,16 July,Les Sables-d'Olonne to Vannes,236.0 km,Road stage",
            ",17 July,Rest day,Vannes",
            "19,18 July,Vannes to Saint-Brieuc,139.0 km,Individual time trial",
            "20,19 July,Saint-Brieuc to Caen,235.0 km,Road stage",
            "21,20 July,Caen to Paris,257.0 km,Road stage",
        ];
        let edition = tour_de_france_1947();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_1948() {
        let route = [
            "1,30 June,Paris to Trouville,237.0 km,Road stage",
            "2,1 July,Trouville to Dinard,259.0 km,Road stage",
            "3,2 July,Dinard to Nantes,251.0 km,Road stage",
            "4,3 July,Nantes to La Rochelle,166.0 km,Road stage",
            "5,4 July,La Rochelle to Bordeaux,262.0 km,Road stage",
            "6,5 July,Bordeaux to Biarritz,244.0 km,Road stage",
            ",6 July,Rest day,Biarritz",
            "7,7 July,Biarritz to Lourdes,219.0 km,Road stage",
            "8,8 July,Lourdes to Toulouse,261.0 km,Road stage",
            ",9 July,Rest day,Toulouse",
            "9,10 July,Toulouse to Montpellier,246.0 km,Road stage",
            "10,11 July,Montpellier to Marseille,248.0 km,Road stage",
            "11,12 July,Marseille to San Remo (Italy),245.0 km,Road stage",
            "12,13 July,San Remo (Italy) to Cannes,170.0 km,Road stage",
            ",14 July,Rest day,Cannes",
            "13,15 July,Cannes to Briançon,274.0 km,Road stage",
            "14,16 July,Briançon to Aix-les-Bains,263.0 km,Road stage",
            ",17 July,Rest day,Aix-les-Bains",
            "15,18 July,Aix-les-Bains to Lausanne (Switzerland),256.0 km,Road stage",
            "16,19 July,Lausanne (Switzerland) to Mulhouse,243.0 km,Road stage",
            ",20 July,Rest day,Mulhouse",
            "17,21 July,Mulhouse to Strasbourg,120.0 km,Individual time trial",
            "18,22 July,Strasbourg to Metz,195.0 km,Road stage",
            "19,23 July,Metz to Liège (Belgium),249.0 km,Road stage",
            "20,24 July,Liège (Belgium) to Roubaix,228.0 km,Road stage",
            "21,25 July,Roubaix to Paris,286.0 km,Road stage",
        ];
        let edition = tour_de_france_1948();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_1949() {
        let route = [
            "1,30 June,Paris to Reims,182.0 km,Road stage",
            "2,1 July,Reims to Brussels (Belgium),273.0 km,Road stage",
            "3,2 July,Brussels (Belgium) to Boulogne-sur-Mer,211.0 km,Road stage",
            "4,3 July,Boulogne-sur-Mer to Rouen,185.0 km,Road stage",
            "5,4 July,Rouen to Saint-Malo,293.0 km,Road stage",
            "6,5 July,Saint-Malo to Les Sables-d'Olonne,305.0 km,Road stage",
            ",6 July,Rest day,Les Sables-d'Olonne",
            "7,7 July,Les Sables-d'Olonne to La Rochelle,92.0 km,Individual time trial",
            "8,8 July,La Rochelle to Bordeaux,262.0 km,Road stage",
            "9,9 July,Bordeaux to San Sebastián (Spain),228.0 km,Road stage",
            "10,10 July,San Sebastián (Spain) to Pau,192.0 km,Road stage",
            ",11 July,Rest day,Pau",
            "11,12 July,Pau to Luchon,193.0 km,Road stage",
            "12,13 July,Luchon to Toulouse,134.0 km,Road stage",
            "13,14 July,Toulouse to Nîmes,289.0 km,Road stage",
            "14,15 July,Nîmes to Marseille,199.0 km,Road stage",
            "15,16 July,Marseille to Cannes,215.0 km,Road stage",
            ",17 July,Rest day,Cannes",
            "16,18 July,Cannes to Briançon,275.0 km,Road stage",
            "17,19 July,Briançon to Aosta (Italy),257.0 km,Road stage",
            ",20 July,Rest day,Aosta (Italy)",
            "18,21 July,Aosta (Italy) to Lausanne (Switzerland),265.0 km,Road stage",
            "19,22 July,Lausanne (Switzerland) to Colmar,283.0 km,Road stage",
            "20,23 July,Colmar to Nancy,137.0 km,Individual time trial",
            "21,24 July,Nancy to Paris,340.0 km,Road stage",
        ];
        let edition = tour_de_france_1949();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_1950() {
        let route = [
            "1,13 July,Paris to Metz,307.0 km,Road stage",
            "2,14 July,Metz to Liège (Belgium),241.0 km,Road stage",
            "3,15 July,Liège (Belgium) to Lille,232.5 km,Road stage",
            "4,16 July,Lille to Rouen,231.0 km,Road stage",
            "5,17 July,Rouen to Dinard,316.0 km,Road stage",
            ",18 July,Rest day,Dinard",
            "6,19 July,Dinard to Saint-Brieuc,78.0 km,Individual time trial",
            "7,20 July,Saint-Brieuc to Angers,248.0 km,Road stage",
            "8,21 July,Angers to Niort,181.0 km,Road stage",
            "9,22 July,Niort to Bordeaux,206.0 km,Road stage",
            "10,23 July,Bordeaux to Pau,202.0 km,Road stage",
            ",24 July,Rest day,Pau",
            "11,25 July,Pau to Saint-Gaudens,230.0 km,Road stage",
            "12,26 July,Saint-Gaudens to Perpignan,233.0 km,Road stage",
            "13,27 July,Perpignan to Nîmes,215.0 km,Road stage",
            "14,28 July,Nîmes to Toulon,222.0 km,Road stage",
            "15,29 July,Toulon to Menton,205.5 km,Road stage",
            "16,30 July,Menton to Nice,96.0 km,Road stage",
            ",31 July,Rest day,Nice",
            "17,1 August,Nice to Gap,229.0 km,Road stage",
            "18,2 August,Gap to Briançon,165.0 km,Road stage",
            "19,3 August,Briançon to Saint-Étienne,291.0 km,Road stage",
            ",4 August,Rest day,Saint-Étienne",
            "20,5 August,Saint-Étienne to Lyon,98.0 km,Individual time trial",
            "21,6 August,Lyon to Dijon,233.0 km,Road stage",
            "22,7 August,Dijon to Paris,314.0 km,Road stage",
        ];
        let edition = tour_de_france_1950();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
    }

    #[test]
    fn test_tour_de_france_1951() {
        let route = [
            "1,4 July,Metz to Reims,185.0 km,Road stage",
            "2,5 July,Reims to Ghent (Belgium),228.0 km,Road stage",
            "3,6 July,Ghent (Belgium) to Le Treport,219.0 km,Road stage",
            "4,7 July,Le Treport to Paris,188.0 km,Road stage",
            "5,8 July,Paris to Caen,215.0 km,Road stage",
            "6,9 July,Caen to Rennes,182.0 km,Road stage",
            "7,10 July,La Guerche-de-Bretagne to Angers,85.0 km,Individual time trial",
            "8,11 July,Angers to Limoges,241.0 km,Road stage",
            ",12 July,Rest day,Limoges",
            "9,13 July,Limoges to Clermont-Ferrand,236.0 km,Road stage",
            "10,14 July,Clermont-Ferrand to Brive,216.0 km,Road stage",
            "11,15 July,Brive to Agen,177.0 km,Road stage",
            "12,16 July,Agen to Dax,185.0 km,Road stage",
            "13,17 July,Dax to Tarbes,201.0 km,Road stage",
            "14,18 July,Tarbes to Luchon,142.0 km,Road stage",
            "15,19 July,Luchon to Carcassonne,213.0 km,Road stage",
            "16,20 July,Carcassonne to Montpellier,192.0 km,Road stage",
            ",21 July,Rest day,Montpellier",
            "17,22 July,Montpellier to Avignon,224.0 km,Road stage",
            "18,23 July,Avignon to Marseille,173.0 km,Road stage",
            "19,24 July,Marseille to Gap,208.0 km,Road stage",
            "20,25 July,Gap to Briançon,165.0 km,Road stage",
            "21,26 July,Briançon to Aix-les-Bains,201.0 km,Road stage",
            "22,27 July,Aix-les-Bains to Geneva (Switzerland),97.0 km,Individual time trial",
            "23,28 July,Geneva (Switzerland) to Dijon,197.0 km,Road stage",
            "24,29 July,Dijon to Paris,322.0 km,Road stage",
        ];
        let edition = tour_de_france_1951();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "24 stages");
    }

    #[test]
    fn test_tour_de_france_1952() {
        let route = [
            "1,25 June,Brest to Rennes,246.0 km,Road stage",
            "2,26 June,Rennes to Le Mans,181.0 km,Road stage",
            "3,27 June,Le Mans to Rouen,189.0 km,Road stage",
            "4,28 June,Rouen to Roubaix,232.0 km,Road stage",
            "5,29 June,Roubaix to Namur (Belgium),197.0 km,Road stage",
            "6,30 June,Namur (Belgium) to Metz,228.0 km,Road stage",
            "7,1 July,Metz to Nancy,60.0 km,Individual time trial",
            "8,2 July,Nancy to Mulhouse,252.0 km,Road stage",
            "9,3 July,Mulhouse to Lausanne (Switzerland),238.0 km,Road stage",
            "10,4 July,Lausanne (Switzerland) to Alpe d'Huez,266.0 km,Road stage",
            ",5 July,Rest day,Alpe d'Huez",
            "11,6 July,Le Bourg-d'Oisans to Sestrière (Italy),182.0 km,Road stage",
            "12,7 July,Sestrière (Italy) to Monaco (Monaco),251.0 km,Road stage",
            "13,8 July,Monaco (Monaco) to Aix-en-Provence,214.0 km,Road stage",
            "14,9 July,Aix-en-Provence to Avignon,178.0 km,Road stage",
            "15,10 July,Avignon to Perpignan,275.0 km,Road stage",
            "16,11 July,Perpignan to Toulouse,200.0 km,Road stage",
            ",12 July,Rest day,Toulouse",
            "17,13 July,Toulouse to Bagnères-de-Bigorre,204.0 km,Road stage",
            "18,14 July,Bagnères-de-Bigorre to Pau,149.0 km,Road stage",
            "19,15 July,Pau to Bordeaux,195.0 km,Road stage",
            "20,16 July,Bordeaux to Limoges,228.0 km,Road stage",
            "21,17 July,Limoges to Puy de Dôme,245.0 km,Road stage",
            "22,18 July,Clermont-Ferrand to Vichy,63.0 km,Individual time trial",
            "23,19 July,Vichy to Paris,354.0 km,Road stage",
        ];
        let edition = tour_de_france_1952();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "23 stages");
        assert_eq!(edition.summit_finishes(), 3);
    }

    #[test]
    fn test_tour_de_france_1953() {
        let route = [
            "1,3 July,Strasbourg to Metz,195.0 km,Road stage",
            "2,4 July,Metz to Liège (Belgium),227.0 km,Road stage",
            "3,5 July,Liège (Belgium) to Lille,221.0 km,Road stage",
            "4,6 July,Lille to Dieppe,188.0 km,Road stage",
            "5,7 July,Dieppe to Caen,200.0 km,Road stage",
            "6,8 July,Caen to Le Mans,206.0 km,Road stage",
            "7,9 July,Le Mans to Nantes,181.0 km,Road stage",
            "8,10 July,Nantes to Bordeaux,345.0 km,Road stage",
            ",11 July,Rest day,Bordeaux",
            "9,12 July,Bordeaux to Pau,197.0 km,Road stage",
            "10,13 July,Pau to Cauterets,103.0 km,Road stage",
            "11,14 July,Cauterets to Luchon,115.0 km,Road stage",
            "12,15 July,Luchon to Albi,228.0 km,Road stage",
            "13,16 July,Albi to Beziers,189.0 km,Road stage",
            "14,17 July,Beziers to Nîmes,214.0 km,Road stage",
            "15,18 July,Nîmes to Marseille,173.0 km,Road stage",
            "16,19 July,Marseille to Monaco (Monaco),236.0 km,Road stage",
            ",20 July,Rest day,Monaco (Monaco)",
            "17,21 July,Monaco (Monaco) to Gap,261.0 km,Road stage",
            "18,22 July,Gap to Briançon,165.0 km,Road stage",
            "19,23 July,Briançon to Lyon,227.0 km,Road stage",
            "20,24 July,Lyon to Saint-Étienne,70.0 km,Individual time trial",
            "21,25 July,Saint-Étienne to Montlucon,210.0 km,Road stage",
            "22,26 July,Montlucon to Paris,328.0 km,Road stage",
        ];
        let edition = tour_de_france_1953();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
        assert_eq!(edition.summit_finishes(), 1);
    }

    #[test]
    fn test_tour_de_france_1954() {
        let route = [
            "1,8 July,Amsterdam (Netherlands) to Brasschaat (Belgium),216.0 km,Road stage",
            "2,9 July,Beveren (Belgium) to Lille,255.0 km,Road stage",
            "3,10 July,Lille to Rouen,219.0 km,Road stage",
            "4a,11 July,Rouen to Circuit des Essarts,10.4 km,Team time trial",
            "4b,11 July,Rouen to Caen,131.0 km,Road stage",
            "5,12 July,Caen to Saint-Brieuc,224.0 km,Road stage",
            "6,13 July,Saint-Brieuc to Brest,179.0 km,Road stage",
            "7,14 July,Brest to Vannes,211.0 km,Road stage",
            "8,15 July,Vannes to Angers,190.0 km,Road stage",
            "9,16 July,Angers to Bordeaux,343.0 km,Road stage",
            ",17 July,Rest day,Bordeaux",
            "10,18 July,Bordeaux to Bayonne,202.0 km,Road stage",
            "11,19 July,Bayonne to Pau,241.0 km,Road stage",
            "12,20 July,Pau to Luchon,161.0 km,Road stage",
            "13,21 July,Luchon to Mulhouse,203.0 km,Road stage",
            "14,22 July,Mulhouse to Millau,225.0 km,Road stage",
            "15,23 July,Millau to Le Puy-en-Velay,197.0 km,Road stage",
            "16,24 July,Le Puy-en-Velay to Lyon,194.0 km,Road stage",
            ",25 July,Rest day,Lyon",
            "17,26 July,Lyon to Grenoble,182.0 km,Road stage",
            "18,27 July,Grenoble to Briançon,216.0 km,Road stage",
            "19,28 July,Briançon to Aix-les-Bains,221.0 km,Road stage",
            "20,29 July,Aix-les-Bains to Besançon,243.0 km,Road stage",
            "21a,30 July,Besançon to Epinal,134.0 km,Road stage",
            "21b,30 July,Epinal to Nancy,72.0 km,Individual time trial",
            "22,31 July,Nancy to Troyes,216.0 km,Road stage",
            "23,1 August,Troyes to Paris,180.0 km,Road stage",
        ];
        let edition = tour_de_france_1954();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "23 stages including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1955() {
        let route = [
            "1a,7 July,Le Havre to Dieppe,102.0 km,Road stage",
            "1b,7 July,Dieppe,12.5 km,Team time trial",
            "2,8 July,Dieppe to Roubaix,204.0 km,Road stage",
            "3,9 July,Roubaix to Namur (Belgium),210.0 km,Road stage",
            "4,10 July,Namur (Belgium) to Metz,225.0 km,Road stage",
            "5,11 July,Metz to Colmar,229.0 km,Road stage",
            "6,12 July,Colmar to Zurich (Switzerland),195.0 km,Road stage",
            "7,13 July,Zurich (Switzerland) to Thonon-les-Bains,267.0 km,Road stage",
            "8,14 July,Thonon-les-Bains to Briançon,253.0 km,Road stage",
            "9,15 July,Briançon to Monaco (Monaco),275.0 km,Road stage",
            ",16 July,Rest day,Monaco (Monaco)",
            "10,17 July,Monaco (Monaco) to Marseille,240.0 km,Road stage",
            "11,18 July,Marseille to Avignon,198.0 km,Road stage",
            "12,19 July,Avignon to Millau,240.0 km,Road stage",
            "13,20 July,Millau to Albi,205.0 km,Road stage",
            "14,21 July,Albi to Narbonne,156.0 km,Road stage",
            "15,22 July,Narbonne to Ax-les-Thermes,151.0 km,Road stage",
            ",23 July,Rest day,Ax-les-Thermes",
            "16,24 July,Ax-les-Thermes to Toulouse,123.0 km,Road stage",
            "17,25 July,Toulouse to Saint-Gaudens,250.0 km,Road stage",
            "18,26 July,Saint-Gaudens to Pau,205.0 km,Road stage",
            "19,27 July,Pau to Bordeaux,195.0 km,Road stage",
            "20,28 July,Bordeaux to Poitiers,243.0 km,Road stage",
            "21,29 July,Chatellerault to Tours,68.6 km,Individual time trial",
            "22,30 July,Tours to Paris,229.0 km,Road stage",
        ];
        let edition = tour_de_france_1955();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }

    #[test]
    fn test_tour_de_france_1956() {
        let route = [
            "1,5 July,Reims to Liège (Belgium),223.0 km,Road stage",
            "2,6 July,Liège (Belgium) to Lille,217.0 km,Road stage",
            "3,7 July,Lille to Rouen,225.0 km,Road stage",
            "4a,8 July,Circuit de Rouen-Les-Essarts,15.1 km,Individual time trial",
            "4b,8 July,Rouen to Caen,125.0 km,Road stage",
            "5,9 July,Caen to Saint-Malo,189.0 km,Road stage",
            "6,10 July,Saint-Malo to Lorient,192.0 km,Road stage",
            "7,11 July,Lorient to Angers,244.0 km,Road stage",
            "8,12 July,Angers to La Rochelle,180.0 km,Road stage",
            "9,13 July,La Rochelle to Bordeaux,219.0 km,Road stage",
            ",14 July,Rest day,Bordeaux",
            "10,15 July,Bordeaux to Bayonne,201.0 km,Road stage",
            "11,16 July,Bayonne to Pau,255.0 km,Road stage",
            "12,17 July,Pau to Luchon,130.0 km,Road stage",
            "13,18 July,Luchon to Toulouse,176.0 km,Road stage",
            "14,19 July,Toulouse to Montpellier,231.0 km,Road stage",
            "15,20 July,Montpellier to Aix-en-Provence,204.0 km,Road stage",
            ",21 July,Rest day,Aix-en-Provence",
            "16,22 July,Aix-en-Provence to Gap,203.0 km,Road stage",
            "17,23 July,Gap to Turin (Italy),234.0 km,Road stage",
            "18,24 July,Turin (Italy) to Grenoble,250.0 km,Road stage",
            "19,25 July,Grenoble to Saint-Étienne,173.0 km,Road stage",
            "20,26 July,Saint-Étienne to Lyon,73.0 km,Individual time trial",
            "21,27 July,Lyon to Montlucon,237.0 km,Road stage",
            "22,28 July,Montlucon to Paris,331.0 km,Road stage",
        ];
        let edition = tour_de_france_1956();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 1 split stage");
    }

    #[test]
    fn test_tour_de_france_1957() {
        let route = [
            "1,27 June,Nantes to Granville,204.0 km,Road stage",
            "2,28 June,Granville to Caen,226.0 km,Road stage",
            "3a,29 June,Circuit de la Prairie,15.0 km,Team time trial",
            "3b,29 June,Caen to Rouen,134.0 km,Road stage",
            "4,30 June,Rouen to Roubaix,232.0 km,Road stage",
            "5,1 July,Roubaix to Charleroi (Belgium),170.0 km,Road stage",
            "6,2 July,Charleroi (Belgium) to Metz,248.0 km,Road stage",
            "7,3 July,Metz to Colmar,223.0 km,Road stage",
            "8,4 July,Colmar to Besançon,192.0 km,Road stage",
            "9,5 July,Besançon to Thonon-les-Bains,188.0 km,Road stage",
            ",6 July,Rest day,Thonon-les-Bains",
            "10,7 July,Thonon-les-Bains to Briançon,247.0 km,Road stage",
            "11,8 July,Briançon to Cannes,286.0 km,Road stage",
            "12,9 July,Cannes to Marseille,239.0 km,Road stage",
            "13,10 July,Marseille to Ales,160.0 km,Road stage",
            "14,11 July,Ales to Perpignan,246.0 km,Road stage",
            "15a,12 July,Perpignan to Barcelona (Spain),197.0 km,Road stage",
            "15b,12 July,Montjuic circuit (Spain),9.8 km,Individual time trial",
            ",13 July,Rest day,Barcelona (Spain)",
            "16,14 July,Barcelona (Spain) to Ax-les-Thermes,220.0 km,Road stage",
            "17,15 July,Ax-les-Thermes to Saint-Gaudens,236.0 km,Road stage",
            "18,16 July,Saint-Gaudens to Pau,207.0 km,Road stage",
            "19,17 July,Pau to Bordeaux,194.0 km,Road stage",
            "20,18 July,Bordeaux to Libourne,66.0 km,Individual time trial",
            "21,19 July,Libourne to Tours,317.0 km,Road stage",
            "22,20 July,Tours to Paris,227.0 km,Road stage",
        ];
        let edition = tour_de_france_1957();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1958() {
        let route = [
            "1,26 June,Brussels (Belgium) to Ghent (Belgium),184.0 km,Road stage",
            "2,27 June,Ghent (Belgium) to Dunkerque,198.0 km,Road stage",
            "3,28 June,Dunkerque to Mers-les-Bains,177.0 km,Road stage",
            "4,29 June,Le Treport to Versailles,205.0 km,Road stage",
            "5,30 June,Versailles to Caen,232.0 km,Road stage",
            "6,1 July,Caen to Saint-Brieuc,223.0 km,Road stage",
            "7,2 July,Saint-Brieuc to Brest,170.0 km,Road stage",
            "8,3 July,Châteaulin,46.0 km,Individual time trial",
            "9,4 July,Quimper to Saint-Nazaire,206.0 km,Road stage",
            "10,5 July,Saint-Nazaire to Royan,255.0 km,Road stage",
            "11,6 July,Royan to Bordeaux,137.0 km,Road stage",
            "12,7 July,Bordeaux to Dax,161.0 km,Road stage",
            "13,8 July,Dax to Pau,230.0 km,Road stage",
            "14,9 July,Pau to Luchon,129.0 km,Road stage",
            "15,10 July,Luchon to Toulouse,176.0 km,Road stage",
            "16,11 July,Toulouse to Beziers,187.0 km,Road stage",
            "17,12 July,Beziers to Nîmes,189.0 km,Road stage",
            "18,13 July,Bedoin to Mont Ventoux,21.0 km,Individual time trial",
            "19,14 July,Carpentras to Gap,178.0 km,Road stage",
            "20,15 July,Gap to Briançon,165.0 km,Road stage",
            "21,16 July,Briançon to Aix-les-Bains,219.0 km,Road stage",
            "22,17 July,Aix-les-Bains to Besançon,237.0 km,Road stage",
            "23,18 July,Besançon to Dijon,74.0 km,Individual time trial",
            "24,19 July,Dijon to Paris,320.0 km,Road stage",
        ];
        let edition = tour_de_france_1958();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "24 stages");
        assert_eq!(edition.summit_finishes(), 1);
    }

    #[test]
    fn test_tour_de_france_1959() {
        let route = [
            "1,25 June,Mulhouse to Metz,238.0 km,Road stage",
            "2,26 June,Metz to Namur (Belgium),234.0 km,Road stage",
            "3,27 June,Namur (Belgium) to Roubaix,217.0 km,Road stage",
            "4,28 June,Roubaix to Rouen,230.0 km,Road stage",
            "5,29 June,Rouen to Rennes,286.0 km,Road stage",
            "6,30 June,Blain to Nantes,45.0 km,Individual time trial",
            "7,1 July,Nantes to La Rochelle,190.0 km,Road stage",
            "8,2 July,La Rochelle to Bordeaux,201.0 km,Road stage",
            "9,3 July,Bordeaux to Bayonne,207.0 km,Road stage",
            ",4 July,Rest day,Bayonne",
            "10,5 July,Bayonne to Bagnères-de-Bigorre,235.0 km,Road stage",
            "11,6 July,Bagnères-de-Bigorre to Saint-Gaudens,119.0 km,Road stage",
            "12,7 July,Saint-Gaudens to Albi,184.0 km,Road stage",
            "13,8 July,Albi to Aurillac,219.0 km,Road stage",
            "14,9 July,Aurillac to Clermont-Ferrand,231.0 km,Road stage",
            "15,10 July,Puy de Dôme,12.0 km,Individual time trial",
            "16,11 July,Clermont-Ferrand to Saint-Étienne,210.0 km,Road stage",
            ",12 July,Rest day,Saint-Étienne",
            "17,13 July,Saint-Étienne to Grenoble,197.0 km,Road stage",
            "18,14 July,Grenoble to Saint-Vincent (Italy),243.0 km,Road stage",
            "19,15 July,Saint-Vincent (Italy) to Annecy,251.0 km,Road stage",
            "20,16 July,Annecy to Chalon-sur-Saône,202.0 km,Road stage",
            "21,17 July,Seurre to Dijon,69.0 km,Individual time trial",
            "22,18 July,Dijon to Paris,331.0 km,Road stage",
        ];
        let edition = tour_de_france_1959();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages");
        assert_eq!(edition.summit_finishes(), 1);
    }

    #[test]
    fn test_tour_de_france_1960() {
        let route = [
            "1a,26 June,Lille to Brussels (Belgium),108.0 km,Road stage",
            "1b,26 June,Brussels (Belgium),27.8 km,Individual time trial",
            "2,27 June,Brussels (Belgium) to Dunkerque,206.0 km,Road stage",
            "3,28 June,Dunkerque to Dieppe,209.0 km,Road stage",
            "4,29 June,Dieppe to Caen,211.0 km,Road stage",
            "5,30 June,Caen to Saint-Malo,189.0 km,Road stage",
            "6,1 July,Saint-Malo to Lorient,191.0 km,Road stage",
            "7,2 July,Lorient to Angers,244.0 km,Road stage",
            "8,3 July,Angers to Limoges,240.0 km,Road stage",
            "9,4 July,Limoges to Bordeaux,225.0 km,Road stage",
            "10,5 July,Mont-de-Marsan to Pau,228.0 km,Road stage",
            "11,6 July,Pau to Luchon,161.0 km,Road stage",
            "12,7 July,Luchon to Toulouse,176.0 km,Road stage",
            "13,8 July,Toulouse to Millau,224.0 km,Road stage",
            ",9 July,Rest day,Millau",
            "14,10 July,Millau to Avignon,217.0 km,Road stage",
            "15,11 July,Avignon to Gap,187.0 km,Road stage",
            "16,12 July,Gap to Briançon,172.0 km,Road stage",
            "17,13 July,Briançon to Aix-les-Bains,229.0 km,Road stage",
            "18,14 July,Aix-les-Bains to Thonon-les-Bains,215.0 km,Road stage",
            "19,15 July,Pontarlier to Besançon,83.0 km,Individual time trial",
            "20,16 July,Besançon to Troyes,229.0 km,Road stage",
            "21,17 July,Troyes to Paris,200.0 km,Road stage",
        ];
        let edition = tour_de_france_1960();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
    }

    #[test]
    fn test_tour_de_france_1961() {
        let route = [
            "1a,25 June,Rouen to Versailles,136.5 km,Road stage",
            "1b,25 June,Versailles,28.5 km,Individual time trial",
            "2,26 June,Pontoise to Roubaix,230.5 km,Road stage",
            "3,27 June,Roubaix to Charleroi (Belgium),197.5 km,Road stage",
            "4,28 June,Charleroi (Belgium) to Metz,237.5 km,Road stage",
            "5,29 June,Metz to Strasbourg,221.0 km,Road stage",
            "6,30 June,Strasbourg to Belfort,180.5 km,Road stage",
            "7,1 July,Belfort to Chalon-sur-Saône,214.5 km,Road stage",
            "8,2 July,Chalon-sur-Saône to Saint-Étienne,240.5 km,Road stage",
            "9,3 July,Saint-Étienne to Grenoble,230.0 km,Road stage",
            "10,4 July,Grenoble to Turin (Italy),250.5 km,Road stage",
            "11,5 July,Turin (Italy) to Antibes,225.0 km,Road stage",
            "12,6 July,Antibes to Aix-en-Provence,199.0 km,Road stage",
            "13,7 July,Aix-en-Provence to Montpellier,177.5 km,Road stage",
            ",8 July,Rest day,Montpellier",
            "14,9 July,Montpellier to Perpignan,174.0 km,Road stage",
            "15,10 July,Perpignan to Toulouse,206.0 km,Road stage",
            "16,11 July,Toulouse to Superbagnères,208.0 km,Road stage",
            "17,12 July,Luchon to Pau,197.0 km,Road stage",
            "18,13 July,Pau to Bordeaux,207.0 km,Road stage",
            "19,14 July,Bergerac to Périgueux,74.5 km,Individual time trial",
            "20,15 July,Périgueux to Tours,309.5 km,Road stage",
            "21,16 July,Tours to Paris,252.5 km,Road stage",
        ];
        let edition = tour_de_france_1961();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 1 split stage");
        assert_eq!(edition.summit_finishes(), 1);
    }

    #[test]
    fn test_tour_de_france_1962() {
        let route = [
            "1,24 June,Nancy to Spa (Belgium),253.0 km,Road stage",
            "2a,25 June,Spa (Belgium) to Herentals (Belgium),147.0 km,Road stage",
            "2b,25 June,Herentals (Belgium),23.0 km,Team time trial",
            "3,26 June,Brussels (Belgium) to Amiens,210.0 km,Road stage",
            "4,27 June,Amiens to Le Havre,196.5 km,Road stage",
            "5,28 June,Pont l'Eveque to Saint-Malo,215.0 km,Road stage",
            "6,29 June,Dinard to Brest,235.5 km,Road stage",
            "7,30 June,Quimper to Saint-Nazaire,201.0 km,Road stage",
            "8a,1 July,Saint-Nazaire to Lucon,155.0 km,Road stage",
            "8b,1 July,Lucon to La Rochelle,43.0 km,Individual time trial",
            "9,2 July,La Rochelle to Bordeaux,214.0 km,Road stage",
            "10,3 July,Bordeaux to Bayonne,184.5 km,Road stage",
            "11,4 July,Bayonne to Pau,155.5 km,Road stage",
            "12,5 July,Pau to Saint-Gaudens,207.5 km,Road stage",
            "13,6 July,Luchon to Superbagnères,18.5 km,Individual time trial",
            "14,7 July,Luchon to Carcassonne,215.0 km,Road stage",
            "15,8 July,Carcassonne to Montpellier,196.5 km,Road stage",
            "16,9 July,Montpellier to Aix-en-Provence,185.0 km,Road stage",
            "17,10 July,Aix-en-Provence to Antibes,201.0 km,Road stage",
            "18,11 July,Antibes to Briançon,241.5 km,Road stage",
            "19,12 July,Briançon to Aix-les-Bains,204.5 km,Road stage",
            "20,13 July,Bourgoin to Lyon,68.0 km,Individual time trial",
            "21,14 July,Lyon to Nevers,232.0 km,Road stage",
            "22,15 July,Nevers to Paris,271.0 km,Road stage",
        ];
        let edition = tour_de_france_1962();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 2 split stages");
        assert_eq!(edition.summit_finishes(), 1);
    }

    #[test]
    fn test_tour_de_france_1963() {
        let route = [
            "1,23 June,Paris to Epernay,152.0 km,Road stage",
            "2a,24 June,Reims to Jambes (Belgium),186.0 km,Road stage",
            "2b,24 June,Jambes (Belgium),22.0 km,Team time trial",
            "3,25 June,Jambes (Belgium) to Roubaix,223.0 km,Road stage",
            "4,26 June,Roubaix to Rouen,236.0 km,Road stage",
            "5,27 June,Rouen to Rennes,285.0 km,Road stage",
            "6a,28 June,Rennes to Angers,118.0 km,Road stage",
            "6b,28 June,Angers,25.0 km,Individual time trial",
            "7,29 June,Angers to Limoges,236.0 km,Road stage",
            "8,30 June,Limoges to Bordeaux,232.0 km,Road stage",
            "9,1 July,Bordeaux to Pau,202.0 km,Road stage",
            "10,2 July,Pau to Bagnères-de-Bigorre,148.0 km,Road stage",
            "11,3 July,Bagnères-de-Bigorre to Luchon,131.0 km,Road stage",
            "12,4 July,Luchon to Toulouse,173.0 km,Road stage",
            "13,5 July,Toulouse to Aurillac,234.0 km,Road stage",
            ",6 July,Rest day,Aurillac",
            "14,7 July,Aurillac to Saint-Étienne,237.0 km,Road stage",
            "15,8 July,Saint-Étienne to Grenoble,174.0 km,Road stage",
            "16,9 July,Grenoble to Val d'Isère,202.0 km,Road stage",
            "17,10 July,Val d'Isère to Chamonix,228.0 km,Road stage",
            "18,11 July,Chamonix to Lons-le-Saunier,225.0 km,Road stage",
            "19,12 July,Arbois to Besançon,54.0 km,Individual time trial",
            "20,13 July,Besançon to Troyes,234.0 km,Road stage",
            "21,14 July,Troyes to Paris,185.0 km,Road stage",
        ];
        let edition = tour_de_france_1963();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages including 2 split stages");
        assert_eq!(edition.summit_finishes(), 1);
    }

    #[test]
    fn test_tour_de_france_1964() {
        let route = [
            "1,22 June,Rennes to Lisieux,215.0 km,Road stage",
            "2,23 June,Lisieux to Amiens,208.0 km,Road stage",
            "3a,24 June,Amiens to Forest (Belgium),197.0 km,Road stage",
            "3b,24 June,Forest (Belgium),21.0 km,Team time trial",
            "4,25 June,Forest (Belgium) to Metz,292.0 km,Road stage",
            "5,26 June,Luneville to Freiburg (West Germany),161.0 km,Road stage",
            "6,27 June,Freiburg (West Germany) to Besançon,200.0 km,Road stage",
            "7,28 June,Besançon to Thonon-les-Bains,195.0 km,Road stage",
            "8,29 June,Thonon-les-Bains to Briançon,249.0 km,Road stage",
            "9,30 June,Briançon to Monaco (Monaco),239.0 km,Road stage",
            "10a,1 July,Monaco (Monaco) to Hyeres,187.0 km,Road stage",
            "10b,1 July,Hyeres to Toulon,21.0 km,Individual time trial",
            "11,2 July,Toulon to Montpellier,250.0 km,Road stage",
            "12,3 July,Montpellier to Perpignan,174.0 km,Road stage",
            "13,4 July,Perpignan to Andorra la Vella (Andorra),170.0 km,Road stage",
            ",5 July,Rest day,Andorra la Vella (Andorra)",
            "14,6 July,Andorra la Vella (Andorra) to Toulouse,186.0 km,Road stage",
            "15,7 July,Toulouse to Luchon,203.0 km,Road stage",
            "16,8 July,Luchon to Pau,197.0 km,Road stage",
            "17,9 July,Peyrehorade to Bayonne,43.0 km,Individual time trial",
            "18,10 July,Bayonne to Bordeaux,187.0 km,Road stage",
            "19,11 July,Bordeaux to Brive,215.0 km,Road stage",
            "20,12 July,Brive to Puy de Dôme,217.0 km,Road stage",
            "21,13 July,Clermont-Ferrand to Orléans,311.0 km,Road stage",
            "22a,14 July,Orléans to Versailles,119.0 km,Road stage",
            "22b,14 July,Versailles to Paris,27.0 km,Individual time trial",
        ];
        let edition = tour_de_france_1964();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 3 split stages");
    }

    #[test]
    fn test_tour_de_france_1965() {
        let route = [
            "1a,22 June,Cologne (West Germany) to Liège (Belgium),149.0 km,Road stage",
            "1b,22 June,Liège (Belgium),22.5 km,Team time trial",
            "2,23 June,Liège (Belgium) to Roubaix,200.5 km,Road stage",
            "3,24 June,Roubaix to Rouen,240.0 km,Road stage",
            "4,25 June,Caen to Saint-Brieuc,227.0 km,Road stage",
            "5a,26 June,Saint-Brieuc to Châteaulin,147.0 km,Road stage",
            "5b,26 June,Châteaulin,26.7 km,Individual time trial",
            "6,27 June,Quimper to La Baule,210.5 km,Road stage",
            "7,28 June,La Baule to La Rochelle,219.0 km,Road stage",
            "8,29 June,La Rochelle to Bordeaux,197.5 km,Road stage",
            "9,30 June,Dax to Bagnères-de-Bigorre,226.5 km,Road stage",
            "10,1 July,Bagnères-de-Bigorre to Ax-les-Thermes,222.5 km,Road stage",
            "11,2 July,Ax-les-Thermes to Barcelona (Spain),240.5 km,Road stage",
            ",3 July,Rest day,Barcelona (Spain)",
            "12,4 July,Barcelona (Spain) to Perpignan,219.0 km,Road stage",
            "13,5 July,Perpignan to Montpellier,164.0 km,Road stage",
            "14,6 July,Montpellier to Mont Ventoux,173.0 km,Road stage",
            "15,7 July,Carpentras to Gap,167.5 km,Road stage",
            "16,8 July,Gap to Briançon,177.0 km,Road stage",
            "17,9 July,Briançon to Aix-les-Bains,193.5 km,Road stage",
            "18,10 July,Aix-les-Bains to Le Revard,26.9 km,Individual time trial",
            "19,11 July,Aix-les-Bains to Lyon,165.0 km,Road stage",
            "20,12 July,Lyon to Auxerre,198.5 km,Road stage",
            "21,13 July,Auxerre to Versailles,225.5 km,Road stage",
            "22,14 July,Versailles to Paris,37.8 km,Individual time trial",
        ];
        let edition = tour_de_france_1965();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1966() {
        let route = [
            "1,21 June,Nancy to Charleville,209.0 km,Road stage",
            "2,22 June,Charleville to Tournai (Belgium),198.0 km,Road stage",
            "3a,23 June,Tournai (Belgium),21.0 km,Team time trial",
            "3b,23 June,Tournai (Belgium) to Dunkerque,131.0 km,Road stage",
            "4,24 June,Dunkerque to Dieppe,205.0 km,Road stage",
            "5,25 June,Dieppe to Caen,178.0 km,Road stage",
            "6,26 June,Caen to Angers,217.0 km,Road stage",
            "7,27 June,Angers to Royan,252.0 km,Road stage",
            "8,28 June,Royan to Bordeaux,138.0 km,Road stage",
            "9,29 June,Bordeaux to Bayonne,201.0 km,Road stage",
            "10,30 June,Bayonne to Pau,234.0 km,Road stage",
            "11,1 July,Pau to Luchon,188.0 km,Road stage",
            ",2 July,Rest day,Luchon",
            "12,3 July,Luchon to Revel,219.0 km,Road stage",
            "13,4 July,Revel to Sete,191.0 km,Road stage",
            "14a,5 July,Montpellier to Vals-les-Bains,144.0 km,Road stage",
            "14b,5 July,Vals-les-Bains,20.0 km,Individual time trial",
            "15,6 July,Privas to Le Bourg-d'Oisans,203.0 km,Road stage",
            "16,7 July,Le Bourg-d'Oisans to Briançon,148.0 km,Road stage",
            "17,8 July,Briançon to Turin (Italy),160.0 km,Road stage",
            ",9 July,Rest day,Turin (Italy)",
            "18,10 July,Ivrea (Italy) to Chamonix,188.0 km,Road stage",
            "19,11 July,Chamonix to Saint-Étienne,265.0 km,Road stage",
            "20,12 July,Saint-Étienne to Montlucon,223.0 km,Road stage",
            "21,13 July,Montlucon to Orléans,232.0 km,Road stage",
            "22a,14 July,Orléans to Rambouillet,111.0 km,Road stage",
            "22b,14 July,Rambouillet to Paris,51.0 km,Individual time trial",
        ];
        let edition = tour_de_france_1966();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages including 3 split stages");
    }

    #[test]
    fn test_tour_de_france_1967() {
        let route = [
            "P,29 June,Angers,5.8 km,Individual time trial",
            "1,30 June,Angers to Saint-Malo,185.5 km,Road stage",
            "2,1 July,Saint-Malo to Caen,180.0 km,Road stage",
            "3,2 July,Caen to Amiens,248.0 km,Road stage",
            "4,3 July,Amiens to Roubaix,191.0 km,Road stage",
            "5a,4 July,Roubaix to Jambes (Belgium),172.0 km,Road stage",
            "5b,4 July,Jambes (Belgium),17.0 km,Team time trial",
            "6,5 July,Jambes (Belgium) to Metz,238.0 km,Road stage",
            "7,6 July,Metz to Strasbourg,205.5 km,Road stage",
            "8,7 July,Metz to Belfort,215.0 km,Road stage",
            ",8 July,Rest day,Belfort",
            "9,9 July,Belfort to Divonne-les-Bains,238.5 km,Road stage",
            "10,10 July,Divonne-les-Bains to Briançon,243.0 km,Road stage",
            "11,11 July,Briançon to Digne,197.0 km,Road stage",
            "12,12 July,Digne to Marseille,207.5 km,Road stage",
            "13,13 July,Marseille to Carpentras,211.5 km,Road stage",
            "14,14 July,Carpentras to Sete,201.5 km,Road stage",
            ",15 July,Rest day,Sete",
            "15,16 July,Sete to Toulouse,230.5 km,Road stage",
            "16,17 July,Toulouse to Luchon,188.0 km,Road stage",
            "17,18 July,Luchon to Pau,250.0 km,Road stage",
            "18,19 July,Pau to Bordeaux,206.5 km,Road stage",
            "19,20 July,Bordeaux to Limoges,217.0 km,Road stage",
            "20,21 July,Limoges to Puy de Dôme,222.0 km,Road stage",
            "21,22 July,Clermont-Ferrand to Fontainebleau,359.0 km,Road stage",
            "22a,23 July,Fontainebleau to Versailles,104.0 km,Road stage",
            "22b,23 July,Versailles to Paris,46.6 km,Individual time trial",
        ];
        let edition = tour_de_france_1967();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1968() {
        let route = [
            "P,27 June,Vittel,6.1 km,Individual time trial",
            "1,28 June,Vittel to Esch-sur-Alzette (Luxembourg),189.0 km,Road stage",
            "2,29 June,Arlon (Belgium) to Forest (Belgium),210.5 km,Road stage",
            "3a,30 June,Forest (Belgium),22.0 km,Team time trial",
            "3b,30 June,Forest (Belgium) to Roubaix,112.0 km,Road stage",
            "4,1 July,Roubaix to Rouen,238.0 km,Road stage",
            "5a,2 July,Rouen to Bagnoles-de-l'Orne,165.0 km,Road stage",
            "5b,2 July,Bagnoles-de-l'Orne to Dinard,154.5 km,Road stage",
            "6,3 July,Dinard to Lorient,188.0 km,Road stage",
            "7,4 July,Lorient to Nantes,190.0 km,Road stage",
            "8,5 July,Nantes to Royan,233.0 km,Road stage",
            ",6 July,Rest day,Royan",
            "9,7 July,Royan to Bordeaux,137.5 km,Road stage",
            "10,8 July,Bordeaux to Bayonne,202.5 km,Road stage",
            "11,9 July,Bayonne to Pau,183.5 km,Road stage",
            "12,10 July,Pau to Saint-Gaudens,226.5 km,Road stage",
            "13,11 July,Saint-Gaudens to La Seu d'Urgell (Spain),208.5 km,Road stage",
            "14,12 July,La Seu d'Urgell (Spain) to Perpignan,231.5 km,Road stage",
            ",13 July,Rest day,Font-Romeu-Odeillo-Via",
            "15,14 July,Font-Romeu-Odeillo-Via to Albi,250.5 km,Road stage",
            "16,15 July,Albi to Aurillac,199.0 km,Road stage",
            "17,16 July,Aurillac to Saint-Étienne,236.5 km,Road stage",
            "18,17 July,Saint-Étienne to Grenoble,235.0 km,Road stage",
            "19,18 July,Grenoble to Sallanches,200.0 km,Road stage",
            "20,19 July,Sallanches to Besançon,242.5 km,Road stage",
            "21,20 July,Besançon to Auxerre,242.0 km,Road stage",
            "22a,21 July,Auxerre to Melun,136.0 km,Road stage",
            "22b,21 July,Melun to Paris,55.2 km,Individual time trial",
        ];
        let edition = tour_de_france_1968();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 3 split stages");
    }

    #[test]
    fn test_tour_de_france_1969() {
        let route = [
            "P,28 June,Roubaix,10.0 km,Individual time trial",
            "1a,29 June,Roubaix to Sint-Pieters-Woluwe (Belgium),147.0 km,Road stage",
            "1b,29 June,Sint-Pieters-Woluwe (Belgium),16.0 km,Team time trial",
            "2,30 June,Sint-Pieters-Woluwe (Belgium) to Maastricht (Netherlands),182.0 km,Road stage",
            "3,1 July,Maastricht (Netherlands) to Charleville-Mezieres,213.0 km,Road stage",
            "4,2 July,Charleville-Mezieres to Nancy,214.0 km,Road stage",
            "5,3 July,Nancy to Mulhouse,194.0 km,Road stage",
            "6,4 July,Mulhouse to Ballon d'Alsace,133.0 km,Road stage",
            "7,5 July,Belfort to Divonne-les-Bains,241.0 km,Road stage",
            "8a,6 July,Divonne-les-Bains,9.0 km,Individual time trial",
            "8b,6 July,Divonne-les-Bains to Thonon-les-Bains,137.0 km,Road stage",
            "9,7 July,Thonon-les-Bains to Chamonix,111.0 km,Road stage",
            "10,8 July,Chamonix to Briançon,221.0 km,Road stage",
            "11,9 July,Briançon to Digne,198.0 km,Road stage",
            "12,10 July,Digne to Aubagne,161.0 km,Road stage",
            "13,11 July,Aubagne to La Grande-Motte,196.0 km,Road stage",
            "14,12 July,La Grande-Motte to Revel,234.0 km,Road stage",
            ",13 July,Rest day,Revel",
            "15,14 July,Castelnaudary to Luchon,199.0 km,Road stage",
            "16,15 July,Luchon to Mourenx,214.0 km,Road stage",
            "17,16 July,Mourenx to Bordeaux,201.0 km,Road stage",
            "18,17 July,Bordeaux to Brive,193.0 km,Road stage",
            "19,18 July,Brive to Puy de Dôme,198.0 km,Road stage",
            "20,19 July,Clermont-Ferrand to Montargis,329.0 km,Road stage",
            "21a,20 July,Montargis to Creteil,111.0 km,Road stage",
            "21b,20 July,Creteil to Paris,37.0 km,Individual time trial",
        ];
        let edition = tour_de_france_1969();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue including 3 split stages");
    }

    #[test]
    fn test_tour_de_france_1970() {
        let route = [
            "P,26 June,Limoges,7.4 km,Individual time trial",
            "1,27 June,Limoges to La Rochelle,224.0 km,Road stage",
            "2,28 June,La Rochelle to Angers,200.0 km,Road stage",
            "3a,29 June,Angers,10.7 km,Team time trial",
            "3b,29 June,Angers to Rennes,140.0 km,Road stage",
            "4,30 June,Rennes to Lisieux,229.0 km,Road stage",
            "5a,1 July,Lisieux to Rouen,94.5 km,Road stage",
            "5b,1 July,Rouen to Amiens,223.0 km,Road stage",
            "6,2 July,Amiens to Valenciennes,135.5 km,Road stage",
            "7a,3 July,Valenciennes to Forest (Belgium),120.0 km,Road stage",
            "7b,3 July,Forest (Belgium),7.2 km,Individual time trial",
            "8,4 July,Ciney to Felsberg (West Germany),232.5 km,Road stage",
            "9,5 July,Saarlouis (West Germany) to Mulhouse,269.5 km,Road stage",
            "10,6 July,Belfort to Divonne-les-Bains,241.0 km,Road stage",
            "11a,7 July,Divonne-les-Bains,8.8 km,Individual time trial",
            "11b,7 July,Divonne-les-Bains to Thonon-les-Bains,139.5 km,Road stage",
            "12,8 July,Divonne-les-Bains to Grenoble,194.0 km,Road stage",
            "13,9 July,Grenoble to Gap,194.5 km,Road stage",
            "14,10 July,Gap to Mont Ventoux,170.0 km,Road stage",
            "15,11 July,Carpentras to Montpellier,140.5 km,Road stage",
            "16,12 July,Montpellier to Toulouse,160.0 km,Road stage",
            "17,13 July,Toulouse to Saint-Gaudens,190.0 km,Road stage",
            "18,14 July,Saint-Gaudens to La Mongie,135.5 km,Road stage",
            "19,15 July,Bagnères-de-Bigorre to Mourenx,185.5 km,Road stage",
            "20a,16 July,Mourenx to Bordeaux,223.5 km,Road stage",
            "20b,16 July,Bordeaux,8.2 km,Individual time trial",
            "21,17 July,Ruffex to Tours,191.5 km,Road stage",
            "22,18 July,Tours to Versailles,238.5 km,Road stage",
            "23,19 July,Versailles to Paris,54.0 km,Individual time trial",
        ];
        let edition = tour_de_france_1970();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "23 stages + Prologue including 5 split stages");
    }

    #[test]
    fn test_tour_de_france_1971() {
        let route = [
            "P,26 June,Mulhouse,11.0 km,Individual time trial",
            "1a,27 June,Mulhouse to Basel (Switzerland),59.5 km,Road stage",
            "1b,27 June,Basel (Switzerland) to Freiburg (West Germany),90.0 km,Road stage",
            "1c,27 June,Freiburg (West Germany) to Mulhouse,74.5 km,Road stage",
            "2,28 June,Mulhouse to Strasbourg,144.0 km,Road stage",
            "3,29 June,Strasbourg to Nancy,165.6 km,Road stage",
            "4,30 June,Nancy to Marche-en-Famenne (Belgium),242.0 km,Road stage",
            "5,1 July,Dinant (Belgium) to Roubaix,208.5 km,Road stage",
            "6a,2 July,Roubaix to Amiens,127.5 km,Road stage",
            "6b,2 July,Amiens to Le Touquet,133.5 km,Road stage",
            ",3 July,Rest day,Le Touquet",
            "7,4 July,Rungis to Nevers,257.5 km,Road stage",
            "8,5 July,Nevers to Puy de Dôme,221.0 km,Road stage",
            "9,6 July,Clermont-Ferrand to Saint-Étienne,153.0 km,Road stage",
            "10,7 July,Saint-Étienne to Grenoble,188.5 km,Road stage",
            "11,8 July,Grenoble to Orcières-Merlette,134.0 km,Road stage",
            ",9 July,Rest day,Orcières-Merlette",
            "12,10 July,Orcières-Merlette to Marseille,251.0 km,Road stage",
            "13,11 July,Albi,16.3 km,Road stage",
            "14,12 July,Revel to Luchon,214.5 km,Road stage",
            "15,13 July,Luchon to Superbagnères,19.6 km,Road stage",
            "16a,14 July,Luchon to Gourette,145.0 km,Road stage",
            "16b,14 July,Gourette to Pau,57.5 km,Road stage",
            "17,15 July,Mont-de-Marsan to Bordeaux,188.0 km,Road stage",
            "18,16 July,Bordeaux to Poitiers,244.0 km,Road stage",
            "19,17 July,Blois to Versailles,244.0 km,Road stage",
            "20,18 July,Versailles to Paris,53.8 km,Individual time trial",
        ];
        let edition = tour_de_france_1971();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue including 3 split stages");
    }

    #[test]
    fn test_tour_de_france_1972() {
        let route = [
            "P,1 July,Angers,7.2 km,Individual time trial",
            "1,2 July,Angers to Saint-Brieuc,235.5 km,Road stage",
            "2,3 July,Saint-Brieuc to La Baule,206.5 km,Road stage",
            "3a,4 July,Pornichet to Saint-Jean-de-Monts,161.0 km,Road stage",
            "3b,4 July,Merlin-Plage,16.2 km,Individual time trial",
            "4,5 July,Merlin-Plage to Royan,236.0 km,Road stage",
            "5a,6 July,Royan to Bordeaux,133.5 km,Road stage",
            "5b,6 July,Bordeaux,12.7 km,Individual time trial",
            "6,7 July,Bordeaux to Bayonne,205.0 km,Road stage",
            ",8 July,Rest day,Bayonne",
            "7,9 July,Bayonne to Pau,220.5 km,Road stage",
            "8,10 July,Pau to Luchon,163.5 km,Road stage",
            "9,11 July,Luchon to Colomiers,179.0 km,Road stage",
            "10,12 July,Castres to La Grande-Motte,210.0 km,Road stage",
            "11,13 July,Carnon-Plage to Mont Ventoux,207.0 km,Road stage",
            "12,14 July,Carpentras to Orcières-Merlette,192.0 km,Road stage",
            ",15 July,Rest day,Orcières-Merlette",
            "13,16 July,Orcières-Merlette to Briançon,201.0 km,Road stage",
            "14a,17 July,Briançon to Valloire,51.0 km,Road stage",
            "14b,17 July,Valloire to Aix-les-Bains,151.0 km,Road stage",
            "15,18 July,Aix-les-Bains to Le Revard,28.0 km,Road stage",
            "16,19 July,Aix-les-Bains to Pontarlier,198.5 km,Road stage",
            "17,20 July,Pontarlier to Ballon d'Alsace,213.0 km,Road stage",
            "18,21 July,Vesoul to Auxerre,257.5 km,Road stage",
            "19,22 July,Auxerre to Versailles,230.0 km,Road stage",
            "20a,23 July,Versailles,42.0 km,Individual time trial",
            "20b,23 July,Versailles to Paris,89.0 km,Road stage",
        ];
        let edition = tour_de_france_1972();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue including 4 split stages");
    }

    #[test]
    fn test_tour_de_france_1973() {
        let route = [
            "P,30 June,Scheveningen (Netherlands),7.1 km,Individual time trial",
            "1a,1 July,Scheveningen (Netherlands) to Rotterdam (Netherlands),84.0 km,Road stage",
            "1b,1 July,Rotterdam (Netherlands) to Sint-Niklaas (Belgium),137.5 km,Road stage",
            "2a,2 July,Sint-Niklaas (Belgium),12.4 km,Team time trial",
            "2b,2 July,Sint-Niklaas (Belgium) to Roubaix,138.0 km,Road stage",
            "3,3 July,Roubaix to Reims,226.0 km,Road stage",
            "4,4 July,Reims to Nancy,214.0 km,Road stage",
            "5,5 July,Nancy to Mulhouse,188.0 km,Road stage",
            "6,6 July,Mulhouse to Divonne-les-Bains,244.5 km,Road stage",
            ",7 July,Rest day,Divonne-les-Bains",
            "7a,8 July,Divonne-les-Bains to Gaillard,86.5 km,Road stage",
            "7b,8 July,Gaillard to Méribel-les-Allues,150.5 km,Road stage",
            "8,9 July,Moûtiers to Les Orres,237.5 km,Road stage",
            "9,10 July,Embrun to Nice,234.5 km,Road stage",
            "10,11 July,Nice to Aubagne,222.5 km,Road stage",
            "11,12 July,Montpellier to Argelès-sur-Mer,238.0 km,Road stage",
            "12a,13 July,Perpignan to Thuir,28.3 km,Individual time trial",
            "12b,13 July,Thuir to Pyrénées 2000,76.0 km,Road stage",
            ",14 July,Rest day,Pyrénées 2000",
            "13,15 July,Bourg-Madame to Luchon,235.0 km,Road stage",
            "14,16 July,Luchon to Pau,227.5 km,Road stage",
            "15,17 July,Pau to Fleurance,137.0 km,Road stage",
            "16a,18 July,Fleurance to Bordeaux,210.0 km,Road stage",
            "16b,18 July,Bordeaux,12.4 km,Individual time trial",
            "17,19 July,Sainte-Foy-la-Grande to Brive-la-Gaillarde,248.0 km,Road stage",
            "18,20 July,Brive-la-Gaillarde to Puy de Dôme,216.5 km,Road stage",
            "19,21 July,Bourges to Versailles,233.5 km,Road stage",
            "20a,22 July,Versailles,16.0 km,Individual time trial",
            "20b,22 July,Versailles to Paris,89.0 km,Road stage",
        ];
        let edition = tour_de_france_1973();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue including 6 split stages");
    }

    #[test]
    fn test_tour_de_france_1974() {
        let route = [
            "P,27 June,Brest,7.0 km,Individual time trial",
            "1,28 June,Brest to Saint-Pol-de-Léon,144.0 km,Road stage",
            "2,29 June,Plymouth (United Kingdom),164.0 km,Road stage",
            "3,30 June,Morlaix to Saint-Malo,190.0 km,Road stage",
            "4,1 July,Saint-Malo to Caen,184.0 km,Road stage",
            "5,2 July,Caen to Dieppe,165.0 km,Road stage",
            "6a,3 July,Dieppe to Harelbeke (Belgium),239.0 km,Road stage",
            "6b,3 July,Harelbeke (Belgium),9.0 km,Team time trial",
            "7,4 July,Mons (Belgium) to Châlons-sur-Marne,221.0 km,Road stage",
            "8a,5 July,Châlons-sur-Marne to Chaumont,136.0 km,Road stage",
            "8b,5 July,Chaumont to Besançon,152.0 km,Road stage",
            "9,6 July,Besançon to Gaillard,241.0 km,Road stage",
            "10,7 July,Gaillard to Aix-les-Bains,131.0 km,Road stage",
            "11,8 July,Aix-les-Bains to Serre Chevalier,199.0 km,Road stage",
            ",9 July,Rest day,Aix-les-Bains",
            "12,10 July,Savines-le-Lac to Orange,231.0 km,Road stage",
            "13,11 July,Avignon to Montpellier,126.0 km,Road stage",
            "14,12 July,Lodève to Colomiers,249.0 km,Road stage",
            ",13 July,Rest day,Colomiers",
            "15,14 July,Colomiers to La Seu d'Urgell (Spain),225.0 km,Road stage",
            "16,15 July,La Seu d'Urgell (Spain) to Saint-Lary-Soulan Pla d'Adet,209.0 km,Road stage",
            "17,16 July,Saint-Lary-Soulan to La Mongie,119.0 km,Road stage",
            "18,17 July,Bagnères-de-Bigorre to Pau,141.0 km,Road stage",
            "19a,18 July,Pau to Bordeaux,196.0 km,Road stage",
            "19b,18 July,Bordeaux,12.0 km,Individual time trial",
            "20,19 July,Saint-Gilles-Croix-de-Vie to Nantes,120.0 km,Road stage",
            "21a,20 July,Vouvray to Orléans,113.0 km,Road stage",
            "21b,20 July,Orléans,37.0 km,Individual time trial",
            "22,21 July,Orléans to Paris,146.0 km,Road stage",
        ];
        let edition = tour_de_france_1974();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 4 split stages");
    }

    #[test]
    fn test_tour_de_france_1975() {
        let route = [
            "P,26 June,Charleroi (Belgium),6.0 km,Individual time trial",
            "1a,27 June,Charleroi (Belgium) to Molenbeek (Belgium),94.0 km,Road stage",
            "1b,27 June,Molenbeek (Belgium) to Roubaix,109.0 km,Road stage",
            "2,28 June,Roubaix to Amiens,121.0 km,Road stage",
            "3,29 June,Amiens to Versailles,170.0 km,Road stage",
            "4,30 June,Versailles to Le Mans,223.0 km,Road stage",
            "5,1 July,Sablé-sur-Sarthe to Merlin-Plage,222.0 km,Road stage",
            "6,2 July,Merlin-Plage,16.0 km,Individual time trial",
            "7,3 July,Saint-Gilles-Croix-de-Vie to Angoulême,236.0 km,Road stage",
            "8,4 July,Angoulême to Bordeaux,134.0 km,Road stage",
            "9a,5 July,Langon to Fleurance,131.0 km,Road stage",
            "9b,5 July,Fleurance to Auch,37.0 km,Individual time trial",
            ",6 July,Rest day,Auch",
            "10,7 July,Auch to Pau,206.0 km,Road stage",
            "11,8 July,Pau to Saint-Lary-Soulan Pla d'Adet,160.0 km,Road stage",
            "12,9 July,Tarbes to Albi,242.0 km,Road stage",
            "13,10 July,Albi to Le Lioran,260.0 km,Road stage",
            "14,11 July,Aurillac to Puy de Dôme,174.0 km,Road stage",
            ",12 July,Rest day,Nice",
            "15,13 July,Nice to Pra-Loup,217.0 km,Road stage",
            "16,14 July,Barcelonnette to Serre Chevalier,107.0 km,Road stage",
            "17,15 July,Valloire to Morzine-Avoriaz,225.0 km,Road stage",
            "18,16 July,Morzine to Châtel,40.0 km,Individual time trial",
            "19,17 July,Thonon-les-Bains to Chalon-sur-Saône,229.0 km,Road stage",
            "20,18 July,Pouilly-en-Auxois to Melun,256.0 km,Road stage",
            "21,19 July,Melun to Senlis,220.0 km,Road stage",
            "22,20 July,Paris,164.0 km,Road stage",
        ];
        let edition = tour_de_france_1975();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1976() {
        let route = [
            "P,24 June,Saint-Jean-de-Monts,8.0 km,Individual time trial",
            "1,25 June,Saint-Jean-de-Monts to Angers,173.0 km,Road stage",
            "2,26 June,Angers to Caen,237.0 km,Road stage",
            "3,27 June,Le Touquet-Paris-Plage,37.0 km,Individual time trial",
            "4,28 June,Le Touquet-Paris-Plage to Bornem (Belgium),258.0 km,Road stage",
            "5a,29 June,Leuven (Belgium),4.0 km,Team time trial",
            "5b,29 June,Leuven (Belgium) to Verviers (Belgium),144.0 km,Road stage",
            "6,30 June,Bastogne (Belgium) to Nancy,209.0 km,Road stage",
            "7,1 July,Nancy to Mulhouse,206.0 km,Road stage",
            "8,2 July,Valentigney to Divonne-les-Bains,220.0 km,Road stage",
            ",3 July,Rest day,Divonne-les-Bains",
            "9,4 July,Divonne-les-Bains to Alpe d'Huez,258.0 km,Road stage",
            "10,5 July,Le Bourg-d'Oisans to Montgenèvre,166.0 km,Road stage",
            "11,6 July,Montgenèvre to Manosque,224.0 km,Road stage",
            ",7 July,Rest day,Le Barcarès",
            "12,8 July,Le Barcarès to Pyrénées 2000,205.0 km,Road stage",
            "13,9 July,Pyrénées 2000 to Saint-Gaudens,188.0 km,Road stage",
            "14,10 July,Saint-Gaudens to Saint-Lary-Soulan,139.0 km,Road stage",
            "15,11 July,Saint-Lary-Soulan to Pau,195.0 km,Road stage",
            "16,12 July,Pau to Fleurance,152.0 km,Road stage",
            "17,13 July,Fleurance to Auch,39.0 km,Individual time trial",
            "18a,14 July,Auch to Langon,86.0 km,Road stage",
            "18b,14 July,Langon to Lacanau,123.0 km,Road stage",
            "18c,14 July,Lacanau to Bordeaux,70.0 km,Road stage",
            "19,15 July,Sainte-Foy-la-Grande to Tulle,220.0 km,Road stage",
            "20,16 July,Tulle to Puy de Dôme,220.0 km,Road stage",
            "21,17 July,Montargis to Versailles,145.0 km,Road stage",
            "22a,18 July,Paris,6.0 km,Individual time trial",
            "22b,18 July,Paris,91.0 km,Road stage",
        ];
        let edition = tour_de_france_1976();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 3 split stages");
    }

    #[test]
    fn test_tour_de_france_1977() {
        let route = [
            "P,30 June,Fleurance,5.0 km,Individual time trial",
            "1,1 July,Fleurance to Auch,237.0 km,Road stage",
            "2,2 July,Auch to Pau,253.0 km,Road stage",
            "3,3 July,Oloron-Sainte-Marie to Vitoria-Gasteiz (Spain),248.0 km,Road stage",
            "4,4 July,Vitoria-Gasteiz (Spain) to Seignosse le Penon,256.0 km,Road stage",
            "5a,5 July,Morcenx to Bordeaux,139.0 km,Road stage",
            "5b,5 July,Bordeaux,30.0 km,Individual time trial",
            ",6 July,Rest day,Bordeaux",
            "6,7 July,Bordeaux to Limoges,225.0 km,Road stage",
            "7a,8 July,Jaunay-Clan to Angers,140.0 km,Road stage",
            "7b,8 July,Angers,4.0 km,Team time trial",
            "8,9 July,Angers to Lorient,247.0 km,Road stage",
            "9,10 July,Lorient to Rennes,187.0 km,Road stage",
            "10,11 July,Bagnoles-de-l'Orne to Rouen,174.0 km,Road stage",
            "11,12 July,Rouen to Roubaix,242.0 km,Road stage",
            "12,13 July,Roubaix to Charleroi (Belgium),193.0 km,Road stage",
            "13a,14 July,Freiburg (West Germany),46.0 km,Road stage",
            "13b,14 July,Altkirch to Besançon,160.0 km,Road stage",
            ",15 July,Rest day,Freiburg (West Germany)",
            "14,16 July,Besançon to Thonon-les-Bains,230.0 km,Road stage",
            "15a,17 July,Thonon-les-Bains to Morzine,105.0 km,Road stage",
            "15b,17 July,Morzine to Avoriaz,14.0 km,Individual time trial",
            "16,18 July,Morzine to Chamonix,121.0 km,Road stage",
            "17,19 July,Chamonix to Alpe d'Huez,185.0 km,Road stage",
            "18,20 July,Rossignol Voiron to Saint-Étienne,199.0 km,Road stage",
            "19,21 July,Saint-Trivier-sur-Moignans to Dijon,172.0 km,Road stage",
            "20,22 July,Dijon,50.0 km,Individual time trial",
            "21,23 July,Montereau-Fault-Yonne to Versailles,142.0 km,Road stage",
            "22a,24 July,Paris,6.0 km,Individual time trial",
            "22b,24 July,Paris,91.0 km,Road stage",
        ];
        let edition = tour_de_france_1977();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 5 split stages");
    }

    #[test]
    fn test_tour_de_france_1978() {
        let route = [
            "P,29 June,Leiden (Netherlands),5.0 km,Individual time trial",
            "1a,30 June,Leiden (Netherlands) to Sint Willebrord (Netherlands),135.0 km,Road stage",
            "1b,30 June,Sint Willebrord (Netherlands) to Brussels (Belgium),100.0 km,Road stage",
            "2,1 July,Brussels (Belgium) to Saint-Amand-les-Eaux,199.0 km,Road stage",
            "3,2 July,Saint-Amand-les-Eaux to Saint-Germain-en-Laye,244.0 km,Road stage",
            "4,3 July,Evreux to Caen,153.0 km,Team time trial",
            "5,4 July,Caen to Maze-Montgeoffroy,244.0 km,Road stage",
            "6,5 July,Maze-Montgeoffroy to Poitiers,162.0 km,Road stage",
            "7,6 July,Poitiers to Bordeaux,242.0 km,Road stage",
            "8,7 July,Saint-Emilion to Sainte-Foy-la-Grande,59.0 km,Individual time trial",
            "9,8 July,Bordeaux to Biarritz,233.0 km,Road stage",
            ",9 July,Rest day,Biarritz",
            "10,10 July,Biarritz to Pau,192.0 km,Road stage",
            "11,11 July,Pau to Saint-Lary-Soulan Pla d'Adet,161.0 km,Road stage",
            "12a,12 July,Tarbes to Valence d'Agen,158.0 km,Road stage",
            "12b,12 July,Valence d'Agen to Toulouse,96.0 km,Road stage",
            "13,13 July,Figeac to Super Besse,221.0 km,Road stage",
            "14,14 July,Besse-en-Chandesse to Puy de Dôme,52.0 km,Individual time trial",
            "15,15 July,Saint-Dier-d'Auvergne to Saint-Étienne,196.0 km,Road stage",
            "16,16 July,Saint-Étienne to Alpe d'Huez,241.0 km,Road stage",
            ",17 July,Rest day,Alpe d'Huez",
            "17,18 July,Grenoble to Morzine,225.0 km,Road stage",
            "18,19 July,Morzine to Lausanne (Switzerland),137.0 km,Road stage",
            "19,20 July,Lausanne (Switzerland) to Belfort,182.0 km,Road stage",
            "20,21 July,Metz to Nancy,72.0 km,Individual time trial",
            "21,22 July,Epernay to Senlis,207.0 km,Road stage",
            "22,23 July,Saint-Germain-en-Laye to Paris,162.0 km,Road stage",
        ];
        let edition = tour_de_france_1978();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1979() {
        let route = [
            "P,27 June,Fleurance,5.0 km,Individual time trial",
            "1,28 June,Fleurance to Luchon,225.0 km,Road stage",
            "2,29 June,Luchon to Superbagnères,24.0 km,Individual time trial",
            "3,30 June,Luchon to Pau,180.0 km,Road stage",
            "4,1 July,Captieux to Bordeaux,87.0 km,Team time trial",
            "5,2 July,Neuville-de-Poitou to Angers,145.0 km,Road stage",
            "6,3 July,Angers to Saint-Brieuc,239.0 km,Road stage",
            "7,4 July,Saint-Hilaire-du-Harcouët to Deauville,158.0 km,Road stage",
            "8,5 July,Deauville to Le Havre,90.0 km,Team time trial",
            "9,6 July,Amiens to Roubaix,201.0 km,Road stage",
            "10,7 July,Roubaix to Brussels (Belgium),124.0 km,Road stage",
            "11,8 July,Brussels (Belgium),33.0 km,Individual time trial",
            "12,9 July,Rochefort (Belgium) to Metz,193.0 km,Road stage",
            "13,10 July,Metz to Ballon d'Alsace,202.0 km,Road stage",
            "14,11 July,Belfort to Evian-les-Bains,248.0 km,Road stage",
            "15,12 July,Evian-les-Bains to Morzine-Avoriaz,54.0 km,Individual time trial",
            "16,13 July,Morzine-Avoriaz to Les Menuires,201.0 km,Road stage",
            ",14 July,Rest day,Les Menuires",
            "17,15 July,Les Menuires to Alpe d'Huez,167.0 km,Road stage",
            "18,16 July,Alpe d'Huez,119.0 km,Road stage",
            "19,17 July,Alpe d'Huez to Saint-Priest,162.0 km,Road stage",
            "20,18 July,Saint-Priest to Dijon,240.0 km,Road stage",
            "21,19 July,Dijon,49.0 km,Individual time trial",
            "22,20 July,Dijon to Auxerre,189.0 km,Road stage",
            "23,21 July,Auxerre to Nogent-sur-Marne,205.0 km,Road stage",
            "24,22 July,Le Perreux-sur-Marne to Paris,180.0 km,Road stage",
        ];
        let edition = tour_de_france_1979();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.summit_finishes(), 4);
        assert_eq!(edition.stage_summary(), "24 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1980() {
        let route = [
            "P,26 June,Frankfurt (West Germany),8.0 km,Individual time trial",
            "1a,27 June,Frankfurt (West Germany) to Wiesbaden (West Germany),133.0 km,Road stage",
            "1b,27 June,Wiesbaden (West Germany) to Frankfurt (West Germany),46.0 km,Team time trial",
            "2,28 June,Frankfurt (West Germany) to Metz,276.0 km,Road stage",
            "3,29 June,Metz to Liège (Belgium),282.0 km,Road stage",
            "4,30 June,Spa (Belgium),35.0 km,Individual time trial",
            "5,1 July,Liège (Belgium) to Lille,249.0 km,Road stage",
            "6,2 July,Lille to Compiegne,216.0 km,Road stage",
            "7a,3 July,Compiegne to Beauvais,65.0 km,Team time trial",
            "7b,3 July,Beauvais to Rouen,92.0 km,Road stage",
            "8,4 July,Flers to Saint-Malo,164.0 km,Road stage",
            ",5 July,Rest day,Saint-Malo",
            "9,6 July,Saint-Malo to Nantes,205.0 km,Road stage",
            "10,7 July,Rochefort to Bordeaux,163.0 km,Road stage",
            "11,8 July,Damazan to Laplume,52.0 km,Individual time trial",
            "12,9 July,Agen to Pau,194.0 km,Road stage",
            "13,10 July,Pau to Bagnères-de-Luchon,200.0 km,Road stage",
            "14,11 July,Lezignan-Corbieres to Montpellier,189.0 km,Road stage",
            "15,12 July,Montpellier to Martigues,160.0 km,Road stage",
            "16,13 July,Trets to Pra-Loup,209.0 km,Road stage",
            "17,14 July,Serre Chevalier to Morzine,242.0 km,Road stage",
            ",15 July,Rest day,Morzine",
            "18,16 July,Morzine to Prapoutel,199.0 km,Road stage",
            "19,17 July,Voreppe to Saint-Étienne,140.0 km,Road stage",
            "20,18 July,Saint-Étienne,34.0 km,Individual time trial",
            "21,19 July,Auxerre to Fontenay-sous-Bois,208.0 km,Road stage",
            "22,20 July,Fontenay-sous-Bois to Paris,186.0 km,Road stage",
        ];
        let edition = tour_de_france_1980();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1981() {
        let route = [
            "P,25 June,Nice,6.0 km,Individual time trial",
            "1a,26 June,Nice,97.0 km,Road stage",
            "1b,26 June,Nice,40.0 km,Team time trial",
            "2,27 June,Nice to Martigues,254.0 km,Road stage",
            "3,28 June,Martigues to Narbonne,232.0 km,Road stage",
            "4,29 June,Narbonne to Carcassonne,77.0 km,Team time trial",
            "5,30 June,Saint-Gaudens to Pla d'Adet,117.0 km,Road stage",
            "6,1 July,Nay to Pau,27.0 km,Individual time trial",
            "7,2 July,Pau to Bordeaux,227.0 km,Road stage",
            "8,3 July,Rochefort to Nantes,182.0 km,Road stage",
            ",4 July,Rest day,Nantes",
            "9,5 July,Nantes to Le Mans,197.0 km,Road stage",
            "10,6 July,Le Mans to Aulnay-sous-Bois,264.0 km,Road stage",
            "11,7 July,Compiegne to Roubaix,246.0 km,Road stage",
            "12a,8 July,Roubaix to Brussels (Belgium),107.0 km,Road stage",
            "12b,8 July,Brussels (Belgium) to Circuit Zolder (Belgium),138.0 km,Road stage",
            "13,9 July,Beringen (Belgium) to Hasselt (Belgium),157.0 km,Road stage",
            "14,10 July,Mulhouse,38.0 km,Individual time trial",
            "15,11 July,Besançon to Thonon-les-Bains,231.0 km,Road stage",
            "16,12 July,Thonon-les-Bains to Morzine,200.0 km,Road stage",
            ",13 July,Rest day,Morzine",
            "17,14 July,Morzine to Alpe d'Huez,230.0 km,Road stage",
            "18,15 July,Le Bourg-d'Oisans to Le Pleynet,134.0 km,Road stage",
            "19,16 July,Veurey to Saint-Priest,118.0 km,Road stage",
            "20,17 July,Saint-Priest,46.0 km,Individual time trial",
            "21,18 July,Auxerre to Fontenay-sous-Bois,207.0 km,Road stage",
            "22,19 July,Fontenay-sous-Bois to Paris,187.0 km,Road stage",
        ];
        let edition = tour_de_france_1981();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 2 split stages");
    }

    #[test]
    fn test_tour_de_france_1982() {
        let route = [
            "P,2 July,Basel (Switzerland),7.0 km,Individual time trial",
            "1,3 July,Basel (Switzerland) to Mohin (Switzerland),207.0 km,Road stage",
            "2,4 July,Basel (Switzerland) to Nancy,250.0 km,Road stage",
            "3,5 July,Nancy to Longwy,134.0 km,Road stage",
            "4,6 July,Beauraing (Belgium) to Mouscron (Belgium),219.0 km,Road stage",
            "5,7 July,Orchies to Fontaine-au-Pire,73.0 km,Team time trial",
            "6,8 July,Lille,233.0 km,Road stage",
            ",9 July,Rest day,Lille",
            "7,10 July,Cancale to Concarneau,235.0 km,Road stage",
            "8,11 July,Concarneau to Châteaulin,201.0 km,Road stage",
            "9a,12 July,Lorient to Plumelec,69.0 km,Team time trial",
            "9b,12 July,Plumelec to Nantes,138.0 km,Road stage",
            "10,13 July,Saintes to Bordeaux,147.0 km,Road stage",
            "11,14 July,Valence d'Agen,57.0 km,Individual time trial",
            "12,15 July,Fleurance to Pau,249.0 km,Road stage",
            "13,16 July,Pau to Saint-Lary-Soulan Pla d'Adet,122.0 km,Road stage",
            ",17 July,Rest day,Martigues",
            "14,18 July,Martigues,33.0 km,Individual time trial",
            "15,19 July,Manosque to Orcières-Merlette,208.0 km,Road stage",
            "16,20 July,Orcières-Merlette to Alpe d'Huez,123.0 km,Road stage",
            "17,21 July,Le Bourg-d'Oisans to Morzine,251.0 km,Road stage",
            "18,22 July,Morzine to Saint-Priest,233.0 km,Road stage",
            "19,23 July,Saint-Priest,48.0 km,Individual time trial",
            "20,24 July,Sens to Aulnay-sous-Bois,161.0 km,Road stage",
            "21,25 July,Fontenay-sous-Bois to Paris,187.0 km,Road stage",
        ];
        let edition = tour_de_france_1982();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_de_france_1983() {
        let route = [
            "P,1 July,Fontenay-sous-Bois,6.0 km,Individual time trial",
            "1,2 July,Nogent-sur-Marne to Creteil,163.0 km,Road stage",
            "2,3 July,Soissons to Fontaine-au-Pire,100.0 km,Road stage",
            "3,4 July,Valenciennes to Roubaix,152.0 km,Road stage",
            "4,5 July,Roubaix to Le Havre,300.0 km,Road stage",
            "5,6 July,Le Havre to Le Mans,257.0 km,Road stage",
            "6,7 July,Chateaubriant to Nantes,58.0 km,Individual time trial",
            "7,8 July,Nantes to Ile d'Oleron,216.0 km,Road stage",
            "8,9 July,La Rochelle to Bordeaux,222.0 km,Road stage",
            "9,10 July,Bordeaux to Pau,207.0 km,Road stage",
            "10,11 July,Pau to Bagnères-de-Luchon,201.0 km,Road stage",
            "11,12 July,Bagnères-de-Luchon to Fleurance,177.0 km,Road stage",
            "12,13 July,Fleurance to Roquefort-sur-Soulzon,261.0 km,Road stage",
            "13,14 July,Roquefort-sur-Soulzon to Aurillac,210.0 km,Road stage",
            "14,15 July,Aurillac to Issoire,149.0 km,Road stage",
            "15,16 July,Clermont-Ferrand to Puy de Dôme,16.0 km,Individual time trial",
            "16,17 July,Issoire to Saint-Étienne,144.0 km,Road stage",
            "17,18 July,La Tour-du-Pin to Alpe d'Huez,233.0 km,Road stage",
            ",19 July,Rest day,Alpe d'Huez",
            "18,20 July,Le Bourg-d'Oisans to Morzine,247.0 km,Road stage",
            "19,21 July,Morzine to Avoriaz,15.0 km,Individual time trial",
            "20,22 July,Morzine to Dijon,291.0 km,Road stage",
            "21,23 July,Dijon,50.0 km,Individual time trial",
            "22,24 July,Alfortville to Paris,195.0 km,Road stage",
        ];
        let edition = tour_de_france_1983();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1984() {
        let route = [
            "P,29 June,Montreuil to Noisy-le-Sec,5.0 km,Individual time trial",
            "1,30 June,Bondy to Saint-Denis,149.0 km,Road stage",
            "2,1 July,Bobigny to Louvroil,249.0 km,Road stage",
            "3a,2 July,Louvroil to Valenciennes,51.0 km,Team time trial",
            "3b,2 July,Valenciennes to Bethune,83.0 km,Road stage",
            "4,3 July,Bethune to Cergy-Pontoise,207.0 km,Road stage",
            "5,4 July,Cergy-Pontoise to Alencon,202.0 km,Road stage",
            "6,5 July,Alencon to Le Mans,67.0 km,Individual time trial",
            "7,6 July,Le Mans to Nantes,192.0 km,Road stage",
            "8,7 July,Nantes to Bordeaux,338.0 km,Road stage",
            "9,8 July,Langon to Pau,198.0 km,Road stage",
            "10,9 July,Pau to Guzet-Neige,227.0 km,Road stage",
            "11,10 July,Saint-Girons to Blagnac,111.0 km,Road stage",
            "12,11 July,Blagnac to Rodez,220.0 km,Road stage",
            "13,12 July,Rodez to Domaine du Rouret,228.0 km,Road stage",
            "14,13 July,Domaine du Rouret to Grenoble,241.0 km,Road stage",
            ",14 July,Rest day,Grenoble",
            "15,15 July,Les Echelles to La Ruchère,22.0 km,Individual time trial",
            "16,16 July,Grenoble to Alpe d'Huez,151.0 km,Road stage",
            "17,17 July,Le Bourg-d'Oisans to La Plagne,185.0 km,Road stage",
            "18,18 July,La Plagne to Morzine,186.0 km,Road stage",
            "19,19 July,Morzine to Crans-Montana (Switzerland),141.0 km,Road stage",
            "20,20 July,Crans-Montana (Switzerland) to Villefranche-sur-Saone,320.0 km,Road stage",
            "21,21 July,Villié-Morgon to Villefranche-sur-Saone,51.0 km,Individual time trial",
            "22,22 July,Pantin to Paris,197.0 km,Road stage",
        ];
        let edition = tour_de_france_1984();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_de_france_1985() {
        let route = [
            "P,28 June,Plumelec,6.0 km,Individual time trial",
            "1,29 June,Vannes to Lanester,256.0 km,Road stage",
            "2,30 June,Lorient to Vitre,242.0 km,Road stage",
            "3,1 July,Vitre to Fougeres,242.0 km,Road stage",
            "4,2 July,Fougeres to Pont-Audemer,239.0 km,Road stage",
            "5,3 July,Neufchatel-en-Bray to Roubaix,224.0 km,Road stage",
            "6,4 July,Roubaix to Reims,222.0 km,Road stage",
            "7,5 July,Reims to Nancy,217.0 km,Road stage",
            "8,6 July,Sarrebourg to Strasbourg,75.0 km,Individual time trial",
            "9,7 July,Strasbourg to Epinal,174.0 km,Road stage",
            "10,8 July,Epinal to Pontarlier,204.0 km,Road stage",
            "11,9 July,Pontarlier to Morzine-Avoriaz,195.0 km,Road stage",
            "12,10 July,Morzine-Avoriaz to Lans-en-Vercors,269.0 km,Road stage",
            "13,11 July,Villard-de-Lans,32.0 km,Individual time trial",
            ",12 July,Rest day,Villard-de-Lans",
            "14,13 July,Autrans to Saint-Étienne,179.0 km,Road stage",
            "15,14 July,Saint-Étienne to Aurillac,238.0 km,Road stage",
            "16,15 July,Aurillac to Toulouse,247.0 km,Road stage",
            "17,16 July,Toulouse to Luz Ardiden,209.0 km,Road stage",
            "18a,17 July,Luz-Saint-Sauveur to Aubisque,53.0 km,Road stage",
            "18b,17 July,Laruns to Pau,83.0 km,Road stage",
            "19,18 July,Pau to Bordeaux,203.0 km,Road stage",
            "20,19 July,Montpon-Menesterol to Limoges,225.0 km,Road stage",
            "21,20 July,Lac de Vassiviere,46.0 km,Individual time trial",
            "22,21 July,Orléans to Paris,196.0 km,Road stage",
        ];
        let edition = tour_de_france_1985();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue including 1 split stage");
    }

    #[test]
    fn test_tour_de_france_1986() {
        let route = [
            "P,4 July,Boulogne-Billancourt,4.6 km,Individual time trial",
            "1,5 July,Nanterre to Sceaux,85.0 km,Road stage",
            "2,5 July,Meudon to Saint-Quentin-en-Yvelines,56.0 km,Team time trial",
            "3,6 July,Levallois-Perret to Lievin,214.0 km,Road stage",
            "4,7 July,Lievin to Evreux,243.0 km,Road stage",
            "5,8 July,Evreux to Villers-sur-Mer,124.5 km,Road stage",
            "6,9 July,Villers-sur-Mer to Cherbourg,200.0 km,Road stage",
            "7,10 July,Cherbourg to Saint-Hilaire-du-Harcouët,201.0 km,Road stage",
            "8,11 July,Saint-Hilaire-du-Harcouët to Nantes,204.0 km,Road stage",
            "9,12 July,Nantes,61.5 km,Individual time trial",
            "10,13 July,Nantes to Futuroscope,183.0 km,Road stage",
            "11,14 July,Futuroscope to Bordeaux,258.3 km,Road stage",
            "12,15 July,Bayonne to Pau,217.5 km,Road stage",
            "13,16 July,Pau to Superbagnères,186.0 km,Road stage",
            "14,17 July,Superbagnères to Blagnac,154.0 km,Road stage",
            "15,18 July,Carcassonne to Nîmes,225.5 km,Road stage",
            "16,19 July,Nîmes to Gap,246.5 km,Road stage",
            "17,20 July,Gap to Serre Chevalier,190.0 km,Road stage",
            "18,21 July,Briançon to Alpe d'Huez,162.5 km,Road stage",
            ",22 July,Rest day,Alpe d'Huez",
            "19,23 July,Villard-de-Lans to Saint-Étienne,179.5 km,Road stage",
            "20,24 July,Saint-Étienne,58.0 km,Individual time trial",
            "21,25 July,Saint-Étienne to Puy de Dôme,190.0 km,Road stage",
            "22,26 July,Clermont-Ferrand to Nevers,194.0 km,Road stage",
            "23,27 July,Cosne-sur-Loire to Paris,255.0 km,Road stage",
        ];
        let edition = tour_de_france_1986();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "23 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1987() {
        let route = [
            "P,1 July,West Berlin (West Germany),6.0 km,Individual time trial",
            "1,2 July,West Berlin (West Germany),105.0 km,Road stage",
            "2,2 July,West Berlin (West Germany),40.5 km,Team time trial",
            ",3 July,Rest day",
            "3,4 July,Karlsruhe (West Germany) to Stuttgart (West Germany),219.0 km,Road stage",
            "4,5 July,Stuttgart (West Germany) to Pforzheim (West Germany),79.0 km,Road stage",
            "5,5 July,Pforzheim (West Germany) to Strasbourg,112.5 km,Road stage",
            "6,6 July,Strasbourg to Epinal,169.0 km,Road stage",
            "7,7 July,Epinal to Troyes,211.0 km,Road stage",
            "8,8 July,Troyes to Epnay-sous-Senart,205.5 km,Road stage",
            "9,9 July,Orléans to Renaze,260.0 km,Road stage",
            "10,10 July,Saumur to Futuroscope,87.5 km,Individual time trial",
            "11,11 July,Poitiers to Chaumeil,255.0 km,Road stage",
            "12,12 July,Brive to Bordeaux,228.0 km,Road stage",
            "13,13 July,Bayonne to Pau,219.0 km,Road stage",
            "14,14 July,Pau to Luz Ardiden,166.0 km,Road stage",
            "15,15 July,Tarbes to Blagnac,164.0 km,Road stage",
            "16,16 July,Blagnac to Millau,216.5 km,Road stage",
            "17,17 July,Millau to Avignon,239.0 km,Road stage",
            ",18 July,Rest day,Avignon",
            "18,19 July,Carpentras to Mont Ventoux,36.5 km,Individual time trial",
            "19,20 July,Valreas to Villard-de-Lans,185.0 km,Road stage",
            "20,21 July,Villard-de-Lans to Alpe d'Huez,201.0 km,Road stage",
            "21,22 July,Le Bourg-d'Oisans to La Plagne,185.5 km,Road stage",
            "22,23 July,La Plagne to Morzine,186.0 km,Road stage",
            "23,24 July,Saint-Julien-en-Genevois to Dijon,224.5 km,Road stage",
            "24,25 July,Dijon,38.0 km,Individual time trial",
            "25,26 July,Creteil to Paris,192.0 km,Road stage",
        ];
        let edition = tour_de_france_1987();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "25 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1988() {
        let route = [
            "P,3 July,Pornichet to La Baule,1.0 km,Individual time trial",
            "1,4 July,Pontchateau to Machecoul,91.5 km,Road stage",
            "2,4 July,La Haie-Fouassiere to Ancenis,48.0 km,Team time trial",
            "3,5 July,Nantes to Le Mans,213.5 km,Road stage",
            "4,6 July,Le Mans to Evreux,158.0 km,Road stage",
            "5,7 July,Neufchatel-en-Bray to Lievin,147.5 km,Road stage",
            "6,8 July,Lievin to Wasquehal,52.0 km,Individual time trial",
            "7,9 July,Wasquehal to Reims,225.5 km,Road stage",
            "8,10 July,Reims to Nancy,219.0 km,Road stage",
            "9,11 July,Nancy to Strasbourg,160.5 km,Road stage",
            "10,12 July,Belfort to Besançon,149.5 km,Road stage",
            "11,13 July,Besançon to Morzine,232.0 km,Road stage",
            "12,14 July,Morzine to Alpe d'Huez,227.0 km,Road stage",
            "13,15 July,Grenoble to Villard-de-Lans,38.0 km,Individual time trial",
            ",16 July,Rest day,Blagnac",
            "14,17 July,Blagnac to Guzet-Neige,163.0 km,Road stage",
            "15,18 July,Saint-Girons to Luz Ardiden,187.5 km,Road stage",
            "16,19 July,Luz Ardiden to Pau,38.0 km,Road stage",
            "17,19 July,Pau to Bordeaux,210.0 km,Road stage",
            "18,20 July,Ruelle sur Tourve to Limoges,93.5 km,Road stage",
            "19,21 July,Limoges to Puy de Dôme,188.0 km,Road stage",
            "20,22 July,Clermont-Ferrand to Chalon-sur-Saône,233.5 km,Road stage",
            "21,23 July,Santennay,48.0 km,Individual time trial",
            "22,24 July,Nemours to Paris,172.5 km,Road stage",
        ];
        let edition = tour_de_france_1988();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1989() {
        let route = [
            "P,1 July,Luxembourg City (Luxembourg),7.8 km,Individual time trial",
            "1,2 July,Luxembourg City (Luxembourg),135.5 km,Road stage",
            "2,2 July,Luxembourg City (Luxembourg),46.0 km,Team time trial",
            "3,3 July,Luxembourg City (Luxembourg) to Spa (Belgium),241.0 km,Road stage",
            "4,4 July,Liège (Belgium) to Wasquehal,255.0 km,Road stage",
            ",5 July,Rest day,Dinard",
            "5,6 July,Dinard to Rennes,73.0 km,Individual time trial",
            "6,7 July,Rennes to Futuroscope,259.0 km,Road stage",
            "7,8 July,Poitiers to Bordeaux,258.5 km,Road stage",
            "8,9 July,Labastide-d'Armagnac to Pau,157.0 km,Road stage",
            "9,10 July,Pau to Cauterets,147.0 km,Road stage",
            "10,11 July,Cauterets to Superbagnères,136.0 km,Road stage",
            "11,12 July,Luchon to Blagnac,158.5 km,Road stage",
            "12,13 July,Toulouse to Montpellier,242.0 km,Road stage",
            "13,14 July,Montpellier to Marseille,179.0 km,Road stage",
            "14,15 July,Marseille to Gap,240.0 km,Road stage",
            "15,16 July,Gap to Orcières-Merlette,39.0 km,Individual time trial",
            ",17 July,Rest day,Orcières-Merlette",
            "16,18 July,Gap to Briançon,175.0 km,Road stage",
            "17,19 July,Briançon to Alpe d'Huez,165.0 km,Road stage",
            "18,20 July,Le Bourg-d'Oisans to Villard-de-Lans,91.5 km,Road stage",
            "19,21 July,Villard-de-Lans to Aix-les-Bains,125.0 km,Road stage",
            "20,22 July,Aix-les-Bains to L'Isle-d'Abeau,130.0 km,Road stage",
            "21,23 July,Versailles to Paris,24.5 km,Individual time trial",
        ];
        let edition = tour_de_france_1989();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1990() {
        let route = [
            "P,30 June,Futuroscope,6.3 km,Individual time trial",
            "1,1 July,Futuroscope,138.5 km,Road stage",
            "2,1 July,Futuroscope,44.5 km,Team time trial",
            "3,2 July,Poitiers to Nantes,233.0 km,Road stage",
            "4,3 July,Nantes to Mont Saint-Michel,203.0 km,Road stage",
            "5,4 July,Avranches to Rouen,301.0 km,Road stage",
            ",5 July,Rest day,Rouen",
            "6,6 July,Sarrebourg to Vittel,202.5 km,Road stage",
            "7,7 July,Vittel to Epinal,61.5 km,Individual time trial",
            "8,8 July,Epinal to Besançon,181.5 km,Road stage",
            "9,9 July,Besançon to Geneva (Switzerland),196.0 km,Road stage",
            "10,10 July,Geneva (Switzerland) to Saint-Gervais,118.5 km,Road stage",
            "11,11 July,Saint-Gervais to Alpe d'Huez,182.5 km,Road stage",
            "12,12 July,Fontaine to Villard-de-Lans,33.5 km,Individual time trial",
            ",13 July,Rest day,Villard-de-Lans",
            "13,14 July,Villard-de-Lans to Saint-Étienne,149.0 km,Road stage",
            "14,15 July,Le Puy-en-Velay to Millau,205.0 km,Road stage",
            "15,16 July,Millau to Revel,170.0 km,Road stage",
            "16,17 July,Blagnac to Luz Ardiden,215.0 km,Road stage",
            "17,18 July,Lourdes to Pau,150.0 km,Road stage",
            "18,19 July,Pau to Bordeaux,202.0 km,Road stage",
            "19,20 July,Castillon-la-Bataille to Limoges,182.5 km,Road stage",
            "20,21 July,Lac de Vassiviere,45.5 km,Individual time trial",
            "21,22 July,Bretigny-sur-Orge to Paris,182.5 km,Road stage",
        ];
        let edition = tour_de_france_1990();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1991() {
        let route = [
            "P,6 July,Lyon,5.4 km,Individual time trial",
            "1,7 July,Lyon,114.5 km,Road stage",
            "2,7 July,Bron to Chassieu,36.5 km,Team time trial",
            "3,8 July,Villeurbanne to Dijon,210.5 km,Road stage",
            "4,9 July,Dijon to Reims,286.0 km,Road stage",
            "5,10 July,Reims to Valenciennes,149.5 km,Road stage",
            "6,11 July,Arras to Le Havre,259.0 km,Road stage",
            "7,12 July,Le Havre to Argentan,167.0 km,Road stage",
            "8,13 July,Argentan to Alencon,73.0 km,Individual time trial",
            "9,14 July,Alencon to Rennes,161.0 km,Road stage",
            "10,15 July,Rennes to Quimper,207.5 km,Road stage",
            "11,16 July,Quimper to Saint-Herblain,246.0 km,Road stage",
            ",17 July,Rest day,Pau",
            "12,18 July,Pau to Jaca (Spain),192.0 km,Road stage",
            "13,19 July,Jaca (Spain) to Val Louron,232.0 km,Road stage",
            "14,20 July,Saint-Gaudens to Castres,172.5 km,Road stage",
            "15,21 July,Albi to Ales,235.0 km,Road stage",
            "16,22 July,Ales to Gap,215.0 km,Road stage",
            "17,23 July,Gap to Alpe d'Huez,125.0 km,Road stage",
            "18,24 July,Le Bourg-d'Oisans to Morzine,255.0 km,Road stage",
            "19,25 July,Morzine to Aix-les-Bains,177.0 km,Road stage",
            "20,26 July,Aix-les-Bains to Macon,166.0 km,Road stage",
            "21,27 July,Lugny to Macon,57.0 km,Individual time trial",
            "22,28 July,Melun to Paris,178.0 km,Road stage",
        ];
        let edition = tour_de_france_1991();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "22 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1992() {
        let route = [
            "P,4 July,San Sebastián (Spain),8.0 km,Individual time trial",
            "1,5 July,San Sebastián (Spain),194.5 km,Road stage",
            "2,6 July,San Sebastián (Spain) to Pau,255.0 km,Road stage",
            "3,7 July,Pau to Bordeaux,210.0 km,Road stage",
            "4,8 July,Libourne,63.5 km,Team time trial",
            "5,9 July,Nogent-sur-Oise to Wasquehal,196.0 km,Road stage",
            "6,10 July,Roubaix to Brussels (Belgium),167.0 km,Road stage",
            "7,11 July,Brussels (Belgium) to Valkenburg (Netherlands),196.5 km,Road stage",
            "8,12 July,Valkenburg (Netherlands) to Koblenz (Germany),206.5 km,Road stage",
            "9,13 July,Luxembourg City (Luxembourg),65.0 km,Individual time trial",
            "10,14 July,Luxembourg City (Luxembourg) to Strasbourg,217.0 km,Road stage",
            "11,15 July,Strasbourg to Mulhouse,249.5 km,Road stage",
            "12,16 July,Dole to Saint-Gervais,267.5 km,Road stage",
            ",17 July,Rest day,Dole",
            "13,18 July,Saint-Gervais to Sestrière (Italy),254.5 km,Road stage",
            "14,19 July,Sestrière (Italy) to Alpe d'Huez,186.5 km,Road stage",
            "15,20 July,Le Bourg-d'Oisans to Saint-Étienne,198.0 km,Road stage",
            "16,21 July,Saint-Étienne to La Bourboule,212.0 km,Road stage",
            "17,22 July,La Bourboule to Montlucon,189.0 km,Road stage",
            "18,23 July,Montlucon to Tours,212.0 km,Road stage",
            "19,24 July,Tours to Blois,64.0 km,Individual time trial",
            "20,25 July,Blois to Nanterre,222.0 km,Road stage",
            "21,26 July,La Defense to Paris,141.0 km,Road stage",
        ];
        let edition = tour_de_france_1992();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1993() {
        let route = [
            "P,3 July,Le Puy du Fou,6.8 km,Individual time trial",
            "1,4 July,Lucon to Les Sables-d'Olonne,215.0 km,Road stage",
            "2,5 July,Les Sables-d'Olonne to Vannes,227.5 km,Road stage",
            "3,6 July,Vannes to Dinard,189.5 km,Road stage",
            "4,7 July,Dinard to Avranches,81.0 km,Team time trial",
            "5,8 July,Avranches to Evreux,225.5 km,Road stage",
            "6,9 July,Evreux to Amiens,158.0 km,Road stage",
            "7,10 July,Peronne to Châlons-sur-Marne,199.0 km,Road stage",
            "8,11 July,Châlons-sur-Marne to Verdun,184.5 km,Road stage",
            "9,12 July,Lac de Madine,59.0 km,Individual time trial",
            ",13 July,Rest day,Villard-de-Lans",
            "10,14 July,Villard-de-Lans to Serre Chevalier,203.0 km,Road stage",
            "11,15 July,Serre Chevalier to Isola 2000,179.0 km,Road stage",
            "12,16 July,Isola to Marseille,286.5 km,Road stage",
            "13,17 July,Marseille to Montpellier,181.5 km,Road stage",
            "14,18 July,Montpellier to Perpignan,223.0 km,Road stage",
            "15,19 July,Perpignan to Pal (Andorra),231.5 km,Road stage",
            ",20 July,Rest day,Pal (Andorra)",
            "16,21 July,Pal (Andorra) to Saint-Lary-Soulan Pla d'Adet,230.0 km,Road stage",
            "17,22 July,Tarbes to Pau,190.0 km,Road stage",
            "18,23 July,Orthez to Bordeaux,195.5 km,Road stage",
            "19,24 July,Bretigny-sur-Orge to Montlhery,48.0 km,Individual time trial",
            "20,25 July,Viry-Chatillon to Paris,196.5 km,Road stage",
        ];
        let edition = tour_de_france_1993();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1994() {
        let route = [
            "P,2 July,Lille,7.2 km,Individual time trial",
            "1,3 July,Lille to Armentieres,234.0 km,Road stage",
            "2,4 July,Roubaix to Boulogne-sur-Mer,203.5 km,Road stage",
            "3,5 July,Calais to Eurotunnel,66.5 km,Team time trial",
            "4,6 July,Dover (United Kingdom) to Brighton (United Kingdom),204.5 km,Road stage",
            "5,7 July,Portsmouth (United Kingdom),187.0 km,Road stage",
            "6,8 July,Cherbourg to Rennes,270.5 km,Road stage",
            "7,9 July,Rennes to Futuroscope,259.5 km,Road stage",
            "8,10 July,Poitiers to Trelissac,218.5 km,Road stage",
            "9,11 July,Périgueux to Bergerac,64.0 km,Individual time trial",
            "10,12 July,Bergerac to Cahors,160.5 km,Road stage",
            "11,13 July,Cahors to Hautacam,263.5 km,Road stage",
            ",14 July,Rest day,Lourdes",
            "12,15 July,Lourdes to Luz Ardiden,204.5 km,Road stage",
            "13,16 July,Bagnères-de-Bigorre to Albi,223.0 km,Road stage",
            "14,17 July,Castres to Montpellier,202.0 km,Road stage",
            "15,18 July,Montpellier to Carpentras,231.0 km,Road stage",
            "16,19 July,Valreas to Alpe d'Huez,224.5 km,Road stage",
            "17,20 July,Le Bourg-d'Oisans to Val Thorens,149.0 km,Road stage",
            "18,21 July,Moûtiers to Cluses,174.5 km,Road stage",
            "19,22 July,Cluses to Avoriaz,47.5 km,Individual time trial",
            "20,23 July,Morzine to Lac Saint-Point,208.5 km,Road stage",
            "21,24 July,Disneyland Paris to Paris,175.0 km,Road stage",
        ];
        let edition = tour_de_france_1994();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1995() {
        let route = [
            "P,1 July,Saint-Brieuc,7.3 km,Individual time trial",
            "1,2 July,Dinan to Lannion,233.5 km,Road stage",
            "2,3 July,Perros-Guirec to Vitre,235.5 km,Road stage",
            "3,4 July,Mayenne to Alencon,67.0 km,Team time trial",
            "4,5 July,Alencon to Le Havre,162.0 km,Road stage",
            "5,6 July,Fecamp to Dunkerque,261.0 km,Road stage",
            "6,7 July,Dunkerque to Charleroi (Belgium),202.0 km,Road stage",
            "7,8 July,Charleroi (Belgium) to Liège (Belgium),203.0 km,Road stage",
            "8,9 July,Huy (Belgium) to Seraing (Belgium),54.0 km,Individual time trial",
            ",10 July,Rest day,Le Grand Bornand",
            "9,11 July,Le Grand Bornand to La Plagne,160.0 km,Road stage",
            "10,12 July,La Plagne to Alpe d'Huez,162.5 km,Road stage",
            "11,13 July,Le Bourg-d'Oisans to Saint-Étienne,199.0 km,Road stage",
            "12,14 July,Saint-Étienne to Mende,222.5 km,Road stage",
            "13,15 July,Mende to Revel,245.0 km,Road stage",
            "14,16 July,Saint-Orens-de-Gameville to Guzet-Neige,164.0 km,Road stage",
            ",17 July,Rest day,Saint-Girons",
            "15,18 July,Saint-Girons to Cauterets,206.0 km,Road stage",
            "16,19 July,Tarbes to Pau,149.0 km,Road stage",
            "17,20 July,Pau to Bordeaux,246.0 km,Road stage",
            "18,21 July,Montpon-Menesterol to Limoges,246.0 km,Road stage",
            "19,22 July,Lac de Vassiviere,46.5 km,Individual time trial",
            "20,23 July,Sainte-Genevieve-des-Bois to Paris,155.0 km,Road stage",
        ];
        let edition = tour_de_france_1995();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1996() {
        let route = [
            "P,29 June,'s-Hertogenbosch (Netherlands),9.4 km,Individual time trial",
            "1,30 June,'s-Hertogenbosch (Netherlands),209.0 km,Road stage",
            "2,1 July,'s-Hertogenbosch (Netherlands) to Wasquehal,247.5 km,Road stage",
            "3,2 July,Wasquehal to Nogent-sur-Oise,195.0 km,Road stage",
            "4,3 July,Soissons to Lac de Madine,232.0 km,Road stage",
            "5,4 July,Lac de Madine to Besançon,242.0 km,Road stage",
            "6,5 July,Arc-et-Senans to Aix-les-Bains,207.0 km,Road stage",
            "7,6 July,Chambery to Les Arcs,200.0 km,Road stage",
            "8,7 July,Bourg-Saint-Maurice to Val d'Isère,30.5 km,Individual time trial",
            "9,8 July,Le Monetier-les-Bains to Sestrière (Italy),46.0 km,Road stage",
            "10,9 July,Turin (Italy) to Gap,208.5 km,Road stage",
            ",10 July,Rest day,Gap",
            "11,11 July,Gap to Valence,202.0 km,Road stage",
            "12,12 July,Valence to Le Puy-en-Velay,143.5 km,Road stage",
            "13,13 July,Le Puy-en-Velay to Super Besse,177.0 km,Road stage",
            "14,14 July,Besse to Tulle,186.5 km,Road stage",
            "15,15 July,Brive-la-Gaillarde to Villeneuve-sur-Lot,176.0 km,Road stage",
            "16,16 July,Agen to Hautacam,199.0 km,Road stage",
            "17,17 July,Argeles-Gazost to Pamplona (Spain),262.0 km,Road stage",
            "18,18 July,Pamplona (Spain) to Hendaye,154.5 km,Road stage",
            "19,19 July,Hendaye to Bordeaux,226.5 km,Road stage",
            "20,20 July,Bordeaux to Saint-Emilion,63.5 km,Individual time trial",
            "21,21 July,Palaiseau to Paris,147.5 km,Road stage",
        ];
        let edition = tour_de_france_1996();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1997() {
        let route = [
            "P,5 July,Rouen,7.3 km,Individual time trial",
            "1,6 July,Rouen to Forges-les-Eaux,192.0 km,Road stage",
            "2,7 July,Saint-Valery-en-Caux to Vire,262.0 km,Road stage",
            "3,8 July,Vire to Plumelec,224.0 km,Road stage",
            "4,9 July,Plumelec to Le Puy du Fou,223.0 km,Road stage",
            "5,10 July,Chantonnay to La Chatre,261.5 km,Road stage",
            "6,11 July,Le Blanc to Marennes,217.5 km,Road stage",
            "7,12 July,Marennes to Bordeaux,194.0 km,Road stage",
            "8,13 July,Sauternes to Pau,161.5 km,Road stage",
            "9,14 July,Pau to Loudenvielle,182.0 km,Road stage",
            "10,15 July,Luchon to Andorra-Arcalis (Andorra),182.0 km,Road stage",
            "11,16 July,Andorra-Arcalis (Andorra) to Perpignan,192.0 km,Road stage",
            ",17 July,Rest day,Saint-Étienne",
            "12,18 July,Saint-Étienne,55.0 km,Individual time trial",
            "13,19 July,Saint-Étienne to Alpe d'Huez,203.5 km,Road stage",
            "14,20 July,Le Bourg-d'Oisans to Courchevel,148.0 km,Road stage",
            "15,21 July,Courchevel to Morzine,208.5 km,Road stage",
            "16,22 July,Morzine to Fribourg (Switzerland),181.0 km,Road stage",
            "17,23 July,Fribourg (Switzerland) to Colmar,218.5 km,Road stage",
            "18,24 July,Colmar to Montbeliard,175.5 km,Road stage",
            "19,25 July,Montbeliard to Dijon,172.0 km,Road stage",
            "20,26 July,Disneyland Paris,63.0 km,Individual time trial",
            "21,27 July,Disneyland Paris to Paris,149.5 km,Road stage",
        ];
        let edition = tour_de_france_1997();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1998() {
        let route = [
            "P,11 July,Dublin (Ireland),5.6 km,Individual time trial",
            "1,12 July,Dublin (Ireland),180.5 km,Road stage",
            "2,13 July,Enniscorthy (Ireland) to Cork (Ireland),205.5 km,Road stage",
            "3,14 July,Roscoff to Lorient,169.0 km,Road stage",
            "4,15 July,Plouay to Cholet,252.0 km,Road stage",
            "5,16 July,Cholet to Chateauroux,228.5 km,Road stage",
            "6,17 July,La Chatre to Brive-la-Gaillarde,204.5 km,Road stage",
            "7,18 July,Meyrignac-l'Eglise to Correze,58.0 km,Individual time trial",
            "8,19 July,Brive-la-Gaillarde to Montauban,190.5 km,Road stage",
            "9,20 July,Montauban to Pau,210.0 km,Road stage",
            "10,21 July,Pau to Luchon,196.5 km,Road stage",
            "11,22 July,Luchon to Plateau de Beille,170.0 km,Road stage",
            ",23 July,Rest day,Ariege",
            "12,24 July,Tarascon-sur-Ariege to Cap d'Agde,222.0 km,Road stage",
            "13,25 July,Frontignan la Peyrade to Carpentras,196.0 km,Road stage",
            "14,26 July,Valreas to Grenoble,186.5 km,Road stage",
            "15,27 July,Grenoble to Les Deux Alpes,189.0 km,Road stage",
            "16,28 July,Vizelle to Albertville,204.0 km,Road stage",
            "17,29 July,Albertville to Aix-les-Bains,149.0 km,Road stage",
            "18,30 July,Aix-les-Bains to Neuchatel (Switzerland),218.5 km,Road stage",
            "19,31 July,La Chaux-de-Fonds (Switzerland) to Autun,242.0 km,Road stage",
            "20,1 August,Montceau-les-Mines to Le Creusot,52.0 km,Individual time trial",
            "21,2 August,Melun to Paris,147.5 km,Road stage",
        ];
        let edition = tour_de_france_1998();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_1999() {
        let route = [
            "P,3 July,Le Puy du Fou,6.8 km,Individual time trial",
            "1,4 July,Montaigu to Challans,208.0 km,Road stage",
            "2,5 July,Challans to Saint-Nazaire,176.0 km,Road stage",
            "3,6 July,Nantes to Laval,194.5 km,Road stage",
            "4,7 July,Laval to Blois,194.5 km,Road stage",
            "5,8 July,Bonneval to Amiens,233.5 km,Road stage",
            "6,9 July,Amiens to Mauberge,171.5 km,Road stage",
            "7,10 July,Avesnes-sur-Helpe to Thionville,227.0 km,Road stage",
            "8,11 July,Metz,56.5 km,Individual time trial",
            ",12 July,Rest day,Le Grand Bornand",
            "9,13 July,Le Grand Bornand to Sestrière (Italy),213.5 km,Road stage",
            "10,14 July,Sestrière (Italy) to Alpe d'Huez,220.5 km,Road stage",
            "11,15 July,Le Bourg-d'Oisans to Saint-Étienne,198.5 km,Road stage",
            "12,16 July,Saint Galmier to Saint-Flour,201.5 km,Road stage",
            "13,17 July,Saint-Flour to Albi,236.5 km,Road stage",
            "14,18 July,Castres to Saint-Gaudens,199.0 km,Road stage",
            ",19 July,Rest day,Saint-Gaudens",
            "15,20 July,Saint-Gaudens to Piau-Engaly,173.0 km,Road stage",
            "16,21 July,Lannemezan to Pau,192.0 km,Road stage",
            "17,22 July,Mourenx to Bordeaux,200.0 km,Road stage",
            "18,23 July,Jonzac to Futuroscope,187.5 km,Road stage",
            "19,24 July,Futuroscope,57.0 km,Individual time trial",
            "20,25 July,Arpajon to Paris,143.5 km,Road stage",
        ];
        let edition = tour_de_france_1999();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2000() {
        let route = [
            "1,1 July,Futuroscope,16.5 km,Individual time trial",
            "2,2 July,Futuroscope to Loudon,194.0 km,Road stage",
            "3,3 July,Loudon to Nantes,161.5 km,Road stage",
            "4,4 July,Nantes to Saint-Nazaire,70.0 km,Team time trial",
            "5,5 July,Vannes to Vitre,202.0 km,Road stage",
            "6,6 July,Vitre to Tours,198.5 km,Road stage",
            "7,7 July,Tours to Limoges,205.5 km,Road stage",
            "8,8 July,Limoges to Villeneuve-sur-Lot,203.5 km,Road stage",
            "9,9 July,Agen to Dax,181.0 km,Road stage",
            "10,10 July,Dax to Hautacam,205.0 km,Road stage",
            "11,11 July,Bagnères-de-Bigorre to Revel,218.5 km,Road stage",
            ",12 July,Rest day,Provence",
            "12,13 July,Carpentras to Mont Ventoux,149.0 km,Road stage",
            "13,14 July,Avignon to Draguignan,185.5 km,Road stage",
            "14,15 July,Draguignan to Briançon,249.5 km,Road stage",
            "15,16 July,Briançon to Courchevel,173.5 km,Road stage",
            ",17 July,Rest day,Courchevel",
            "16,18 July,Courchevel to Morzine,196.5 km,Road stage",
            "17,19 July,Evian-les-Bains to Lausanne (Switzerland),155.0 km,Road stage",
            "18,20 July,Lausanne (Switzerland) to Freiburg (Germany),246.5 km,Road stage",
            "19,21 July,Freiburg (Germany) to Mulhouse,58.5 km,Individual time trial",
            "20,22 July,Belfort to Troyes,254.5 km,Road stage",
            "21,23 July,Paris,138.0 km,Road stage",
        ];
        let edition = tour_de_france_2000();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2001() {
        let route = [
            "P,7 July,Dunkerque,8.2 km,Individual time trial",
            "1,8 July,Saint-Omer to Boulogne-sur-Mer,194.5 km,Road stage",
            "2,9 July,Calais to Antwerp (Belgium),220.5 km,Road stage",
            "3,10 July,Antwerp (Belgium) to Seraing (Belgium),198.5 km,Road stage",
            "4,11 July,Huy (Belgium) to Verdun,215.0 km,Road stage",
            "5,12 July,Verdun to Bar-le-Duc,67.0 km,Team time trial",
            "6,13 July,Commercy to Strasbourg,211.5 km,Road stage",
            "7,14 July,Strasbourg to Colmar,162.5 km,Road stage",
            "8,15 July,Colmar to Pontarlier,222.5 km,Road stage",
            "9,16 July,Pontarlier to Aix-les-Bains,185.0 km,Road stage",
            "10,17 July,Aix-les-Bains to Alpe d'Huez,209.0 km,Road stage",
            "11,18 July,Grenoble to Chamrousse,32.0 km,Individual time trial",
            ",19 July,Rest day,Perpignan",
            "12,20 July,Perpignan to Plateau de Bonascre,165.5 km,Road stage",
            "13,21 July,Foix to Saint-Lary-Soulan Pla d'Adet,194.0 km,Road stage",
            "14,22 July,Tarbes to Luz Ardiden,141.5 km,Road stage",
            ",23 July,Rest day,Pau",
            "15,24 July,Pau to Lavaur,232.5 km,Road stage",
            "16,25 July,Castelsarrasin to Sarran,229.5 km,Road stage",
            "17,26 July,Brive-la-Gaillarde to Montlucon,194.0 km,Road stage",
            "18,27 July,Montlucon to Saint-Amand-Montrond,61.0 km,Individual time trial",
            "19,28 July,Orléans to Evry,149.5 km,Road stage",
            "20,29 July,Corbeil-Essonnes to Paris,160.5 km,Road stage",
        ];
        let edition = tour_de_france_2001();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2002() {
        let route = [
            "P,6 July,Luxembourg City (Luxembourg),7.0 km,Individual time trial",
            "1,7 July,Luxembourg City (Luxembourg),192.5 km,Road stage",
            "2,8 July,Luxembourg City (Luxembourg) to Saarbrücken (Germany),181.0 km,Road stage",
            "3,9 July,Metz to Reims,174.5 km,Road stage",
            "4,10 July,Epernay to Chateau-Thierry,67.5 km,Team time trial",
            "5,11 July,Soissons to Rouen,195.0 km,Road stage",
            "6,12 July,Forges-les-Eaux to Alencon,199.5 km,Road stage",
            "7,13 July,Bagnoles-de-l'Orne to Avranches,176.0 km,Road stage",
            "8,14 July,Saint-Martin-de-Landelles to Plouay,217.5 km,Road stage",
            "9,15 July,Lanester to Lorient,52.0 km,Individual time trial",
            ",16 July,Rest day,Bordeaux",
            "10,17 July,Bazas to Pau,147.0 km,Road stage",
            "11,18 July,Pau to La Mongie,158.0 km,Road stage",
            "12,19 July,Lannemezan to Plateau de Beille,199.5 km,Road stage",
            "13,20 July,Lavelanet to Beziers,171.0 km,Road stage",
            "14,21 July,Lodève to Mont Ventoux,221.0 km,Road stage",
            ",22 July,Rest day,Vaucluse",
            "15,23 July,Vaison-la-Romaine to Les Deux Alpes,226.5 km,Road stage",
            "16,24 July,Les Deux Alpes to La Plagne,179.5 km,Road stage",
            "17,25 July,Aime to Cluses,142.0 km,Road stage",
            "18,26 July,Cluses to Bourg-en-Bresse,176.5 km,Road stage",
            "19,27 July,Regnie-Durette to Macon,50.0 km,Individual time trial",
            "20,28 July,Melun to Paris,144.0 km,Road stage",
        ];
        let edition = tour_de_france_2002();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2003() {
        let route = [
            "P,5 July,Paris,6.5 km,Individual time trial",
            "1,6 July,Saint-Denis to Meaux,168.0 km,Road stage",
            "2,7 July,La Ferte-sous-Jouarre to Sedan,204.5 km,Road stage",
            "3,8 July,Charleville-Mezieres to Saint-Dizier,167.5 km,Road stage",
            "4,9 July,Joinville to Saint-Dizier,69.0 km,Team time trial",
            "5,10 July,Troyes to Nevers,196.5 km,Road stage",
            "6,11 July,Nevers to Lyon,230.0 km,Road stage",
            "7,12 July,Lyon to Morzine,230.5 km,Road stage",
            "8,13 July,Sallanches to Alpe d'Huez,219.0 km,Road stage",
            "9,14 July,Le Bourg-d'Oisans to Gap,184.5 km,Road stage",
            "10,15 July,Gap to Marseille,219.5 km,Road stage",
            ",16 July,Rest day,Narbonne",
            "11,17 July,Narbonne to Toulouse,153.5 km,Road stage",
            "12,18 July,Gaillac to Cap Decouverte,47.0 km,Individual time trial",
            "13,19 July,Toulouse to Ax 3 Domaines,197.5 km,Road stage",
            "14,20 July,Saint-Girons to Loudenvielle,191.5 km,Road stage",
            "15,21 July,Bagnères-de-Bigorre to Luz Ardiden,159.5 km,Road stage",
            ",22 July,Rest day,Pau",
            "16,23 July,Pau to Bayonne,197.5 km,Road stage",
            "17,24 July,Dax to Bordeaux,181.0 km,Road stage",
            "18,25 July,Bordeaux to Saint-Maixent-l'Ecole,203.5 km,Road stage",
            "19,26 July,Pornic to Nantes,49.0 km,Individual time trial",
            "20,27 July,Ville-d'Avray to Paris,152.0 km,Road stage",
        ];
        let edition = tour_de_france_2003();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2004() {
        let route = [
            "P,3 July,Liège (Belgium),6.1 km,Individual time trial",
            "1,4 July,Liège (Belgium) to Charleroi (Belgium),202.5 km,Road stage",
            "2,5 July,Charleroi (Belgium) to Namur (Belgium),197.0 km,Road stage",
            "3,6 July,Waterloo (Belgium) to Wasquehal,210.0 km,Road stage",
            "4,7 July,Cambrai to Arras,64.5 km,Team time trial",
            "5,8 July,Amiens to Chartres,200.5 km,Road stage",
            "6,9 July,Bonneval to Angers,196.0 km,Road stage",
            "7,10 July,Chateaubriant to Saint-Brieuc,204.5 km,Road stage",
            "8,11 July,Lamballe to Quimper,168.0 km,Road stage",
            ",12 July,Rest day,Limoges",
            "9,13 July,Saint-Leonard-de-Noblat to Gueret,160.5 km,Road stage",
            "10,14 July,Limoges to Saint-Flour,237.0 km,Road stage",
            "11,15 July,Saint-Flour to Figeac,164.0 km,Road stage",
            "12,16 July,Castelsarrasin to La Mongie,197.5 km,Road stage",
            "13,17 July,Lannemezan to Plateau de Beille,205.5 km,Road stage",
            "14,18 July,Carcassonne to Nîmes,192.5 km,Road stage",
            ",19 July,Rest day,Nîmes",
            "15,20 July,Valreas to Villard-de-Lans,180.5 km,Road stage",
            "16,21 July,Le Bourg-d'Oisans to Alpe d'Huez,15.5 km,Individual time trial",
            "17,22 July,Le Bourg-d'Oisans to Le Grand Bornand,204.5 km,Road stage",
            "18,23 July,Annemasse to Lons-le-Saunier,166.5 km,Road stage",
            "19,24 July,Besançon,55.0 km,Individual time trial",
            "20,25 July,Montereau-Fault-Yonne to Paris,163.0 km,Road stage",
        ];
        let edition = tour_de_france_2004();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2005() {
        let route = [
            "1,2 July,Fromentine to Noirmoutier-en-l'Île,19.0 km,Individual time trial",
            "2,3 July,Challans to Les Essarts,181.5 km,Road stage",
            "3,4 July,La Chataigneraie to Tours,212.5 km,Road stage",
            "4,5 July,Tours to Blois,67.5 km,Team time trial",
            "5,6 July,Chambord to Montargis,183.0 km,Road stage",
            "6,7 July,Troyes to Nancy,199.0 km,Road stage",
            "7,8 July,Luneville to Karlsruhe (Germany),228.5 km,Road stage",
            "8,9 July,Pforzheim (Germany) to Gerardmer,231.5 km,Road stage",
            "9,10 July,Gerardmer to Mulhouse,171.0 km,Road stage",
            "10,11 July,Grenoble to Courchevel,177.0 km,Road stage",
            ",12 July,Rest day,Grenoble",
            "11,13 July,Courchevel to Briançon,173.0 km,Road stage",
            "12,14 July,Briançon to Digne-les-Bains,187.0 km,Road stage",
            "13,15 July,Miramas to Montpellier,173.5 km,Road stage",
            "14,16 July,Agde to Ax 3 Domaines,220.5 km,Road stage",
            "15,17 July,Lezat-sur-Leze to Saint-Lary-Soulan Pla d'Adet,205.5 km,Road stage",
            "16,18 July,Mourenx to Pau,180.5 km,Road stage",
            ",19 July,Rest day,Pau",
            "17,20 July,Pau to Revel,239.5 km,Road stage",
            "18,21 July,Albi to Mende,189.0 km,Road stage",
            "19,22 July,Issoire to Le Puy-en-Velay,153.5 km,Road stage",
            "20,23 July,Saint-Étienne,55.5 km,Individual time trial",
            "21,24 July,Corbeil-Essonnes to Paris,144.5 km,Road stage",
        ];
        let edition = tour_de_france_2005();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2006() {
        let route = [
            "P,1 July,Strasbourg,7.1 km,Individual time trial",
            "1,2 July,Strasbourg,184.5 km,Road stage",
            "2,3 July,Obernai to Esch-sur-Alzette (Luxembourg),228.5 km,Road stage",
            "3,4 July,Esch-sur-Alzette (Luxembourg) to Valkenburg (Netherlands),216.5 km,Road stage",
            "4,5 July,Huy (Belgium) to Saint-Quentin,207.0 km,Road stage",
            "5,6 July,Beauvais to Caen,225.0 km,Road stage",
            "6,7 July,Lisieux to Vitre,189.0 km,Road stage",
            "7,8 July,Saint Gregoire to Rennes,52.0 km,Individual time trial",
            "8,9 July,Saint-Meen-le-Grand to Lorient,181.0 km,Road stage",
            ",10 July,Rest day,Bordeaux",
            "9,11 July,Bordeaux to Dax,169.5 km,Road stage",
            "10,12 July,Cambo-les-Bains to Pau,190.5 km,Road stage",
            "11,13 July,Tarbes to Val d'Aran/Pla-de-Beret,206.5 km,Road stage",
            "12,14 July,Luchon to Carcassonne,211.5 km,Road stage",
            "13,15 July,Beziers to Montelimar,230.0 km,Road stage",
            "14,16 July,Montelimar to Gap,180.5 km,Road stage",
            ",17 July,Rest day,Gap",
            "15,18 July,Gap to Alpe d'Huez,187.0 km,Road stage",
            "16,19 July,Le Bourg-d'Oisans to La Toussuire,182.0 km,Road stage",
            "17,20 July,Saint-Jean-de-Maurienne to Morzine,200.5 km,Road stage",
            "18,21 July,Morzine to Macon,197.0 km,Road stage",
            "19,22 July,Le Creusot to Montceau-les-Mines,57.0 km,Individual time trial",
            "20,23 July,Antony/Parc de Sceaux to Paris,154.5 km,Road stage",
        ];
        let edition = tour_de_france_2006();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2007() {
        let route = [
            "P,7 July,London (United Kingdom),7.9 km,Individual time trial",
            "1,8 July,London (United Kingdom) to Canterbury (United Kingdom),203.0 km,Road stage",
            "2,9 July,Dunkerque to Ghent (Belgium),168.5 km,Road stage",
            "3,10 July,Waregem (Belgium) to Compiegne,236.5 km,Road stage",
            "4,11 July,Villers-Cotterets to Joigny,193.0 km,Road stage",
            "5,12 July,Chablis to Autun,182.5 km,Road stage",
            "6,13 July,Semur-en-Auxois to Bourg-en-Bresse,199.5 km,Road stage",
            "7,14 July,Bourg-en-Bresse to Le Grand Bornand,197.5 km,Road stage",
            "8,15 July,Le Grand Bornand to Tignes,165.0 km,Road stage",
            ",16 July,Rest day,Tignes",
            "9,17 July,Val d'Isère to Briançon,159.5 km,Road stage",
            "10,18 July,Tallard to Marseille,229.5 km,Road stage",
            "11,19 July,Marseille to Montpellier,182.5 km,Road stage",
            "12,20 July,Montpellier to Castres,178.5 km,Road stage",
            "13,21 July,Albi,54.0 km,Individual time trial",
            "14,22 July,Mazamet to Plateau de Beille,197.0 km,Road stage",
            "15,23 July,Foix to Loudenvielle,196.0 km,Road stage",
            ",24 July,Rest day,Pau",
            "16,25 July,Orthez to Gourette-Col d'Aubisque,218.5 km,Road stage",
            "17,26 July,Pau to Castelsarrasin,188.5 km,Road stage",
            "18,27 July,Cahors to Angoulême,211.0 km,Road stage",
            "19,28 July,Cognac to Angoulême,55.5 km,Individual time trial",
            "20,29 July,Marcoussis to Paris,146.0 km,Road stage",
        ];
        let edition = tour_de_france_2007();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2008() {
        let route = [
            "1,5 July,Brest to Plumelec,197.5 km,Road stage",
            "2,6 July,Aulay to Saint-Brieuc,164.5 km,Road stage",
            "3,7 July,Saint-Malo to Nantes,208.0 km,Road stage",
            "4,8 July,Cholet,29.5 km,Individual time trial",
            "5,9 July,Cholet to Chateauroux,232.0 km,Road stage",
            "6,10 July,Aigurande to Super Besse,195.0 km,Road stage",
            "7,11 July,Brioude to Aurillac,159.0 km,Road stage",
            "8,12 July,Figeac to Toulouse,172.5 km,Road stage",
            "9,13 July,Toulouse to Bagnères-de-Bigorre,224.0 km,Road stage",
            "10,14 July,Pau to Hautacam,156.0 km,Road stage",
            ",15 July,Rest day,Pau",
            "11,16 July,Lannemezan to Foix,167.5 km,Road stage",
            "12,17 July,Lavalanet to Narbonne,168.5 km,Road stage",
            "13,18 July,Narbonne to Nîmes,182.0 km,Road stage",
            "14,19 July,Nîmes to Digne-les-Bains,194.5 km,Road stage",
            "15,20 July,Embrun to Prato Nevoso (Italy),183.0 km,Road stage",
            ",21 July,Rest day,Cuneo (Italy)",
            "16,22 July,Cuneo (Italy) to Jausiers,157.0 km,Road stage",
            "17,23 July,Embrun to Alpe d'Huez,210.5 km,Road stage",
            "18,24 July,Le Bourg-d'Oisans to Saint-Étienne,196.5 km,Road stage",
            "19,25 July,Roanne to Montlucon,165.5 km,Road stage",
            "20,26 July,Cerilly to Saint-Amand-Montrond,53.0 km,Individual time trial",
            "21,27 July,Etampes to Paris,143.0 km,Road stage",
        ];
        let edition = tour_de_france_2008();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2009() {
        let route = [
            "1,4 July,Monaco (Monaco),15.5 km,Individual time trial",
            "2,5 July,Monaco (Monaco) to Brignoles,187.0 km,Road stage",
            "3,6 July,Marseille to La Grande-Motte,196.5 km,Road stage",
            "4,7 July,Montpellier,39.0 km,Team time trial",
            "5,8 July,Cap d'Agde to Perpignan,196.5 km,Road stage",
            "6,9 July,Girona (Spain) to Barcelona (Spain),181.5 km,Road stage",
            "7,10 July,Barcelona (Spain) to Andorra-Arcalis (Andorra),224.0 km,Road stage",
            "8,11 July,Andorra la Vella (Andorra) to Saint-Girons,176.5 km,Road stage",
            "9,12 July,Saint-Girons to Tarbes,160.5 km,Road stage",
            ",13 July,Rest day,Limoges",
            "10,14 July,Limoges to Issoudun,194.5 km,Road stage",
            "11,15 July,Vatan to Saint-Fargeau,192.0 km,Road stage",
            "12,16 July,Tonnerre to Vittel,211.5 km,Road stage",
            "13,17 July,Vittel to Colmar,200.0 km,Road stage",
            "14,18 July,Colmar to Besançon,199.0 km,Road stage",
            "15,19 July,Pontarlier to Verbier (Switzerland),207.5 km,Road stage",
            ",20 July,Rest day,Verbier (Switzerland)",
            "16,21 July,Martigny (Switzerland) to Bourg-Saint-Maurice,159.0 km,Road stage",
            "17,22 July,Bourg-Saint-Maurice to Le Grand Bornand,169.5 km,Road stage",
            "18,23 July,Annecy,40.5 km,Individual time trial",
            "19,24 July,Bourgoin-Jallieu to Aubenas,178.0 km,Road stage",
            "20,25 July,Montelimar to Mont Ventoux,167.0 km,Road stage",
            "21,26 July,Montereau-Fault-Yonne to Paris,164.0 km,Road stage",
        ];
        let edition = tour_de_france_2009();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2010() {
        let route = [
            "P,3 July,Rotterdam (Netherlands),8.9 km,Individual time trial",
            "1,4 July,Rotterdam (Netherlands) to Brussels (Belgium),223.5 km,Road stage",
            "2,5 July,Brussels (Belgium) to Spa (Belgium),201.0 km,Road stage",
            "3,6 July,Wanze (Belgium) to Arenberg Porte du Hainaut,213.0 km,Road stage",
            "4,7 July,Cambrai to Reims,153.5 km,Road stage",
            "5,8 July,Epernay to Montargis,187.5 km,Road stage",
            "6,9 July,Montargis to Gueugnon,227.5 km,Road stage",
            "7,10 July,Tournus to Station des Rousses,165.6 km,Road stage",
            "8,11 July,Station des Rousses to Morzine-Avoriaz,189.0 km,Road stage",
            ",12 July,Rest day,Morzine-Avoriaz",
            "9,13 July,Morzine-Avoriaz to Saint-Jean-de-Maurienne,204.5 km,Road stage",
            "10,14 July,Chambery to Gap,179.0 km,Road stage",
            "11,15 July,Sisteron to Bourg-lès-Valence,184.5 km,Road stage",
            "12,16 July,Bourg-de-Peage to Mende,210.5 km,Road stage",
            "13,17 July,Rodez to Revel,196.0 km,Road stage",
            "14,18 July,Revel to Ax 3 Domaines,184.5 km,Road stage",
            "15,19 July,Pamiers to Bagnères-de-Luchon,187.5 km,Road stage",
            "16,20 July,Bagnères-de-Luchon to Pau,199.5 km,Road stage",
            ",21 July,Rest day,Pau",
            "17,22 July,Pau to Col du Tourmalet,174.0 km,Road stage",
            "18,23 July,Salies-de-Bearn to Bordeaux,198.0 km,Road stage",
            "19,24 July,Bordeaux to Pauillac,52.0 km,Individual time trial",
            "20,25 July,Longjumeau to Paris,102.5 km,Road stage",
        ];
        let edition = tour_de_france_2010();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "20 stages + Prologue");
    }

    #[test]
    fn test_tour_de_france_2011() {
        let route = [
            "1,2 July,Passage du Gois to Mont des Alouettes,191.5 km,Road stage",
            "2,3 July,Les Essarts,23.0 km,Team time trial",
            "3,4 July,Olonne-sur-Mer to Redon,198.0 km,Road stage",
            "4,5 July,Lorient to Mûr-de-Bretagne,172.5 km,Road stage",
            "5,6 July,Carhaix to Cap Frehel,164.5 km,Road stage",
            "6,7 July,Dinan to Lisieux,226.5 km,Road stage",
            "7,8 July,Le Mans to Chateauroux,218.0 km,Road stage",
            "8,9 July,Aigurande to Super Besse,189.0 km,Road stage",
            "9,10 July,Issoire to Saint-Flour,208.0 km,Road stage",
            ",11 July,Rest day,Le Lioran",
            "10,12 July,Aurillac to Carmaux,158.0 km,Road stage",
            "11,13 July,Blaye-les-Mines to Lavaur,167.5 km,Road stage",
            "12,14 July,Cugnaux to Luz Ardiden,211.0 km,Road stage",
            "13,15 July,Pau to Lourdes,152.5 km,Road stage",
            "14,16 July,Saint-Gaudens to Plateau de Beille,168.5 km,Road stage",
            "15,17 July,Limoux to Montpellier,192.5 km,Road stage",
            ",18 July,Rest day,Drome",
            "16,19 July,Saint-Paul-Trois-Chateaux to Gap,162.5 km,Road stage",
            "17,20 July,Gap to Pinerolo (Italy),179.0 km,Road stage",
            "18,21 July,Pinerolo (Italy) to Col du Galibier - Serre Chevalier,200.5 km,Road stage",
            "19,22 July,Modane to Alpe d'Huez,109.5 km,Road stage",
            "20,23 July,Grenoble,42.5 km,Individual time trial",
            "21,24 July,Creteil to Paris,95.0 km,Road stage",
        ];
        let edition = tour_de_france_2011();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2012() {
        let route = [
            "1,30 June,Liège (Belgium),6.4 km,Road stage",
            "2,1 July,Liège (Belgium) to Seraing (Belgium),198.0 km,Road stage",
            "3,2 July,Vise (Belgium) to Tournai (Belgium),198.0 km,Road stage",
            "4,3 July,Orchies to Boulogne-sur-Mer,197.0 km,Road stage",
            "5,4 July,Abbeville to Rouen,214.5 km,Road stage",
            "6,5 July,Rouen to Saint-Quentin,196.5 km,Road stage",
            "7,6 July,Epernay to Metz,207.5 km,Road stage",
            "8,7 July,Tomblaine to La Planche des Belles Filles,199.0 km,Road stage",
            "9,8 July,Belfort to Porrentruy,157.5 km,Road stage",
            "10,9 July,Arc-et-Senans to Besançon,41.5 km,Individual time trial",
            ",10 July,Rest day,Macon",
            "11,11 July,Macon to Bellegarde-sue-Valserine,194.5 km,Road stage",
            "12,12 July,Albertville to La Toussuire - Les Sybelles,148.0 km,Road stage",
            "13,13 July,Saint-Jean-de-Maurienne to Annonay-Davezieux,226.0 km,Road stage",
            "14,14 July,Saint-Paul-Trois-Chateaux to Cap d'Agde,217.0 km,Road stage",
            "15,15 July,Limoux to Foix,191.0 km,Road stage",
            "16,16 July,Samatan to Pau,158.5 km,Road stage",
            ",17 July,Rest day,Pau",
            "17,18 July,Pau to Bagnères-de-Luchon,197.0 km,Road stage",
            "18,19 July,Bagnères-de-Luchon to Peyragudes,143.5 km,Road stage",
            "19,20 July,Blagnac to Brive-la-Gaillarde,222.5 km,Road stage",
            "20,21 July,Bonneval to Chartres,53.5 km,Individual time trial",
            "21,22 July,Rambouillet to Paris,120.0 km,Road stage",
        ];
        let edition = tour_de_france_2012();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2013() {
        let route = [
            "1,29 June,Porto-Vecchio to Bastia,213.0 km,Road stage",
            "2,30 June,Bastia to Ajaccio,156.0 km,Road stage",
            "3,1 July,Ajaccio to Calvi,145.5 km,Road stage",
            "4,2 July,Nice,25.0 km,Team time trial",
            "5,3 July,Cagnes-sur-Mer to Marseille,228.5 km,Road stage",
            "6,4 July,Aix-en-Provence to Montpellier,176.5 km,Road stage",
            "7,5 July,Montpellier to Albi,205.5 km,Road stage",
            "8,6 July,Castres to Ax 3 Domaines,195.0 km,Road stage",
            "9,7 July,Saint-Girons to Bagnères-de-Bigorre,168.5 km,Road stage",
            ",8 July,Rest day,Saint-Nazaire",
            "10,9 July,Saint-Gildas-des-Bois to Saint-Malo,197.0 km,Road stage",
            "11,10 July,Avranches to Mont Saint-Michel,33.0 km,Individual time trial",
            "12,11 July,Fougeres to Tours,218.0 km,Road stage",
            "13,12 July,Tours to Saint-Amand-Montrond,173.0 km,Road stage",
            "14,13 July,Saint-Pourcain-sur-Sioule to Lyon,191.0 km,Road stage",
            "15,14 July,Givors to Mont Ventoux,242.5 km,Road stage",
            ",15 July,Rest day,Vaucluse",
            "16,16 July,Vaison-la-Romaine to Gap,168.0 km,Road stage",
            "17,17 July,Embrun to Chorges,32.0 km,Individual time trial",
            "18,18 July,Gap to Alpe d'Huez,172.5 km,Road stage",
            "19,19 July,Le Bourg-d'Oisans to Le Grand Bornand,204.5 km,Road stage",
            "20,20 July,Annecy to Semnoz,125.0 km,Road stage",
            "21,21 July,Versailles to Paris,133.5 km,Road stage",
        ];
        let edition = tour_de_france_2013();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2014() {
        let route = [
            "1,5 July,Leeds (United Kingdom) to Harrogate (United Kingdom),190.5 km,Road stage",
            "2,6 July,York (United Kingdom) to Sheffield (United Kingdom),201.0 km,Road stage",
            "3,7 July,Cambridge (United Kingdom) to London (United Kingdom),155.0 km,Road stage",
            "4,8 July,Le Touquet-Paris-Plage to Lille Metropole,163.5 km,Road stage",
            "5,9 July,Ypres (Belgium) to Arenberg Porte du Hainaut,152.5 km,Road stage",
            "6,10 July,Arras to Reims,194.0 km,Road stage",
            "7,11 July,Epernay to Nancy,234.5 km,Road stage",
            "8,12 July,Tomblaine to Gerardmer las Mauselaine,161.0 km,Road stage",
            "9,13 July,Gerardmer to Mulhouse,170.0 km,Road stage",
            "10,14 July,Mulhouse to La Planche des Belles Filles,161.5 km,Road stage",
            ",15 July,Rest day,Besançon",
            "11,16 July,Besançon to Oyonnax,187.5 km,Road stage",
            "12,17 July,Bourg-en-Bresse to Saint-Étienne,185.5 km,Road stage",
            "13,18 July,Saint-Étienne to Chamrousse,197.5 km,Road stage",
            "14,19 July,Grenoble to Risoul,177.0 km,Road stage",
            "15,20 July,Tallard to Nîmes,222.0 km,Road stage",
            ",21 July,Rest day,Carcassonne",
            "16,22 July,Carcassonne to Bagnères-de-Luchon,237.5 km,Road stage",
            "17,23 July,Saint-Gaudens to Saint-Lary Pla d'Adet,124.5 km,Road stage",
            "18,24 July,Pau to Hautacam,145.5 km,Road stage",
            "19,25 July,Maubourguet Pays du Val d'Adour to Bergerac,208.5 km,Road stage",
            "20,26 July,Bergerac to Périgueux,54.0 km,Individual time trial",
            "21,27 July,Evry to Paris,137.5 km,Road stage",
        ];
        let edition = tour_de_france_2014();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2015() {
        let route = [
            "1,4 July,Utrecht (Netherlands),13.8 km,Individual time trial",
            "2,5 July,Utrecht (Netherlands) to Zeeland (Netherlands),166.0 km,Road stage",
            "3,6 July,Antwerp (Belgium) to Huy (Belgium),159.5 km,Road stage",
            "4,7 July,Seraing (Belgium) to Cambrai,223.5 km,Road stage",
            "5,8 July,Arras to Amiens,189.5 km,Road stage",
            "6,9 July,Abbeville to Le Havre,191.5 km,Road stage",
            "7,10 July,Livarot to Fougeres,190.5 km,Road stage",
            "8,11 July,Rennes to Mûr-de-Bretagne,181.5 km,Road stage",
            "9,12 July,Vannes to Plumelec,28.0 km,Team time trial",
            ",13 July,Rest day,Pau",
            "10,14 July,Tarbes to La Pierre Saint-Martin,167.0 km,Road stage",
            "11,15 July,Pau to Cauterets,188.0 km,Road stage",
            "12,16 July,Lannemezan to Plateau de Beille,195.0 km,Road stage",
            "13,17 July,Muret to Rodez,198.5 km,Road stage",
            "14,18 July,Rodez to Mende,178.5 km,Road stage",
            "15,19 July,Mende to Valence,183.0 km,Road stage",
            "16,20 July,Bourg-de-Peage to Gap,201.0 km,Road stage",
            ",21 July,Rest day,Gap",
            "17,22 July,Digne-les-Bains to Pra-Loup,161.0 km,Road stage",
            "18,23 July,Gap to Saint-Jean-de-Maurienne,186.5 km,Road stage",
            "19,24 July,Saint-Jean-de-Maurienne to La Toussuire - Les Sybelles,138.0 km,Road stage",
            "20,25 July,Modane to Alpe d'Huez,110.5 km,Road stage",
            "21,26 July,Sevres to Paris,109.5 km,Road stage",
        ];
        let edition = tour_de_france_2015();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2016() {
        let route = [
            "1,2 July,Mont Saint-Michel to Utah Beach (Sainte-Marie-du-Mont),188.0 km,Road stage",
            "2,3 July,Saint-Lo to Cherbourg-en-Cotentin,183.0 km,Road stage",
            "3,4 July,Granville to Angers,223.5 km,Road stage",
            "4,5 July,Saumur to Limoges,237.5 km,Road stage",
            "5,6 July,Limoges to Le Lioran,216.0 km,Road stage",
            "6,7 July,Arpajon-sur-Cere to Montauban,190.5 km,Road stage",
            "7,8 July,L'Isle-Jourdain to Lac de Payolle,162.5 km,Road stage",
            "8,9 July,Pau to Bagnères-de-Luchon,184.0 km,Road stage",
            "9,10 July,Vielha Val d'Aran (Spain) to Andorra-Arcalis (Andorra),184.5 km,Road stage",
            ",11 July,Rest day,Andorra-Arcalis (Andorra)",
            "10,12 July,Escaldes-Engordany (Andorra) to Revel,197.0 km,Road stage",
            "11,13 July,Carcassonne to Montpellier,162.5 km,Road stage",
            "12,14 July,Montpellier to Chalet Reynard (Mont Ventoux),178.0 km,Road stage",
            "13,15 July,Bourg-Saint-Andeol to La Caverne du Pont-d'Arc,37.5 km,Individual time trial",
            "14,16 July,Montelimar to Villar-les-Dombes (Parc des Oiseaux),208.5 km,Road stage",
            "15,17 July,Bourg-en-Bresse to Culoz,160.0 km,Road stage",
            "16,18 July,Moirans-en-Montagne to Bern (Switzerland),209.0 km,Road stage",
            ",19 July,Rest day,Bern (Switzerland)",
            "17,20 July,Bern (Switzerland) to Finhaut-Emosson (Switzerland),184.5 km,Road stage",
            "18,21 July,Sallanches to Megeve,17.0 km,Individual time trial",
            "19,22 July,Abbeville to Saint Gervais-les-Bains,146.0 km,Road stage",
            "20,23 July,Megeve to Morzine,146.5 km,Road stage",
            "21,24 July,Chantilly to Paris,113.0 km,Road stage",
        ];
        let edition = tour_de_france_2016();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2017() {
        let route = [
            "1,1 July,Dusseldorf (Germany),14.0 km,Individual time trial",
            "2,2 July,Dusseldorf (Germany) to Liège (Belgium),203.5 km,Road stage",
            "3,3 July,Verviers (Belgium) to Longwy,212.5 km,Road stage",
            "4,4 July,Mondorf-les-Bains (Luxembourg) to Vittel,207.5 km,Road stage",
            "5,5 July,Vittel to La Planche des Belles Filles,160.5 km,Road stage",
            "6,6 July,Vesoul to Troyes,216.0 km,Road stage",
            "7,7 July,Troyes to Nuits-Saint-Georges,213.5 km,Road stage",
            "8,8 July,Dole to Station des Rousses,187.5 km,Road stage",
            "9,9 July,Nantua to Chambery,181.5 km,Road stage",
            ",10 July,Rest day,Dordogne",
            "10,11 July,Périgueux to Bergerac,178.0 km,Road stage",
            "11,12 July,Eymet to Pau,203.5 km,Road stage",
            "12,13 July,Pau to Peyragudes,214.5 km,Road stage",
            "13,14 July,Saint-Girons to Foix,101.0 km,Road stage",
            "14,15 July,Blagnac to Rodez,181.5 km,Road stage",
            "15,16 July,Laissac-Severac-l'Eglise to Le Puy-en-Velay,189.5 km,Road stage",
            ",17 July,Rest day,Le Puy-en-Velay",
            "16,18 July,Le Puy-en-Velay to Romans-sur-Isere,165.0 km,Road stage",
            "17,19 July,La Mure to Serre Chevalier,183.0 km,Road stage",
            "18,20 July,Briançon to Col d'Izoard,179.5 km,Road stage",
            "19,21 July,Embrun to Salon-de-Provence,222.5 km,Road stage",
            "20,22 July,Marseille,22.5 km,Individual time trial",
            "21,23 July,Montgeron to Paris,103.0 km,Road stage",
        ];
        let edition = tour_de_france_2017();
        assert_eq!(edition.route(), route);
        assert_eq!(edition.stage_summary(), "21 stages");
    }

    #[test]
    fn test_tour_de_france_2018() {
        let route = [
            "1,7 July,Noirmoutier-en-l'Île to Fontenay-le-Comte,201.0 km,Road stage",
            "2,8 July,Mouilleron-Saint-Germain to La Roche-sur-Yon,182.5 km,Road stage",
            "3,9 July,Cholet,35.5 km,Team time trial",
            "4,10 July,La Baule to Sarzeau,195.0 km,Road stage",
            "5,11 July,Lorient to Quimper,204.5 km,Road stage",
            "6,12 July,Brest to Mûr-de-Bretagne,181.0 km,Road stage",
            "7,13 July,Fougres to Chartres,231.0 km,Road stage",
            "8,14 July,Dreux to Amiens,181.0 km,Road stage",
            "9,15 July,Arras to Roubaix,156.5 km,Road stage",
            ",16 July,Rest day,Annecy",
            "10,17 July,Annecy to Le Grand Bornand,158.5 km,Road stage",
            "11,18 July,Albertville to La Rosiere,108.5 km,Road stage",
            "12,19 July,Bourg-Saint-Maurice to Alpe d'Huez,175.5 km,Road stage",
            "13,20 July,Le Bourg-d'Oisans to Valence,169.5 km,Road stage",
            "14,21 July,Saint-Paul-Trois-Chateaux to Mende,188.0 km,Road stage",
            "15,22 July,Millau to Carcassonne,181.5 km,Road stage",
            ",23 July,Rest day,Lourdes",
            "16,24 July,Carcassonne to Bagnères-de-Luchon,218.0 km,Road stage",
            "17,25 July,Bagnères-de-Luchon to Saint-Lary-Soulan (Col de Portet),65.0 km,Road stage",
            "18,26 July,Trie-sur-Basie to Pau,171.0 km,Road stage",
            "19,27 July,Lourdes to Laruns,200.5 km,Road stage",
            "20,28 July,Saint-Pée-sur-Nivelle to Espelette,31.0 km,Individual time trial",
            "21,29 July,Houilles to Paris,116.0 km,Road stage",
        ];

        let edition = tour_de_france_2018();

        assert_eq!(edition.route(), route);
        assert_eq!(edition.summit_finishes(), 4);
        assert_eq!(edition.stage_summary(), "21 stages");
    }
}
