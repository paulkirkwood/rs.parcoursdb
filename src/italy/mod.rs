use crate::country::Country;
use crate::location::Location;

pub fn abano_terme() -> Location { italian_location("Abano Terme", None) }
pub fn abbadia_san_salvatore() -> Location { italian_location("Abbadia San Salvatore", None) }
pub fn abbazia() -> Location { italian_location("Abbazia", None) }
pub fn abbiategrasso() -> Location { italian_location("Abbiategrasso", None) }
pub fn abetone() -> Location { italian_location("Abetone", Some(1386)) }
pub fn acquappesa() -> Location { italian_location("Acquappesa", None) }
pub fn acqui_terme() -> Location { italian_location("Acqui Terme", None) }
pub fn agile() -> Location { italian_location("Agile", None) }
pub fn agordo() -> Location { italian_location("Agordo", None) }
pub fn agrigento() -> Location { italian_location("Agrigento", None) }
pub fn agropoli() -> Location { italian_location("Agropoli", None) }
pub fn alassio() -> Location { italian_location("Alassio", None) }
pub fn alba() -> Location { italian_location("Alba", None) }
pub fn alba_adriatica() -> Location { italian_location("Alba Adriatica", None) }
pub fn albenga() -> Location { italian_location("Albenga", None) }
pub fn alberobello() -> Location { italian_location("Alberobello", None) }
pub fn albese_con_cassano() -> Location { italian_location("Albese con Cassano", None) }
pub fn alessandria() -> Location { italian_location("Alessandria", None) }
pub fn alghero() -> Location { italian_location("Alghero", None) }
pub fn alleghe() -> Location { italian_location("Alleghe", None) }
pub fn alpago() -> Location { italian_location("Alpago", None) }
pub fn alpe_di_pampeago() -> Location { italian_location("Alpe di Pampeago", Some(1983)) }
pub fn alpe_di_siusi() -> Location { italian_location("Alpe di Siusi", Some(1844)) }
pub fn altopiano_del_montasio() -> Location { italian_location("Altopiano del Montasio", None) }
pub fn amalfi() -> Location { italian_location("Amalfi", None) }
pub fn amantea() -> Location { italian_location("Amantea", None) }
pub fn amatrice() -> Location { italian_location("Amatrice", None) }
pub fn anagi() -> Location { italian_location("Anagi", None) }
pub fn ancona() -> Location { italian_location("Ancona", None) }
pub fn andalo() -> Location { italian_location("Andalo", Some(1024)) }
pub fn annifo() -> Location { italian_location("Annifo", Some(895)) }
pub fn anterselva() -> Location { italian_location("Anterselva", Some(1635)) }
pub fn aosta() -> Location { italian_location("Aosta", None) }
pub fn aprica() -> Location { italian_location("Aprica", Some(1173)) }
pub fn arabba() -> Location { italian_location("Arabba", None) }
pub fn arco() -> Location { italian_location("Arco", None) }
pub fn arcore() -> Location { italian_location("Arcore", None) }
pub fn arenzano() -> Location { italian_location("Arenzano", None) }
pub fn arezzo() -> Location { italian_location("Arezzo", None) }
pub fn arona() -> Location { italian_location("Arona", None) }
pub fn arosio() -> Location { italian_location("Arosio", None) }
pub fn arta_terme() -> Location { italian_location("Arta Terme", None) }
pub fn arzignano() -> Location { italian_location("Arzignano", None) }
pub fn ascoli_piceno() -> Location { italian_location("Ascoli Piceno", None) }
pub fn asiago() -> Location { italian_location("Asiago", None) }
pub fn asolo() -> Location { italian_location("Asolo", None) }
pub fn assisi() -> Location { italian_location("Assisi", None) }
pub fn asti() -> Location { italian_location("Asti", None) }
pub fn aulla() -> Location { italian_location("Aulla", None) }
pub fn auronzo_di_cadore() -> Location { italian_location("Auronzo di Cadore", None) }
pub fn autodromo_enzo_e_dino_ferrari() -> Location { italian_location("Imola (Autodromo Enzo e Dino Ferrari)", None) }
pub fn avaglio() -> Location { italian_location("Avaglio", Some(738)) }
pub fn avellino() -> Location { italian_location("Avellino", None) }
pub fn aversa() -> Location { italian_location("Aversa", None) }
pub fn avezzano() -> Location { italian_location("Avezzano", None) }

pub fn bacoli() -> Location { italian_location("Bacoli", None) }
pub fn bagni_di_casciana_terme() -> Location { italian_location("Bagni di Casciana Terme", None) }
pub fn bagno_di_romagna() -> Location { italian_location("Bagno di Romagna", None) }
pub fn bagnoli_irpino() -> Location { italian_location("Bagnoli Irpino", None) }
pub fn baia_domizia() -> Location { italian_location("Baia Domizia", None) }
pub fn barbaresco() -> Location { italian_location("Barbaresco", None) }
pub fn barberino_di_mugello() -> Location { italian_location("Barberino di Mugello", None) }
pub fn bardolino() -> Location { italian_location("Bardolino", None) }
pub fn bardonecchia() -> Location { italian_location("Bardonecchia", None) }
pub fn bari() -> Location { italian_location("Bari", None) }
pub fn barletta() -> Location { italian_location("Barletta", None) }
pub fn barolo() -> Location { italian_location("Barolo", None) }
pub fn barumini() -> Location { italian_location("Barumini", None) }
pub fn barzio() -> Location { italian_location("Barzio", None) }
pub fn baselga_di_pine() -> Location { italian_location("Baselga di Pine", None) }
pub fn bassano_del_grappa() -> Location { italian_location("Bassano del Grappa", None) }
pub fn bellaria() -> Location { italian_location("Bellaria", None) }
pub fn belluno() -> Location { italian_location("Belluno", None) }
pub fn belluno_nevegal() -> Location { italian_location("Belluno Nevegal", None) }
pub fn belvedere_marittimo() -> Location { italian_location("Belvedere Marittimo", None) }
pub fn benevento() -> Location { italian_location("Benevento", None) }
pub fn bergamo() -> Location { italian_location("Bergamo", None) }
pub fn bezzecca() -> Location { italian_location("Bezzecca", None) }
pub fn biandronno() -> Location { italian_location("Biandronno", None) }
pub fn bibbiena() -> Location { italian_location("Bibbiena", None) }
pub fn bibione() -> Location { italian_location("Bibione", None) }
pub fn biella() -> Location { italian_location("Biella", None) }
pub fn binago() -> Location { italian_location("Binago", None) }
pub fn bitonto() -> Location { italian_location("Bitonto", None) }
pub fn boario_terme() -> Location { italian_location("Boario Terme", None) }
pub fn bologna() -> Location { italian_location("Bologna", None) }
pub fn bolsena() -> Location { italian_location("Bolsena", None) }
pub fn bolzano() -> Location { italian_location("Bolzano", None) }
pub fn bordighera() -> Location { italian_location("Bordighera", None) }
pub fn borgo() -> Location { italian_location("Borgo Val di Taro", None) }
pub fn borgo_a_mozzano() -> Location { italian_location("Borgo a Mozzano", None) }
pub fn borgo_san_dalmazzo() -> Location { italian_location("Borgo San Dalmazzo", None) }
pub fn borgo_val_di_taro() -> Location { italian_location("Borgo Val di Taro", None) }
pub fn borgo_valsugana() -> Location { italian_location("Borgo Valsugana", None) }
pub fn borgomanero() -> Location { italian_location("Borgomanero", None) }
pub fn bormio() -> Location { italian_location("Bormio", Some(1225)) }
pub fn bormio2000() -> Location { italian_location("Bormio 2000", None) }
pub fn borno() -> Location { italian_location("Borno", None) }
pub fn bosa() -> Location { italian_location("Bosa", None) }
pub fn bosco_chiesanuova() -> Location { italian_location("Bosco Chiesanuova", None) }
pub fn bra() -> Location { italian_location("Bra", None) }
pub fn brentonico() -> Location { italian_location("Brentonico", None) }
pub fn brescia() -> Location { italian_location("Brescia", None) }
pub fn brescia_sant_eufemia() -> Location { italian_location("Brescia Sant'Eufemia", None) }
pub fn bressanone() -> Location { italian_location("Bressanone", None) }
pub fn breuil_cervinia() -> Location { italian_location("Breuil-Cervinia", None) }
pub fn brindisi() -> Location { italian_location("Brindisi", None) }
pub fn brixen() -> Location { italian_location("Brixen", None) }
pub fn bruneck() -> Location { italian_location("Bruneck", None) }
pub fn bruzzolana() -> Location { italian_location("Bruzzolana", Some(523)) }
pub fn bucchianico() -> Location { italian_location("Bucchianico", None) }
pub fn busseto() -> Location { italian_location("Busseto", None) }
pub fn busto_arsizio() -> Location { italian_location("Busto Arsizio", None) }

pub fn cagliari() -> Location { italian_location("Cagliari", None) }
pub fn caldes_val_di_sole() -> Location { italian_location("Caldes-Val di Sole", None) }
pub fn caltagirone() -> Location { italian_location("Caltagirone", None) }
pub fn caltanissetta() -> Location { italian_location("Caltanissetta", None) }
pub fn camaiore() -> Location { italian_location("Camaiore", None) }
pub fn cambiago() -> Location { italian_location("Cambiago", None) }
pub fn camigliatello_silano() -> Location { italian_location("Camigliatello Silano", None) }
pub fn campi_bisenzio() -> Location { italian_location("Campi Bisenzio", None) }
pub fn campi_salentina() -> Location { italian_location("Campi Salentina", None) }
pub fn campione_d_italia() -> Location { italian_location("Campione d'Italia", None) }
pub fn campitello_matese() -> Location { italian_location("Campitello Matese", None) }
pub fn campo_dei_fiori() -> Location { italian_location("Campo dei Fiori", None) }
pub fn campobasso() -> Location { italian_location("Campobasso", None) }
pub fn campotenese() -> Location { italian_location("Campotenese", None) }
pub fn canazei() -> Location { italian_location("Canazei", None) }
pub fn canelli() -> Location { italian_location("Canelli", None) }
pub fn cannobio() -> Location { italian_location("Cannobio", None) }
pub fn cantu() -> Location { italian_location("Cantu", None) }
pub fn capannori() -> Location { italian_location("Capannori", None) }
pub fn capo_d_orlando() -> Location { italian_location("Capo d'Orlando", None) }
pub fn caprera() -> Location { italian_location("Caprera", None) }
pub fn capua() -> Location { italian_location("Capua", None) }
pub fn caravaggio() -> Location { italian_location("Caravaggio", None) }
pub fn carovigno() -> Location { italian_location("Carovigno", None) }
pub fn carpegna() -> Location { italian_location("Carpegna", None) }
pub fn carpi() -> Location { italian_location("Carpi", None) }
pub fn carrara() -> Location { italian_location("Carrara", None) }
pub fn casale_monferrato() -> Location { italian_location("Casale Monferrato", None) }
pub fn cascata_del_toce() -> Location { italian_location("Cascata del Toce", None) }
pub fn cascia() -> Location { italian_location("Cascia", None) }
pub fn casciana_terme() -> Location { italian_location("Casciana Terme", None) }
pub fn cascina() -> Location { italian_location("Cascina", None) }
pub fn caserta() -> Location { italian_location("Caserta", None) }
pub fn cassano_d_adda() -> Location { italian_location("Cassano d'Adda", None) }
pub fn cassino() -> Location { italian_location("Cassino", None) }
pub fn casteggio() -> Location { italian_location("Casteggio", None) }
pub fn castel_gandolfo() -> Location { italian_location("Castel Gandolfo", None) }
pub fn castelfidardo() -> Location { italian_location("Castelfidardo", None) }
pub fn castelfranco_veneto() -> Location { italian_location("Castelfranco Veneto", None) }
pub fn castellammare_di_stabia() -> Location { italian_location("Castellammare di Stabia", None) }
pub fn castellamonte() -> Location { italian_location("Castellamonte", None) }
pub fn castellana_grotte() -> Location { italian_location("Castellana Grotte", None) }
pub fn castellania() -> Location { italian_location("Castellania", None) }
pub fn castelnuouvo_di_val_di_cecina() -> Location { italian_location("Castelnuouvo di Val di Cecina", None) }
pub fn castelraimondo() -> Location { italian_location("Castelraimondo", None) }
pub fn castelrotto() -> Location { italian_location( "Castelrotto", None ) }
pub fn castigilione_della_pescaia() -> Location { italian_location("Castiglione della Pescaia", None) }
pub fn castiglione_del_lago() -> Location { italian_location("Castiglione del Lago", None) }
pub fn castiglione_della_pescaia() -> Location { italian_location("Castiglione della Pescaia", None) }
pub fn castrocaro_terme() -> Location { italian_location("Castrocaro Terme", None) }
pub fn castrovillari() -> Location { italian_location("Castrovillari", None) }
pub fn catania() -> Location { italian_location("Catania", None) }
pub fn catanzaro() -> Location { italian_location("Catanzaro", None) }
pub fn catanzaro_lido() -> Location { italian_location("Catanzaro Lido", None) }
pub fn catanzaro_lungomare() -> Location { italian_location("Catanzaro-Lungomare", None) }
pub fn cattolica() -> Location { italian_location("Cattolica", None) }
pub fn cava_de_tirreni() -> Location { italian_location("Cava de'Tirreni", None) }
pub fn cavalese() -> Location { italian_location("Cavalese", None) }
pub fn cave_del_predil() -> Location { italian_location("Cave del Predil", None) }
pub fn cave_di_carrara() -> Location { italian_location("Cave di Carrara", None) }
pub fn cecina() -> Location { italian_location("Cecina", None) }
pub fn cefalu() -> Location { italian_location("Cefalu", None) }
pub fn celano() -> Location { italian_location("Celano", None) }
pub fn celle_ligure() -> Location { italian_location("Celle Ligure", None) }
pub fn cellole() -> Location { italian_location("Cellole", None) }
pub fn cenate_sotto() -> Location { italian_location("Cenate Sotto", None) }
pub fn cento() -> Location { italian_location("Cento", None) }
pub fn cepagatti() -> Location { italian_location("Cepagatti", None) }
pub fn ceresole_reale() -> Location { italian_location("Ceresole Reale", Some(2247)) }
pub fn certosa_di_pavia() -> Location { italian_location("Certosa di Pavia", None) }
pub fn cervere() -> Location { italian_location("Cervere", None) }
pub fn cervia() -> Location { italian_location("Cervia", None) }
pub fn cervinia() -> Location { italian_location("Cervinia", Some(2001)) }
pub fn cesana_torinese() -> Location { italian_location("Cesana Torinese", None) }
pub fn cesano_maderno() -> Location { italian_location("Cesano Maderno", None) }
pub fn cesena() -> Location { italian_location("Cesena", None) }
pub fn cesenatico() -> Location { italian_location("Cesenatico", None) }
pub fn cevo() -> Location { italian_location("Cevo", Some(1054)) }
pub fn cherasco() -> Location { italian_location("Cherasco", None) }
pub fn chianti_classico_stage() -> Location { italian_location("Chianti Classico Stage", None) }
pub fn chianale() -> Location { italian_location("Chianale", None) }
pub fn chianciano() -> Location { italian_location("Chianciano", None) }
pub fn chianciano_terme() -> Location { italian_location("Chianciano Terme", None) }
pub fn chiavari() -> Location { italian_location("Chiavari", None) }
pub fn chiavenna() -> Location { italian_location("Chiavenna", None) }
pub fn chieri() -> Location { italian_location("Chieri", None) }
pub fn chiesa_in_valmalenco() -> Location { italian_location("Chiesa in Valmalenco", None) }
pub fn chieti() -> Location { italian_location("Chieti", None) }
pub fn chioggia() -> Location { italian_location("Chioggia", None) }
pub fn cima_grappa() -> Location { italian_location("Cima Grappa", None) }
pub fn circuito_del_mugello() -> Location { italian_location("Circuito del Mugello", None) }
pub fn circuito_di_lido_d_albaro() -> Location { italian_location("Circuito di Lido d'Albaro", None) }
pub fn citta_di_castello() -> Location { italian_location("Citta di Castello", None) }
pub fn citta_sant_angelo() -> Location { italian_location("Citta Sant'Angelo", None) }
pub fn cittadella() -> Location { italian_location("Cittadella", None) }
pub fn cittareale() -> Location { italian_location("Cittareale", None) }
pub fn cividale_del_fruili() -> Location { italian_location("Cividale del Friuli", None) }
pub fn civitanova_marche() -> Location { italian_location("Civitanova Marche", None) }
pub fn civitavecchia() -> Location { italian_location("Civitavecchia", None) }
pub fn civitella_di_val_di_chiana() -> Location { italian_location("Civitella di Val di Chiana", None) }
pub fn cles() -> Location { italian_location("Cles", None) }
pub fn clusone() -> Location { italian_location("Clusone", None) }
pub fn col_du_galibier_valloire() -> Location { italian_location("Col du Galibier Valloire", None) }
pub fn collecchio() -> Location { italian_location("Collecchio", None) }
pub fn colle_don_bosco() -> Location { italian_location("Colle Don Bosco", None) }
pub fn colli_di_san_fermo() -> Location { italian_location("Colli di San Fermo", None) }
pub fn collina_di_superga() -> Location { italian_location("Collina di Superga", None) }
pub fn comacchio() -> Location { italian_location("Comacchio", None) }
pub fn comerio() -> Location { italian_location("Comerio", None) }
pub fn commezzadura() -> Location { italian_location("Commezzadura", None) }
pub fn como() -> Location { italian_location("Como", None) }
pub fn conegliano() -> Location { italian_location("Conegliano", None) }
pub fn contursi_terme() -> Location { italian_location("Contursi Terme", None) }
pub fn copertino() -> Location { italian_location("Copertino", None) }
pub fn cordenons() -> Location { italian_location("Cordenons", None) }
pub fn corinaldo() -> Location { italian_location("Corinaldo", None) }
pub fn corno_alle_scale() -> Location { italian_location("Corno alle Scale", None) }
pub fn cortina_d_ampezzo() -> Location { italian_location("Cortina d'Ampezzo", Some(1224)) }
pub fn cortona() -> Location { italian_location("Cortona", None) }
pub fn corvara() -> Location { italian_location("Corvara", None) }
pub fn cosenza() -> Location { italian_location("Cosenza", None) }
pub fn costalissoio() -> Location { italian_location("Costalissoio", Some(1300)) }
pub fn courmayeur() -> Location { italian_location("Courmayeur", None) }
pub fn courmayeur_skyway_monte_bianco() -> Location { italian_location("Courmayeur (Skyway Monte Bianco)", None) }
pub fn cremona() -> Location { italian_location("Cremona", None) }
pub fn crotone() -> Location { italian_location("Crotone", None) }
pub fn cuneo() -> Location { italian_location("Cuneo", None) }

pub fn dalmine() -> Location { italian_location("Dalmine", None) }
pub fn darfo_boario_terme() -> Location { italian_location("Darfo Boario Terme", None) }
pub fn desenzano_del_garda() -> Location { italian_location("Desenzano del Garda", None) }
pub fn diamante() -> Location { italian_location("Diamante", None) }
pub fn diano_marina() -> Location { italian_location("Diano Marina", None) }
pub fn dimaro() -> Location { italian_location("Dimaro", None) }
pub fn dobbiaco() -> Location { italian_location("Dobbiaco", None) }
pub fn domodossola() -> Location { italian_location("Domodossola", None) }
pub fn donoratico() -> Location { italian_location("Donoratico", None) }
pub fn dozza() -> Location { italian_location("Dozza", None) }

pub fn edolo() -> Location { italian_location("Edolo", None) }
pub fn empoli() -> Location { italian_location("Empoli", None) }
pub fn erba() -> Location { italian_location("Erba", None) }
pub fn erbusco() -> Location { italian_location("Erbusco", None) }
pub fn ercolano() -> Location { italian_location("Ercolano", None) }
pub fn erto_e_casso() -> Location { italian_location("Erto e Casso", None) }
pub fn esanatoglia() -> Location { italian_location("Esanatoglia", None) }
pub fn etna() -> Location { italian_location("Etna", None) }

pub fn fabriano() -> Location { italian_location("Fabriano", None) }
pub fn faenza() -> Location { italian_location("Faenza", None) }
pub fn falcade() -> Location { italian_location("Falcade", None) }
pub fn fano() -> Location { italian_location("Fano", None) }
pub fn felino() -> Location { italian_location("Felino", None) }
pub fn feltre() -> Location { italian_location("Feltre", None) }
pub fn fermo() -> Location { italian_location("Fermo", None) }
pub fn ferrara() -> Location { italian_location("Ferrara", None) }
pub fn fidenza() -> Location { italian_location("Fidenza", None) }
pub fn fiera_di_primiero() -> Location { italian_location("Fiera di Primiero", None) }
pub fn fiorano_modenese() -> Location { italian_location("Fiorano Modenese", None) }
pub fn firenze() -> Location { italian_location("Firenze", None) }
pub fn firenze_ponte_a_ema() -> Location { italian_location("Firenze (Ponte a Ema)", None) }
pub fn fiuggi() -> Location { italian_location("Fiuggi", None) }
pub fn florence() -> Location { italian_location("Florence", None) }
pub fn foggia() -> Location { italian_location("Foggia", None) }
pub fn folgaria() -> Location { italian_location("Folgaria", None) }
pub fn folgarida() -> Location { italian_location("Folgarida", None) }
pub fn foligno() -> Location { italian_location("Foligno", None) }
pub fn follonica() -> Location { italian_location("Follonica", None) }
pub fn fondo_sarnonico() -> Location { italian_location("Fondo/Sarnonico", None) }
pub fn foppolo() -> Location { italian_location("Foppolo", None) }
pub fn forio() -> Location { italian_location("Forio", None) }
pub fn forli() -> Location { italian_location("Forlì", None) }
pub fn formia() -> Location { italian_location("Formia", None) }
pub fn forte_delia_creta() -> Location { italian_location("Forte Delia Creta", Some(1254)) }
pub fn forte_dei_marmi() -> Location { italian_location("Forte dei Marmi", None) }
pub fn fossacesia() -> Location { italian_location("Fossacesia", None) }
pub fn fossano() -> Location { italian_location("Fossano", None) }
pub fn frabosa_soprana() -> Location { italian_location("Frabosa Soprana", None) }
pub fn francavilla_al_mare() -> Location { italian_location("Francavilla al Mare", None) }
pub fn frascati() -> Location { italian_location("Frascati", None) }
pub fn frosinone() -> Location { italian_location("Frosinone", None) }
pub fn fucecchio() -> Location { italian_location("Fucecchio", None) }

pub fn gabicce_mare() -> Location { italian_location("Gabicce Mare", None) }
pub fn gaeta() -> Location { italian_location("Gaeta", None) }
pub fn gallarate() -> Location { italian_location("Gallarate", None) }
pub fn garda() -> Location { italian_location("Garda", None) }
pub fn gardeccia_val_di_fassa() -> Location { italian_location("Gardeccia-Val di Fassa", None) }
pub fn gardone_riviera() -> Location { italian_location("Gardone Riviera", None) }
pub fn gatteo_a_mare() -> Location { italian_location("Gatteo a Mare", None) }
pub fn gemona_del_friuli() -> Location { italian_location("Gemona del Friuli", None) }
pub fn genoa() -> Location { italian_location("Genoa", None) }
pub fn giffoni_valle_piana() -> Location { italian_location("Giffoni Valle Piana", None) }
pub fn giovinazzo() -> Location { italian_location("Giovinazzo", None) }
pub fn giulianova() -> Location { italian_location("Giulianova", None) }
pub fn gorizia() -> Location { italian_location("Gorizia", None) }
pub fn gradisca_d_isonzo() -> Location { italian_location("Gradisca d'Isonzo", None) }
pub fn grado() -> Location { italian_location("Grado", None) }
pub fn gran_sasso_d_italia() -> Location { italian_location("Gran Sasso d'Italia", None) }
pub fn gran_sasso_d_italia_campo_imperatore() -> Location { italian_location("Gran Sasso d'Italia (Campo Imperatore)", None) }
pub fn gravellona_toce() -> Location { italian_location("Gravellona Toce", None) }
pub fn gressoney_saint_jean() -> Location { italian_location("Gressoney-Saint-Jean", None) }
pub fn grinzane_cavour() -> Location { italian_location("Grinzane Cavour", None) }
pub fn grosseto() -> Location { italian_location("Grosseto", None) }
pub fn gualdo_tadino() -> Location { italian_location("Gualdo Tadino", None) }
pub fn guardiagrele() -> Location { italian_location("Guardiagrele", None) }
pub fn gubbio() -> Location { italian_location("Gubbio", None) }
pub fn guilianova() -> Location { italian_location("Guilianova", None) }

pub fn igea_marina() -> Location { italian_location("Igea Marina", None) }
pub fn il_ciocco() -> Location { italian_location("Il Ciocco", None) }
pub fn imola() -> Location { italian_location("Imola", None) }
pub fn imola_autodromo_enzo_e_dino_ferrari() -> Location { italian_location("Imola (Autodromo Enzo e Dino Ferrari)", None) }
pub fn imperia() -> Location { italian_location("Imperia", None) }
pub fn indicatore() -> Location { italian_location("Indicatore", None) }
pub fn inverigo() -> Location { italian_location("Inverigo", None) }
pub fn ischia() -> Location { italian_location("Ischia", None) }
pub fn iseo() -> Location { italian_location("Iseo", None) }
pub fn isernia() -> Location { italian_location("Isernia", None) }
pub fn ivrea() -> Location { italian_location("Ivrea", None) }

pub fn jesi() -> Location { italian_location("Jesi", None) }
pub fn jesolo() -> Location { italian_location("Jesolo", None) }

pub fn l_aquila() -> Location { italian_location("L'Aquila", None) }
pub fn la_maddalena() -> Location { italian_location("La Maddalena", None) }
pub fn la_spezia() -> Location { italian_location("La Spezia", None) }
pub fn la_thuile() -> Location { italian_location("La Thuile", None) }
pub fn laga_laceno() -> Location { italian_location("Laga Laceno", None) }
pub fn lago_di_piediluco() -> Location { italian_location("Lago di Piediluco", None) }
pub fn lago_di_scanno() -> Location { italian_location("Lago di Scanno", None) }
pub fn lago_laceno() -> Location { italian_location("Lago Laceno", None) }
pub fn lago_miseno() -> Location { italian_location("Lago Miseno", None) }
pub fn lainate() -> Location { italian_location("Lainate", None) }
pub fn lamporecchio() -> Location { italian_location("Lamporecchio", None) }
pub fn lanciano() -> Location { italian_location("Lanciano", None) }
pub fn langhirano() -> Location { italian_location("Langhirano", None) }
pub fn larciano() -> Location { italian_location("Larciano", None) }
pub fn latina() -> Location { italian_location("Latina", None) }
pub fn lauria() -> Location { italian_location("Lauria", None) }
pub fn lavagna() -> Location { italian_location("Lavagna", None) }
pub fn lavarone() -> Location { italian_location("Lavarone", None) }
pub fn lecce() -> Location { italian_location("Lecce", None) }
pub fn lecco() -> Location { italian_location("Lecco", None) }
pub fn lecco_pian_dei_resinelli() -> Location { italian_location("Lecco-Pian dei Resinelli", None) }
pub fn legnago() -> Location { italian_location("Legnago", None) }
pub fn lerici() -> Location { italian_location("Lerici", None) }
pub fn levico_terme() -> Location { italian_location("Levico Terme", None) }
pub fn lido_d_albaro() -> Location { italian_location("Lido d'Albaro", None) }
pub fn lido_degli_estensi() -> Location { italian_location("Lido degli Estensi", None) }
pub fn lido_delle_nazioni() -> Location { italian_location("Lido delle Nazioni", None) }
pub fn lido_di_caldonazzo() -> Location { italian_location("Lido di Caldonazzo", None) }
pub fn lido_di_camaiore() -> Location { italian_location("Lido di Camaiore", None) }
pub fn lido_di_jesolo() -> Location { italian_location("Lido di Jesolo", None) }
pub fn lignano_sabbiadoro() -> Location { italian_location("Lignano Sabbiadoro", None) }
pub fn limone_piemonte() -> Location { italian_location("Limone Piemonte", None) }
pub fn limone_sul_garda() -> Location { italian_location("Limone sul Garda", None) }
pub fn lissone() -> Location { italian_location("Lissone", None) }
pub fn livigno() -> Location { italian_location("Livigno", Some(1816)) }
pub fn livorno() -> Location { italian_location("Livorno", None) }
pub fn loano() -> Location { italian_location("Loano", None) }
pub fn lodi() -> Location { italian_location("Lodi", None) }
pub fn lodrino() -> Location { italian_location("Lodrino", Some(736)) }
pub fn longarne() -> Location { italian_location("Longarne", None) }
pub fn longarone() -> Location { italian_location("Longarone", None) }
pub fn loreto() -> Location { italian_location("Loreto", None) }
pub fn loreto_aprutino() -> Location { italian_location("Loreto Aprutino", None) }
pub fn lovere() -> Location { italian_location("Lovere", None) }
pub fn lucca() -> Location { italian_location("Lucca", None) }
pub fn lucera() -> Location { italian_location("Lucera", None) }
pub fn lugo() -> Location { italian_location("Lugo", None) }
pub fn luino() -> Location { italian_location("Luino", None) }
pub fn lumezzane() -> Location { italian_location("Lumezzane", None) }

pub fn macerata() -> Location { italian_location("Macerata", None) }
pub fn macugnaga() -> Location { italian_location("Macugnaga", None) }
pub fn maddaloni() -> Location { italian_location("Maddaloni", None) }
pub fn madesimo() -> Location { italian_location("Madesimo", None) }
pub fn madonna_di_campiglio() -> Location { italian_location("Madonna di Campiglio", Some(1715)) }
pub fn madonna_di_san_luca() -> Location { italian_location("Madonna di San Luca", None) }
pub fn maielletta() -> Location { italian_location("Maielletta", None) }
pub fn malcesine() -> Location { italian_location("Malcesine", None) }
pub fn male() -> Location { italian_location("Male", None) }
pub fn maniago() -> Location { italian_location("Maniago", None) }
pub fn mantua() -> Location { italian_location("Mantua", None) }
pub fn maratea() -> Location { italian_location("Maratea", None) }
pub fn marcianise() -> Location { italian_location("Marcianise", None) }
pub fn marconia_di_pisticci() -> Location { italian_location("Marconia di Pisticci", None) }
pub fn mareo() -> Location { italian_location("Mareo", None) }
pub fn margherita_di_savoia() -> Location { italian_location("Margherita di Savoia", None) }
pub fn marina_di_ascea() -> Location { italian_location("Marina di Ascea", None) }
pub fn marina_di_carrara() -> Location { italian_location("Marina di Carrara", None) }
pub fn marina_di_grosseto() -> Location { italian_location("Marina di Grosseto", None) }
pub fn marina_di_massa() -> Location { italian_location("Marina di Massa", None) }
pub fn marina_di_pietrasanta() -> Location { italian_location("Marina di Pietrasanta", None) }
pub fn marina_di_pisa() -> Location { italian_location("Marina di Pisa", None) }
pub fn marina_di_ravenna() -> Location { italian_location("Marina di Ravenna", None) }
pub fn marina_di_san_vito() -> Location { italian_location("Marina di San Vito", None) }
pub fn marina_romea() -> Location { italian_location("Marina Romea", None) }
pub fn marino_di_massa() -> Location { italian_location("Marino di Massa", None) }
pub fn marmolada() -> Location { italian_location("Marmolada", None) }
pub fn marostica() -> Location { italian_location("Marostica", None) }
pub fn marotta() -> Location { italian_location("Marotta", None) }
pub fn marsala() -> Location { italian_location("Marsala", None) }
pub fn martell() -> Location { italian_location("Martell", None) }
pub fn massiccio_del_sirino() -> Location { italian_location("Massiccio del Sirino", None) }
pub fn matera() -> Location { italian_location("Matera", None) }
pub fn mazzin() -> Location { italian_location("Mazzin", None) }
pub fn meda() -> Location { italian_location("Meda", None) }
pub fn melfi() -> Location { italian_location("Melfi", None) }
pub fn mentana() -> Location { italian_location("Mentana", None) }
pub fn merano() -> Location { italian_location("Merano", None) }
pub fn merano2000() -> Location { italian_location("Merano 2000", None) }
pub fn mercogliano() -> Location { italian_location("Mercogliano", None) }
pub fn mergozzo() -> Location { italian_location("Mergozzo", None) }
pub fn messina() -> Location { italian_location("Messina", None) }
pub fn mestre() -> Location { italian_location("Mestre", None) }
pub fn metaponto() -> Location { italian_location("Metaponto", None) }
pub fn mezzocorona() -> Location { italian_location("Mezzocorona", None) }
pub fn mezzolombardo() -> Location { italian_location("Mezzolombardo", None) }
pub fn milano() -> Location { italian_location("Milano", None) }
pub fn milazzo() -> Location { italian_location("Milazzo", None) }
pub fn mira() -> Location { italian_location("Mira", None) }
pub fn mirandola() -> Location { italian_location("Mirandola", None) }
pub fn misurina() -> Location { italian_location("Misurina", None) }
pub fn modena() -> Location { italian_location("Modena", None) }
pub fn modica() -> Location { italian_location("Modica", None) }
pub fn modigiliana() -> Location { italian_location("Modigiliana", None) }
pub fn moena() -> Location { italian_location("Moena", None) }
pub fn mola_di_bari() -> Location { italian_location("Mola di Bari", None) }
pub fn molfetta() -> Location { italian_location("Molfetta", None) }
pub fn molveno() -> Location { italian_location("Molveno", None) }
pub fn mondolfo() -> Location { italian_location("Mondolfo", None) }
pub fn mondovi() -> Location { italian_location("Mondovi", None) }
pub fn mondragone() -> Location { italian_location("Mondragone", None) }
pub fn monesi() -> Location { italian_location("Monesi", None) }
pub fn montalcino() -> Location { italian_location("Montalcino", None) }
pub fn montalto_di_castro() -> Location { italian_location("Montalto di Castro", None) }
pub fn monte_argentario() -> Location { italian_location("Monte Argentario", None) }
pub fn monte_di_procida() -> Location { italian_location("Monte di Procida", None) }
pub fn monte_san_vicino() -> Location { italian_location("Monte San Vicino", None) }
pub fn montebelluna() -> Location { italian_location("Montebelluna", None) }
pub fn montecampione() -> Location { italian_location("Montecampione", Some(1200)) }
pub fn montecassino() -> Location { italian_location("Montecassino", None) }
pub fn montecatini() -> Location { italian_location("Montecatini", None) }
pub fn montecatini_terme() -> Location { italian_location("Montecatini Terme", None) }
pub fn montecchio_maggiore() -> Location { italian_location("Montecchio Maggiore", None) }
pub fn montecopiolo() -> Location { italian_location("Montecopiolo", None) }
pub fn montefalco() -> Location { italian_location("Montefalco", None) }
pub fn montefiascone() -> Location { italian_location("Montefiascone", None) }
pub fn montelibretti() -> Location { italian_location("Montelibretti", None) }
pub fn montevarchi() -> Location { italian_location("Montevarchi", None) }
pub fn montella() -> Location { italian_location("Montella", None) }
pub fn montello() -> Location { italian_location("Montello", Some(242)) }
pub fn monteluco_di_spoleto() -> Location { italian_location("Monteluco di Spoleto", None) }
pub fn montenero_di_bisaccia() -> Location { italian_location("Montenero di Bisaccia", None) }
pub fn montepulciano() -> Location { italian_location("Montepulciano", None) }
pub fn montesano_sulla_marcellana() -> Location { italian_location("Montesano sulla Marcellana", None) }
pub fn montesilvano() -> Location { italian_location("Montesilvano", None) }
pub fn monticello_brianza() -> Location { italian_location("Monticello Brianza", None) }
pub fn monza() -> Location { italian_location("Monza", None) }
pub fn morbegno() -> Location { italian_location("Morbegno", None) }
pub fn mori() -> Location { italian_location("Mori", None) }
pub fn muggio() -> Location { italian_location("Muggio", None) }
pub fn museo_del_ghisallo() -> Location { italian_location("Museo del Ghisallo", None) }

pub fn naples() -> Location { italian_location("Naples", None) }
pub fn narni_scalo() -> Location { italian_location("Narni Scalo", None) }
pub fn nervesa_della_battaglia() -> Location { italian_location("Nervesa della Battaglia", None) }
pub fn nettuno() -> Location { italian_location("Nettuno", None) }
pub fn neumarkt() -> Location { italian_location("Neumarkt", None) }
pub fn nevegal() -> Location { italian_location("Nevegal", None) }
pub fn nicotera() -> Location { italian_location("Nicotera", None) }
pub fn noale() -> Location { italian_location("Noale", None) }
pub fn nola() -> Location { italian_location("Nola", None) }
pub fn noto() -> Location { italian_location("Noto", None) }
pub fn novara() -> Location { italian_location("Novara", None) }
pub fn novi_ligure() -> Location { italian_location("Novi Ligure", None) }
pub fn numana() -> Location { italian_location("Numana", None) }

pub fn olbia() -> Location { italian_location("Olbia", None) }
pub fn oneglia() -> Location { italian_location("Oneglia", None) }
pub fn orbetello() -> Location { italian_location("Orbetello", None) }
pub fn oropa() -> Location { italian_location("Oropa", None) }
pub fn orta_san_giulio() -> Location { italian_location("Orta San Giulio", None) }
pub fn ortisei() -> Location { italian_location("Ortisei", None) }
pub fn ortisei_st_ulrich_val_gardena() -> Location { italian_location("Ortisei/St. Ulrich (Val Gardena)", None) }
pub fn ortona() -> Location { italian_location("Ortona", None) }
pub fn orvieto() -> Location { italian_location("Orvieto", None) }
pub fn osimo() -> Location { italian_location("Osimo", Some(265)) }
pub fn ostuni() -> Location { italian_location("Ostuni", None) }
pub fn ozegna() -> Location { italian_location("Ozegna", None) }

pub fn padua() -> Location { italian_location("Padua", None) }
pub fn padula() -> Location { italian_location("Padula", None) }
pub fn paestum() -> Location { italian_location("Paestum", None) }
pub fn palazzolo_sull_oglio() -> Location { italian_location("Palazzolo sull'Oglio", None) }
pub fn palermo() -> Location { italian_location("Palermo", None) }
pub fn palinuro() -> Location { italian_location("Palinuro", None) }
pub fn palmanova() -> Location { italian_location("Palmanova", None) }
pub fn palmi() -> Location { italian_location("Palmi", None) }
pub fn panicagliora_marliana() -> Location { italian_location("Panicagliora (Marliana)", None) }
pub fn paola() -> Location { italian_location("Paola", None) }
pub fn parabiago() -> Location { italian_location("Parabiago", None) }
pub fn parma() -> Location { italian_location("Parma", None) }
pub fn passo_cornello() -> Location { italian_location("Passo Cornello", Some(814)) }
pub fn passo_del_bocco() -> Location { italian_location("Passo del Bocco", None) }
pub fn passo_del_tonale() -> Location { italian_location("Passo del Tonale", None) }
pub fn passo_di_pampeagno() -> Location { italian_location("Passo di Pampeagno", None) }
pub fn passo_di_san_pellegrino() -> Location { italian_location("Passo di San Pellegrino", None) }
pub fn pavia() -> Location { italian_location("Pavia", None) }
pub fn pedara() -> Location { italian_location("Pedara", None) }
pub fn pedavena() -> Location { italian_location("Pedavena", None) }
pub fn peio() -> Location { italian_location("Peio", None) }
pub fn peio_terme() -> Location { italian_location("Peio Terme", None) }
pub fn penne() -> Location { italian_location("Penne", None) }
pub fn pergola() -> Location { italian_location("Pergola", None) }
pub fn perugia() -> Location { italian_location("Perugia", None) }
pub fn pesaro() -> Location { italian_location("Pesaro", None) }
pub fn pescara() -> Location { italian_location("Pescara", None) }
pub fn pescasseroli() -> Location { italian_location("Pescasseroli", None) }
pub fn peschici() -> Location { italian_location("Peschici", None) }
pub fn pesco_sannita() -> Location { italian_location("Pesco Sannita", None) }
pub fn pescocostanzo() -> Location { italian_location("Pescocostanzo", None) }
pub fn pfalzen() -> Location { italian_location("Pfalzen", None) }
pub fn piacenza() -> Location { italian_location("Piacenza", None) }
pub fn pian_del_re() -> Location { italian_location("Pian del Re", None) }
pub fn pian_del_resinelli() -> Location { italian_location("Pian del Resinelli", None) }
pub fn piancavallo() -> Location { italian_location("Piancavallo", None) }
pub fn piancogno() -> Location { italian_location("Piancogno", None) }
pub fn pienza() -> Location { italian_location("Pienza", None) }
pub fn pietra_ligure() -> Location { italian_location("Pietra Ligure", None) }
pub fn pietrasanta() -> Location { italian_location("Pietrasanta", None) }
pub fn pieve_di_cadore() -> Location { italian_location("Pieve di Cadore", None) }
pub fn pieve_di_cento() -> Location { italian_location("Pieve di Cento", None) }
pub fn pila() -> Location { italian_location("Pila", None) }
pub fn pinerolo() -> Location { italian_location("Pinerolo", None) }
pub fn pineta_di_cervia() -> Location { italian_location("Pineta di Cervia", None) }
pub fn pinzolo() -> Location { italian_location("Pinzolo", None) }
pub fn piombino() -> Location { italian_location("Piombino", None) }
pub fn pisa() -> Location { italian_location("Pisa", None) }
pub fn pistoia() -> Location { italian_location("Pistoia", None) }
pub fn pizzo() -> Location { italian_location("Pizzo", None) }
pub fn pizzo_calabro() -> Location { italian_location("Pizzo Calabro", None) }
pub fn plan_de_corones() -> Location { italian_location("Plan de Corones", Some(2273)) }
pub fn plan_di_montecampione() -> Location { italian_location("Plan di Montecampione", Some(1732)) }
pub fn poggibonsi() -> Location { italian_location("Poggibonsi", None) }
pub fn poggio() -> Location { italian_location("Poggio", None) }
pub fn poggio_di_san_remo() -> Location { italian_location("Poggio di San Remo", None) }
pub fn policastro_bussentino() -> Location { italian_location("Policastro Bussentino", None) }
pub fn policoro() -> Location { italian_location("Policoro", None) }
pub fn polla() -> Location { italian_location("Polla", None) }
pub fn polsa() -> Location { italian_location("Polsa", None) }
pub fn pomarance() -> Location { italian_location("Pomarance", None) }
pub fn pompeii() -> Location { italian_location("Pompeii", None) }
pub fn pont_saint_martin() -> Location { italian_location("Pont-Saint-Martin", None) }
pub fn ponte() -> Location { italian_location("Ponte", None) }
pub fn ponte_di_legno() -> Location { italian_location("Ponte di Legno", None) }
pub fn pontechianale() -> Location { italian_location("Pontechianale", None) }
pub fn pontedera() -> Location { italian_location("Pontedera", None) }
pub fn pontoglio() -> Location { italian_location("Pontoglio", None) }
pub fn pontremoli() -> Location { italian_location("Pontremoli", None) }
pub fn pordenone() -> Location { italian_location("Pordenone", None) }
pub fn porretta_terme() -> Location { italian_location("Porretta Terme", None) }
pub fn porteferraio() -> Location { italian_location("Porteferraio", None) }
pub fn porto_azzurro() -> Location { italian_location("Porto Azzurro", None) }
pub fn porto_recanati() -> Location { italian_location("Porto Recanati", None) }
pub fn porto_san_giorgio() -> Location { italian_location("Porto San Giorgio", None) }
pub fn porto_sant_elpidio() -> Location { italian_location("Porto Sant'Elpidio", None) }
pub fn portocivitanova() -> Location { italian_location("Portocivitanova", None) }
pub fn portoferraio() -> Location { italian_location("Portoferraio", None) }
pub fn portorose() -> Location { italian_location("Portorose", None) }
pub fn portovenere() -> Location { italian_location("Portovenere", None) }
pub fn potenza() -> Location { italian_location("Potenza", None) }
pub fn pozza_di_fassa() -> Location { italian_location("Pozza di Fassa", None) }
pub fn praia_a_mare() -> Location { italian_location("Praia a Mare", None) }
pub fn prati_di_tivo() -> Location { italian_location("Prati di Tivo", None) }
pub fn prato() -> Location { italian_location("Prato", None) }
pub fn prato_nevoso() -> Location { italian_location("Prato Nevoso", Some(1607)) }
pub fn predappio() -> Location { italian_location("Predappio", None) }
pub fn predazzo() -> Location { italian_location("Predazzo", None) }
pub fn presolana() -> Location { italian_location("Presolana", None) }

pub fn quarto_dei_mille() -> Location { italian_location("Quarto dei Mille", None) }

pub fn racconigi() -> Location { italian_location("Racconigi", None) }
pub fn rapallo() -> Location { italian_location("Rapallo", None) }
pub fn ravello() -> Location { italian_location("Ravello", None) }
pub fn ravenna() -> Location { italian_location("Ravenna", None) }
pub fn recanati() -> Location { italian_location("Recanati", None) }
pub fn recoaro_terme() -> Location { italian_location("Recoaro Terme", None) }
pub fn reggello() -> Location { italian_location("Reggello", None) }
pub fn reggio_calabria() -> Location { italian_location("Reggio Calabria", None) }
pub fn reggio_emilia() -> Location { italian_location("Reggio Emilia", None) }
pub fn riccione() -> Location { italian_location("Riccione", None) }
pub fn riese_pio_x() -> Location { italian_location("Riese Pio X", None) }
pub fn rieti() -> Location { italian_location("Rieti", None) }
pub fn rifugio_panarotta() -> Location { italian_location("Rifugio Panarotta", None) }
pub fn rimini() -> Location { italian_location("Rimini", None) }
pub fn rio_marina() -> Location { italian_location("Rio Marina", None) }
pub fn riolo_terme() -> Location { italian_location("Riolo Terme", None) }
pub fn riomaggiore() -> Location { italian_location("Riomaggiore", None) }
pub fn riva_del_garda() -> Location { italian_location("Riva del Garda", None) }
pub fn rivarolo_canavese() -> Location { italian_location("Rivarolo Canavese", None) }
pub fn rivisondoli() -> Location { italian_location("Rivisondoli", None) }
pub fn rocca_di_cambio() -> Location { italian_location("Rocca di Cambio", None) }
pub fn rocca_di_papa() -> Location { italian_location("Rocca di Papa", None) }
pub fn rocca_pietore() -> Location { italian_location("Rocca Pietore", None) }
pub fn roccaraso() -> Location { italian_location("Roccaraso", Some(1252)) }
pub fn roccaraso_aremogna() -> Location { italian_location("Roccaraso (Aremogna)", None) }
pub fn rodi_garganico() -> Location { italian_location("Rodi Garganico", None) }
pub fn rome() -> Location { italian_location("Rome", None) }
pub fn rossanto_veneto() -> Location { italian_location("Rossanto Veneto", None) }
pub fn rovato() -> Location { italian_location("Rovato", None) }
pub fn rovereto() -> Location { italian_location("Rovereto", None) }
pub fn rovetta() -> Location { italian_location("Rovetta", None) }
pub fn rovigo() -> Location { italian_location("Rovigo", None) }

pub fn sacro_monte_di_varese() -> Location { italian_location("Sacro Monte di Varese", None) }
pub fn saint_vincent() -> Location { italian_location("Saint-Vincent", None) }
pub fn sala_baganza() -> Location { italian_location("Sala Baganza", None) }
pub fn sala_consilina() -> Location { italian_location("Sala Consilina", None) }
pub fn salerno() -> Location { italian_location("Salerno", None) }
pub fn salice_terme() -> Location { italian_location("Salice Terme", None) }
pub fn salo() -> Location { italian_location("Salo", None) }
pub fn salsomaggiore_terme() -> Location { italian_location("Salsomaggiore Terme", None) }
pub fn saltara() -> Location { italian_location("Saltara", None) }
pub fn saluzzo() -> Location { italian_location("Saluzzo", None) }
pub fn sampeyre() -> Location { italian_location("Sampeyre", None) }
pub fn san_benedetto_del_tronto() -> Location { italian_location("San Benedetto del Tronto", None) }
pub fn san_candido() -> Location { italian_location("San Candido", None) }
pub fn san_dona_di_piave() -> Location { italian_location("San Dona di Piave", None) }
pub fn san_giacomo_di_roburent() -> Location { italian_location("San Giacomo di Roburent", None) }
pub fn san_giacomo_di_valle_castellana() -> Location { italian_location("San Giacomo di Valle Castellana", None) }
pub fn san_giorgio_del_sannio() -> Location { italian_location("San Giorgio del Sannio", None) }
pub fn san_giorgio_piacentino() -> Location { italian_location("San Giorgio Piacentino", None) }
pub fn san_giovanni_rotondo() -> Location { italian_location("San Giovanni Rotondo", None) }
pub fn san_lorenzo_al_mare() -> Location { italian_location("San Lorenzo al Mare", None) }
pub fn san_luca() -> Location { italian_location("San Luca", None) }
pub fn san_marcello_pistoiese() -> Location { italian_location("San Marcello Pistoiese", None) }
pub fn san_martino_di_castrozza() -> Location { italian_location("San Martino di Castrozza", Some(1487)) }
pub fn san_pellegrino_terme() -> Location { italian_location("San Pellegrino Terme", None) }
pub fn san_remo() -> Location { italian_location("San Remo", None) }
pub fn san_romolo() -> Location { italian_location("San Romolo", None) }
pub fn san_salvo() -> Location { italian_location("San Salvo", None) }
pub fn san_severo() -> Location { italian_location("San Severo", None) }
pub fn san_vendemiano() -> Location { italian_location("San Vendemiano", None) }
pub fn san_vigillo_di_marebbe() -> Location { italian_location("San Vigillo di Marebbe", None) }
pub fn san_vincenzo() -> Location { italian_location("San Vincenzo", None) }
pub fn san_vito_al_tagliamento() -> Location { italian_location("San Vito al Tagliamento", None) }
pub fn san_vito_di_cadore() -> Location { italian_location("San Vito di Cadore", None) }
pub fn sanctuario_di_vicoforte() -> Location { italian_location("Sanctuario di Vicoforte", None) }
pub fn sansepolcro() -> Location { italian_location("Sansepolcro", None) }
pub fn sant_anna_di_vinadio() -> Location { italian_location("Sant'Anna di Vinadio", None) }
pub fn santa_caterina_di_valfurva() -> Location { italian_location("Santa Caterina di Valfurva", None) }
pub fn santa_maria_capua_vetere() -> Location { italian_location("Santa Maria Capua Vetere", None) }
pub fn santa_margherita_ligure() -> Location { italian_location("Santa Margherita Ligure", None) }
pub fn santa_maria_del_cedro() -> Location { italian_location("Santa Maria del Cedro", None) }
pub fn santa_maria_della_versa() -> Location { italian_location("Santa Maria della Versa", None) }
pub fn santa_maria_di_sala() -> Location { italian_location("Santa Maria di Sala", None) }
pub fn santa_ninfa() -> Location { italian_location("Santa Ninfa", None) }
pub fn santarcangelo_di_romagna() -> Location { italian_location("Santarcangelo di Romagna", None) }
pub fn santuario_di_oropa() -> Location { italian_location("Santuario di Oropa", None) }
pub fn santuario_nostra_signora_della_guardia() -> Location { italian_location("Santuario Nostra Signora della Guardia", None) }
pub fn sanuario_di_vicoforte() -> Location { italian_location("Sanuario di Vicoforte", None) }
pub fn sappada() -> Location { italian_location("Sappada", None) }
pub fn sapri() -> Location { italian_location("Sapri", None) }
pub fn sarezzo() -> Location { italian_location("Sarezzo", None) }
pub fn sarnico() -> Location { italian_location("Sarnico", None) }
pub fn sarnonico() -> Location { italian_location("Sarnonico", None) }
pub fn saronno() -> Location { italian_location("Saronno", None) }
pub fn sarzana() -> Location { italian_location("Sarzana", None) }
pub fn sassari() -> Location { italian_location("Sassari", None) }
pub fn sassono() -> Location { italian_location("Sassono", None) }
pub fn sassuolo() -> Location { italian_location("Sassuolo", None) }
pub fn sauze_d_oulx() -> Location { italian_location("Sauze d'Oulx", None) }
pub fn savigliano() -> Location { italian_location("Savigliano", None) }
pub fn savona() -> Location { italian_location("Savona", None) }
pub fn scalea() -> Location { italian_location("Scalea", None) }
pub fn scalenghe() -> Location { italian_location("Scalenghe", None) }
pub fn scanno() -> Location { italian_location("Scanno", None) }
pub fn scarperia() -> Location { italian_location("Scarperia", None) }
pub fn schio() -> Location { italian_location("Schio", None) }
pub fn schlanders() -> Location { italian_location("Schlanders", None) }
pub fn schnals() -> Location { italian_location("Schnals", None) }
pub fn sciacca() -> Location { italian_location("Sciacca", None) }
pub fn scilla() -> Location { italian_location("Scilla", None) }
pub fn sella_valsugana() -> Location { italian_location("Sella Valsugana", None) }
pub fn selva() -> Location { italian_location("Selva", None) }
pub fn selva_di_fasano() -> Location { italian_location("Selva di Fasano", None) }
pub fn selva_di_val_gardena() -> Location { italian_location("Selva di Val Gardena", None) }
pub fn selvino() -> Location { italian_location("Selvino", None) }
pub fn senigallia() -> Location { italian_location("Senigallia", None) }
pub fn seravezza() -> Location { italian_location("Seravezza", None) }
pub fn seregno() -> Location { italian_location("Seregno", None) }
pub fn serniga_di_salo() -> Location { italian_location("Serniga di Salo", None) }
pub fn serra_san_bruno() -> Location { italian_location("Serra San Bruno", None) }
pub fn serraville_scrivia() -> Location { italian_location("Serraville Scrivia", None) }
pub fn sestola() -> Location { italian_location("Sestola", None) }
pub fn sestriere() -> Location { italian_location("Sestrière", Some(2035)) }
pub fn sestri_levante() -> Location { italian_location("Sestri Levante", None) }
pub fn siena() -> Location { italian_location("Siena", None) }
pub fn sillian() -> Location { italian_location("Sillian", None) }
pub fn silvi_marina() -> Location { italian_location("Silvi Marina", None) }
pub fn sinalunga() -> Location { italian_location("Sinalunga", None) }
pub fn siracusa() -> Location { italian_location("Siracusa", None) }
pub fn sirmione() -> Location { italian_location("Sirmione", None) }
pub fn soave() -> Location { italian_location("Soave", None) }
pub fn sondrio() -> Location { italian_location("Sondrio", None) }
pub fn sora() -> Location { italian_location("Sora", None) }
pub fn sorrento() -> Location { italian_location("Sorrento", None) }
pub fn sottomarina_di_chioggia() -> Location { italian_location("Sottomarina di Chioggia", None) }
pub fn spilimbergo() -> Location { italian_location("Spilimbergo", None) }
pub fn spoleto() -> Location { italian_location("Spoleto", None) }
pub fn spondigna() -> Location { italian_location("Spondigna", None) }
pub fn stradella() -> Location { italian_location("Stradella", None) }
pub fn sulden() -> Location { italian_location("Sulden", None) }
pub fn sulmona() -> Location { italian_location("Sulmona", None) }
pub fn susa() -> Location { italian_location("Susa", None) }

pub fn tabiano_terme() -> Location { italian_location("Tabiano Terme", None) }
pub fn taormina() -> Location { italian_location("Taormina", None) }
pub fn taranto() -> Location { italian_location("Taranto", None) }
pub fn tarvisio() -> Location { italian_location("Tarvisio", None) }
pub fn teano() -> Location { italian_location("Teano", None) }
pub fn telese_terme() -> Location { italian_location("Telese Terme", None) }
pub fn tempio_pausania() -> Location { italian_location("Tempio Pausania", None) }
pub fn teramo() -> Location { italian_location("Teramo", None) }
pub fn term_luigiane() -> Location { italian_location("Term Luigiane", None) }
pub fn termamo() -> Location { italian_location("Termamo", None) }
pub fn terme_di_comano() -> Location { italian_location("Terme di Comano", None) }
pub fn terme_euganee() -> Location { italian_location("Terme Euganee", None) }
pub fn terme_la_calda() -> Location { italian_location("Terme La Calda", None) }
pub fn terme_luigiane() -> Location { italian_location("Terme Luigiane", None) }
pub fn termoli() -> Location { italian_location("Termoli", None) }
pub fn terni() -> Location { italian_location("Terni", None) }
pub fn terracina() -> Location { italian_location("Terracina", None) }
pub fn tirano() -> Location { italian_location("Tirano", None) }
pub fn tivoli() -> Location { italian_location("Tivoli", None) }
pub fn todi() -> Location { italian_location("Todi", None) }
pub fn tolmezzo() -> Location { italian_location("Tolmezzo", None) }
pub fn torino() -> Location { italian_location("Torino", None) }
pub fn tortoli() -> Location { italian_location("Tortolì", None) }
pub fn tortona() -> Location { italian_location("Tortona", None) }
pub fn tortoreto() -> Location { italian_location("Tortoreto", None) }
pub fn tortoreto_lido() -> Location { italian_location("Tortoreto Lido", None) }
pub fn tramin() -> Location { italian_location("Tramin", None) }
pub fn trani() -> Location { italian_location("Trani", None) }
pub fn trento() -> Location { italian_location("Trento", None) }
pub fn trescore() -> Location { italian_location("Trescore Balneario", None) }
pub fn trescore_balneario() -> Location { italian_location("Trescore Balneario", None) }
pub fn treviglio() -> Location { italian_location("Treviglio", None) }
pub fn treviso() -> Location { italian_location("Treviso", None) }
pub fn trieste() -> Location { italian_location("Trieste", None) }
pub fn tropea() -> Location { italian_location("Tropea", None) }
pub fn turbigo() -> Location { italian_location("Turbigo", None) }
pub fn turin() -> Location { italian_location("Turin", None) }

pub fn udine() -> Location { italian_location("Udine", None) }
pub fn uliveto_terme() -> Location { italian_location("Uliveto Terme", None) }
pub fn urbania() -> Location { italian_location("Urbania", None) }
pub fn urbino() -> Location { italian_location("Urbino", None) }
pub fn urtijei() -> Location { italian_location("Urtijei", None) }

pub fn vajolet_towers() -> Location { italian_location("Vajolet Towers", None) }
pub fn val_martello() -> Location { italian_location("Val Martello", None) }
pub fn valdaora() -> Location { italian_location("Valdaora", None) }
pub fn valdengo() -> Location { italian_location("Valdengo", None) }
pub fn valdobbiadene() -> Location { italian_location("Valdobbiadene", None) }
pub fn valenza() -> Location { italian_location("Valenza", None) }
pub fn valico_del_vetriolo() -> Location { italian_location("Valico del Vetriolo", None) }
pub fn valico_di_pietra_rossa() -> Location { italian_location("Valico di Pietra Rossa", Some(674)) }
pub fn vallombrosa() -> Location { italian_location("Vallombrosa", None) }
pub fn valmontone() -> Location { italian_location("Valmontone", None) }
pub fn valnontey_di_cogne() -> Location { italian_location("Valnontey di Cogne", None) }
pub fn varazze() -> Location { italian_location("Varazze", None) }
pub fn varese() -> Location { italian_location("Varese", None) }
pub fn varzi() -> Location { italian_location("Varzi", None) }
pub fn vasto() -> Location { italian_location("Vasto", None) }
pub fn vedelago() -> Location { italian_location("Vedelago", None) }
pub fn veneria_reale() -> Location { italian_location("Veneria Reale", None) }
pub fn venezia() -> Location { italian_location("Venezia", None) }
pub fn venice() -> Location { italian_location("Venice", None) }
pub fn venice_lido() -> Location { italian_location("Lido (Venice)", None) }
pub fn verbania() -> Location { italian_location("Verbania", None) }
pub fn vercelli() -> Location { italian_location("Vercelli", None) }
pub fn verona() -> Location { italian_location("Verona", None) }
pub fn verres() -> Location { italian_location("Verres", None) }
pub fn vestone() -> Location { italian_location("Vestone", None) }
pub fn viareggio() -> Location { italian_location("Viareggio", None) }
pub fn vibo_valentia() -> Location { italian_location("Vibo Valentia", None) }
pub fn vicenza() -> Location { italian_location("Vicenza", None) }
pub fn vicenza_monte_berico() -> Location { italian_location("Vicenza (Monte Berico)", None) }
pub fn vieste() -> Location { italian_location("Vieste", None) }
pub fn vigevano() -> Location { italian_location("Vigevano", None) }
pub fn viggiano() -> Location { italian_location("Viggiano", None) }
pub fn vigo_di_fassa() -> Location { italian_location("Vigo di Fassa", None) }
pub fn villa_di_tirano() -> Location { italian_location("Villa di Tirano", None) }
pub fn villa_san_giovanni() -> Location { italian_location("Villa San Giovanni", None) }
pub fn villafranca() -> Location { italian_location("Villafranca", None) }
pub fn villafranca_tirrena() -> Location { italian_location("Villafranca Tirrena", None) }
pub fn villapiana_lido() -> Location { italian_location("Villapiana Lido", None) }
pub fn vinci() -> Location { italian_location("Vinci", None) }
pub fn viterbo() -> Location { italian_location("Viterbo", None) }
pub fn vittorio_veneto() -> Location { italian_location("Vittorio Veneto", None) }
pub fn voghera() -> Location { italian_location("Voghera", None) }

pub fn zingonia() -> Location { italian_location("Zingonia", None) }

fn italian_location(name: &'static str, elevation: Option<i32>) -> Location {
    Location::new(name.to_string(), Country::Italy, elevation)
}
