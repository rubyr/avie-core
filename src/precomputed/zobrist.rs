    pub static BLACK_PAWN: usize = 0;
    pub static BLACK_KNIGHT: usize = 1;
    pub static BLACK_BISHOP: usize = 2;
    pub static BLACK_ROOK: usize = 3;
    pub static BLACK_QUEEN: usize = 4;
    pub static BLACK_KING: usize = 5;
    pub static WHITE_PAWN: usize = 6;
    pub static WHITE_KNIGHT: usize = 7;
    pub static WHITE_BISHOP: usize = 8;
    pub static WHITE_ROOK: usize = 9;
    pub static WHITE_QUEEN: usize = 10;
    pub static WHITE_KING: usize = 11;
    pub static EN_PASSANT: usize = 6; // uses range 0-7 which will never be used for white pawns
    pub static BLACK_KING_CASTLE: [usize; 2] = [0, 56];
    pub static BLACK_QUEEN_CASTLE: [usize; 2] = [0, 57];
    pub static WHITE_KING_CASTLE: [usize; 2] = [0, 58];
    pub static WHITE_QUEEN_CASTLE: [usize; 2] = [0, 59];
    pub static ZOBRIST: [[u64; 64]; 12] = [
    [
        8636970095801407947,
        2929859699184349512,
        13407012695823196876,
        11305718228661632364,
        15382294020502184456,
        5093263269123337604,
        16068546149528232580,
        18404167277022713092,
        15509345540276157801,
        9346332256715928510,
        13314030811272092392,
        10855183777123477703,
        7025962242432887101,
        16909731669433181905,
        14116445011886234430,
        15113999568725535260,
        3780096179168925778,
        16607436119293071389,
        8860099959035760073,
        7487716906382124032,
        15328017481040093569,
        991954341064061923,
        5763522281376640281,
        11047750829857515465,
        8164411758874955622,
        3322816221871044358,
        14727444023311099094,
        13553386869004813427,
        5708808953779461922,
        8729223233963039765,
        8095235565932632721,
        17226986683394254107,
        2842699930029854019,
        13485928868124021195,
        12985602408459902010,
        3154041130116369415,
        7336409095335229966,
        13635584628298953312,
        3751911641606179585,
        10512920361205386723,
        10065449092898381473,
        5117494213265391141,
        9827643638261984803,
        3538600552867160889,
        9976104724474726829,
        2125855233051681994,
        16900918321649777074,
        2426113294557430702,
        13489708595174381025,
        5148578021594305129,
        2074177470199758810,
        17226745476921621555,
        2773368951418254785,
        1295979587590847720,
        18046537786519033356,
        2302685870564821857,
        14266240102715784018,
        6219493018363072678,
        9683475669456702524,
        13972995546080581130,
        16464237943907287761,
        16377034407591008737,
        3182633904072973292,
        233673638179181814,
    ],
    [
        9473084728740025790,
        3025445391618805033,
        8952856313903854416,
        13191627299246798919,
        2282337027067065147,
        5159957043577819284,
        3330840799646183693,
        7993435623763039409,
        14720153321285150435,
        14923299650777196325,
        14959403801940347508,
        14879424953683321156,
        15853690808292805775,
        4134473112198212621,
        9790008165719511748,
        1455518046179254745,
        12646367795506190138,
        1705884718418068884,
        3966915392127441075,
        12116873603079295790,
        15920040531684202294,
        11021206202235391169,
        9956988809198294385,
        8724968766031008558,
        6443443279525136529,
        9498145265847807693,
        14749805407176008159,
        1333799691445709520,
        13721070893694792649,
        14844549447638342425,
        3019703841898142332,
        6712323039395017616,
        11068241526347015819,
        7414767578375521722,
        14180894794265531168,
        7745091452179634820,
        14392049948048881997,
        3671982762554588905,
        14586754318378112123,
        8293268592455390809,
        18274565238171321848,
        9431018376752317130,
        16306311732621311056,
        12042846111588938249,
        15467546507961840308,
        14934177512053687529,
        13630429847971208943,
        3476533775290057308,
        15931982285233373355,
        1203672622800751374,
        8589101836754611146,
        16323854569456846207,
        13062855992194264597,
        14320246338496715238,
        5206851053981487277,
        1162828661802402932,
        12086632622933289534,
        4008768645464608033,
        8276554895530393068,
        17790986987454794206,
        15586508440688285964,
        17291054670101023884,
        1140002929011572022,
        10883563806951751617,
    ],
    [
        13377512622627354511,
        14813887351596260092,
        16570186167884810667,
        17036853425289611788,
        17142851241162997518,
        3842807045031685858,
        16417773934473575259,
        2344302260998126549,
        2131530813420226529,
        7698633242792514124,
        13734139611748554743,
        16659497526238059889,
        11724659401060244982,
        8503267984727187731,
        6529510817414217784,
        8382988184362550696,
        16351811833534043711,
        11589674251603086500,
        18207384874909177020,
        13209941593328053171,
        6597112658683807085,
        12548293633468871001,
        10827236906165131931,
        16535033968726884153,
        3699264550592598817,
        9359316935684716900,
        6617485788343133726,
        15443600569354012825,
        4816569463906123722,
        6672151747393481723,
        12387736742728154322,
        2622562007164360509,
        10090881679036973255,
        617612017040482081,
        13036436225383518051,
        5602548835848969919,
        8443558826098466443,
        7313930843564645486,
        13290159803782144443,
        4418908438175076030,
        6577126223191723118,
        6195578568315149172,
        13365904886417785940,
        16161750032807115603,
        16930596953672393059,
        16526323090708886967,
        12963446536429027565,
        14723407622999168558,
        18067528025893383608,
        642900014195697467,
        3727450690956739345,
        3601160018571959578,
        2441834744249844683,
        5208048346964668286,
        5253260334624506324,
        13150980951800261553,
        1266110334620015099,
        5806834831489155783,
        4191755891244942621,
        10426271489879361742,
        17250028022000052291,
        10016264514785415793,
        8812327069656472924,
        11331153041881017005,
    ],
    [
        15185425053754941542,
        15346396372749176760,
        1683938092956563818,
        9391719802790940244,
        3531396739771578550,
        15181993387858519934,
        18412656513387092231,
        10183513485541153750,
        10018048082581733620,
        16472116924999159777,
        4411494007305062872,
        14612124491184547773,
        15473449536445499449,
        14587408112241668716,
        3816694446276948695,
        7155458285308697455,
        13585231696562573418,
        5555843188241855081,
        11137948738919377088,
        17321099967898097620,
        12975814833144010450,
        5980184541917726907,
        9691791055794751953,
        9020894737999121281,
        7357421245223161795,
        2138552865101210297,
        13230562779917564078,
        12541616386634733709,
        4985279548511790013,
        1282390092563405512,
        13747218569788059211,
        15595966355992646407,
        13950006443295991112,
        13442381003921476161,
        8492583686216978554,
        15551731953030672475,
        1931302397665943405,
        3616510233646521713,
        3906328364955969437,
        2796215278243791884,
        10780290777153071627,
        7838954308981405254,
        14065262810210697357,
        11228647638848026703,
        10995076745835184828,
        8156575038471235755,
        10415048863127705947,
        17224639835734825035,
        6309844786386905182,
        17303516174152468410,
        17816907688940545779,
        12048920119021426251,
        7829868589361586195,
        2768437397628879835,
        13747803345482836562,
        1177089683895780270,
        8914231290145753915,
        8328042334141338815,
        1455368490443492321,
        14684101712077116097,
        11380815842873699237,
        5059821031323144472,
        16366359456253775244,
        4237053242828881904,
    ],
    [
        13155357159139154471,
        16996063202010187278,
        2357701405657698267,
        3863596714496195643,
        12910264141503864264,
        64574017339153197,
        2074650121906144271,
        9730737187560808517,
        6826931259441368150,
        7396026387531514780,
        8983809173377346346,
        5104550587161090908,
        3664571370317416665,
        2874201125967643518,
        12091736142125453302,
        7797333104603827724,
        801590023111071723,
        3371429249391208969,
        722287690627257865,
        12339889855498981244,
        6544732387142464665,
        18342027804855536688,
        15391839642612735217,
        605550056397386875,
        16267136468490428635,
        13965556051646742611,
        4077965467291721954,
        8102090145922237870,
        788179590181870535,
        17739946380474751813,
        6426913489831955082,
        2848397690142206371,
        2971631523531193310,
        16806622277112999111,
        13856186171440123700,
        12651646922489502022,
        7465110366194110265,
        2074147508320750442,
        17736314833501978102,
        7326246527628137752,
        908618887726979410,
        5764267958426205531,
        4522913135524633522,
        5285354417471190646,
        6541422805295364059,
        6600865112764320479,
        13091267592364911849,
        3283032387571828738,
        2376870838527901154,
        14892619515735806449,
        15329914806669619637,
        12320990574816881722,
        995575163480117317,
        16577818498402926025,
        18303273413194150107,
        377978390581411747,
        4161126901812860835,
        11623223206521152292,
        9403825852646842256,
        16526964072123434276,
        2217196501817954826,
        13738499946375637047,
        14159974096544633390,
        17311523263314845288,
    ],
    [
        1574451517713273854,
        5521011525187315086,
        15088672791878378885,
        15364803835130544712,
        6303295243763454871,
        13572373949809328676,
        16685833419211813316,
        9078057828886245745,
        13831383426959799018,
        11513497854632342099,
        16306528356313639984,
        16089644230372369426,
        13383851206046593926,
        16499340510717785318,
        16500834681136450979,
        2390339032665490489,
        17370482988207087896,
        3210095816367728215,
        14120390737013483178,
        7315584120202497479,
        16068249268265980356,
        15785543213783390724,
        9962622578754345071,
        14829985017507720534,
        13891191496862394202,
        15483388847176084847,
        11276531530237173524,
        14768114522301987585,
        16693440534720706262,
        13598513501465221976,
        11992034573429169946,
        6374562471226751820,
        7960456928127509915,
        10072814940823291879,
        792847633471682511,
        13172273028138252517,
        1528180411735259720,
        7620730651006406302,
        4868049293319356200,
        9300479361136591082,
        16169919042098121921,
        5471913616032533740,
        5294733659530966072,
        8277731749992884745,
        6941494343892923199,
        17691876657951109621,
        5680166949959405294,
        17714760925067221874,
        13853002060175701527,
        18408163875718178449,
        16023933700540595614,
        12496093509853272215,
        3548642396082078868,
        13057932201648011132,
        15302574190676070834,
        6009608407754954691,
        8642182081657759894,
        9526209971388699205,
        14744825997155353623,
        15279177959177843582,
        2644371061870648793,
        13454787080675700123,
        8886921606663456517,
        3263397814639873949,
    ],
    [
        9608005580459523282,
        1779545989292046320,
        13964326704667383464,
        9369322635486788841,
        12718284927189372145,
        3583953072074464831,
        3382512053636438851,
        17689929144361603133,
        18166795716221776868,
        14463677118360404357,
        4748094519812918584,
        16401688786574140307,
        6758013369420941307,
        12645496410005902725,
        7156642961136729858,
        13411995623400864394,
        5039790151071917712,
        4711522630463866331,
        18004944911015931199,
        753036840786029699,
        8018968530894028254,
        10912006002834381821,
        7661906160694939062,
        9442214637557898183,
        17945641799580073538,
        18295287339519628944,
        1504510082141230988,
        5438618173254838961,
        4541304357176512045,
        11166988697528388505,
        10962152837737411444,
        13202647781759578197,
        13566185738182666879,
        5822147232395273836,
        11524733834655795001,
        9811585577691045314,
        10435920862651649632,
        17436582276327970264,
        11763829047068445936,
        9263008812832361987,
        3344259172657074777,
        5080593461838290660,
        14498816171416165940,
        5584030906388510585,
        13448934891440415669,
        7246722926850434891,
        3017761509008583582,
        13832482892546944071,
        13227141385231408056,
        7589462577525076907,
        16878122174341389618,
        5990820541295811430,
        2445933700775633139,
        8748482116253127467,
        12511586224934441885,
        7383824205019299048,
        3168979622324487453,
        5440686157403856106,
        8746163335425936950,
        9955491334123169841,
        9450580438369886541,
        252014936901586730,
        9930415185214486250,
        9879417725414290123,
    ],
    [
        11759910398089444073,
        3633665924803618295,
        18154667609794341514,
        5251426505728689017,
        2453202848992544235,
        9413153433316476047,
        15980861174550261778,
        3248432253759742704,
        15899342239406455020,
        1585248966036200110,
        2930168682382826346,
        11452958732344587074,
        9246855361247217422,
        5517916113709730705,
        15011832259346827144,
        13382371926227844614,
        6080764645860543850,
        9102011691667278308,
        15111686038152471160,
        11873953187737057512,
        1515379881877114429,
        18287653480061291776,
        16151174764859780202,
        8819693933431273906,
        14131370419980091594,
        2874376202990324247,
        13122868896662910949,
        10234618042651360172,
        5674040523207714375,
        16571314763977264777,
        5288536857449408966,
        3847981106476593528,
        6142947668384298618,
        4158227436498378287,
        14638960969322570192,
        8443842561374556953,
        4330825875504674225,
        2660172313342832006,
        10038316749812807514,
        12927053372574991655,
        12837329677025136364,
        13934175750256656042,
        7525846579021996876,
        1471332322091615070,
        11878544598877867314,
        16156773789853552572,
        12111186688125732443,
        5267727899132328122,
        3213751246647322873,
        6388093945412633767,
        8584389708018990108,
        15259697445208799184,
        12322795072949237942,
        8320158267435108932,
        8609627384719100140,
        18269249874479319122,
        2462399806182025861,
        12115969418256798179,
        11419384508133015979,
        18405959278956055274,
        4736975856420530993,
        587737007810976185,
        4863733899365686858,
        16564944713779012481,
    ],
    [
        12999349433016689291,
        4647922643922544740,
        12776792002805945997,
        6783991966565072736,
        13300912480279117483,
        4022841722493233275,
        17561147893198553615,
        10419543369554397365,
        5714739568270017328,
        7538749690909854582,
        3144568778276045789,
        610308180641965005,
        17066969654013048270,
        640330268128256972,
        5595383892669644534,
        14739383115152240184,
        1310312667775347075,
        16072621487865888282,
        14391522990793380668,
        7706374484441734180,
        15008955132680163472,
        3940351935922423124,
        5873890535090939377,
        17573568754700538152,
        17367753551741430975,
        4922017806860634368,
        11233912418115198183,
        1483316514782045180,
        2440755523917410309,
        11924903510966803402,
        15753930415351833453,
        5226542696856740604,
        12198800063796976229,
        17992465001530781049,
        12517623475792967974,
        15069840509459181358,
        11993472180459029993,
        9840061076079979129,
        6377237197755721146,
        11908524557561081848,
        5760529300668767670,
        9511192131098273240,
        16046215123958748960,
        8538881676595777539,
        8634575074993300525,
        12375822621850517666,
        12209734153605972842,
        2272304548057846211,
        11148292372148606428,
        2714068486186156747,
        3708162086961692518,
        6960188425825865575,
        663777970678945632,
        4732592626443805406,
        9345063290873555724,
        18100252433033388183,
        17846540966831685069,
        13816422715518442683,
        3401483228556571222,
        14621585064784044266,
        9930055931106424212,
        3097271465993152374,
        18299013100159777874,
        14116515453040625277,
    ],
    [
        17035031329507215272,
        3594844294455448648,
        3491375611524609858,
        15371507138636015653,
        7725600472968298745,
        16802635380568259912,
        13417258712160830880,
        16813473967886565551,
        15595815422482455016,
        1415343936802024953,
        2335984357660814986,
        17810804753327305838,
        14968572783671943759,
        17710608004400771764,
        14191076226724256839,
        8131466486531851310,
        16859618305871239593,
        7320380779625787543,
        2503066192788467602,
        256399120092929484,
        8625007353562261615,
        3932006480065913717,
        6445253580972132631,
        4882879544892163825,
        14907055182983029000,
        4308208808365322986,
        5237937075258646664,
        7638776402683729328,
        4680052744545647081,
        17333686008851694043,
        4196089036787335398,
        4431405339285758847,
        1654791933349684650,
        3191943901787310050,
        4462966195038050032,
        15794859701879391662,
        12471185848443094257,
        8563571311210679164,
        4225731952342845923,
        983398892875756865,
        4580371217851623631,
        3332642285841527572,
        11404250290421082376,
        10384975675921074497,
        15841152015527031726,
        16355106196940567293,
        11501186589787152655,
        6999465420659741667,
        10566390953372269654,
        12166696185149810088,
        11340185154709138403,
        11352515964737810490,
        4251763751010255140,
        2782393782625713644,
        9287847467524526044,
        17113916471157813419,
        16190321823045122194,
        11759320446679125305,
        10355792750559605281,
        2174790220091648219,
        13052991089705168109,
        8634985592630570068,
        7546258485240516711,
        8292436146293289257,
    ],
    [
        6725212127120795002,
        13761053294016903939,
        1461920462754405514,
        18351109840552544586,
        9422170523370598714,
        3735473896724225035,
        7492539398526740304,
        9329355604977110519,
        18153433695243447567,
        16652873906850041972,
        9848110229076454716,
        5698934182701518392,
        7227097340245843256,
        9557074716798920392,
        10502749146670955381,
        4948594590187050147,
        8920174037208427711,
        6238957887293435398,
        5377905555281332396,
        13176592491165964081,
        7953304772462169738,
        4711685173553264673,
        7896060460494710858,
        6594119309691964256,
        5214524198079456111,
        2660472977567940559,
        9038104936451089512,
        9773509453128739172,
        13166397069386350517,
        3503653194299487121,
        3228555619956716974,
        2488966627274618653,
        3580083006979502320,
        576202076713616868,
        13005094400920501011,
        10907527228234325861,
        2381039530497426425,
        6444458086469575437,
        18294197174217413334,
        14854084146390915823,
        13715495782620890876,
        12572768677843669875,
        1133866823934528044,
        8073837092385560532,
        8054625526485188175,
        6933283226583629674,
        2303139397532864178,
        6782756083739297509,
        2813969525934185021,
        4655945454918880054,
        18078989817706926313,
        1949907726123029870,
        8177055344434895420,
        401154593375178371,
        7348159368145513849,
        15852667308774544611,
        6771932344520401054,
        14369599106511035368,
        928520195555529091,
        1669109232616751304,
        8419493424941591726,
        16360917189163440252,
        2357042573060929915,
        12589040476931890853,
    ],
    [
        5140670513146169840,
        743315633976944703,
        13582676171422563948,
        2638595461163004327,
        12644105601641636943,
        2398458091662109938,
        13015125187164133513,
        14304294970901378235,
        4318879720598669461,
        8671497611996725222,
        1067553322138439583,
        11086078853501007558,
        16881507310895520947,
        12091018789365520593,
        14549872807588833547,
        14579353026727586635,
        17998069729103815894,
        18041036539728830829,
        12603754704364389563,
        2894688930753113495,
        7162136479952151190,
        9395201758488883932,
        14231976473752138622,
        7668513201038283048,
        14721695515078204688,
        7025805144404525208,
        10730981453172786337,
        11535951848138854688,
        9542648823927342089,
        3876046097422586164,
        16001132375536246778,
        13068955472415077719,
        3307136937186377385,
        784124797531155829,
        17458830080818098668,
        9227499168228370557,
        9592744940430751657,
        11371735870544628322,
        16353248239766556346,
        11838199980884523633,
        16544762769490736067,
        9285949876080219392,
        17086370026349053575,
        4899957944632563360,
        41750434025721799,
        9887975749037497621,
        15589066637187670831,
        10780951752726569580,
        5510683516311591621,
        1798338076284745119,
        9607830010422182961,
        6673741108158826897,
        11141256770183185241,
        5396665252375371648,
        599958435157789040,
        16425742684553054605,
        15102840200553320429,
        1756136895484374608,
        14754677468590774166,
        15547160085166890709,
        6124859904211737730,
        1520178769392021515,
        8084850378084145521,
        1866698899611277201,
    ],
];