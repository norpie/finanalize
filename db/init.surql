OPTION IMPORT;

DEFINE TABLE api TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD name ON api TYPE string PERMISSIONS FULL;
DEFINE FIELD model ON api TYPE string PERMISSIONS FULL;

DEFINE TABLE column TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD description ON column TYPE string PERMISSIONS FULL;
DEFINE FIELD name ON column TYPE string PERMISSIONS FULL;
DEFINE FIELD values ON column TYPE array<string> PERMISSIONS FULL;

DEFINE TABLE data_source TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD description ON data_source TYPE string PERMISSIONS FULL;
DEFINE FIELD title ON data_source TYPE string PERMISSIONS FULL;

DEFINE TABLE has TYPE RELATION IN user OUT report ENFORCED SCHEMALESS PERMISSIONS NONE;
DEFINE FIELD in ON has TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD out ON has TYPE record<report> PERMISSIONS FULL;

DEFINE TABLE has_accepted_date TYPE RELATION IN report_source OUT source_date ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_accepted_date TYPE record<report_source> PERMISSIONS FULL;
DEFINE FIELD out ON has_accepted_date TYPE record<source_date> PERMISSIONS FULL;

DEFINE TABLE has_call TYPE RELATION IN report OUT llm_call ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_call TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_call TYPE record<llm_call> PERMISSIONS FULL;

DEFINE TABLE has_column TYPE RELATION IN data_source OUT column ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_column TYPE record<data_source> PERMISSIONS FULL;
DEFINE FIELD out ON has_column TYPE record<column> PERMISSIONS FULL;

DEFINE TABLE has_content_chunk TYPE RELATION IN report_source OUT source_chunk ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_content_chunk TYPE record<report_source> PERMISSIONS FULL;
DEFINE FIELD out ON has_content_chunk TYPE record<source_chunk> PERMISSIONS FULL;

DEFINE TABLE has_cost TYPE RELATION IN api | local_api OUT model_cost ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_cost TYPE record<api | local_api> PERMISSIONS FULL;
DEFINE FIELD out ON has_cost TYPE record<model_cost> PERMISSIONS FULL;

DEFINE TABLE has_data_source TYPE RELATION IN report_source OUT data_source ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_data_source TYPE record<report_source> PERMISSIONS FULL;
DEFINE FIELD out ON has_data_source TYPE record<data_source> PERMISSIONS FULL;

DEFINE TABLE has_final_cost TYPE RELATION IN report OUT ledger_entry ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_final_cost TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_final_cost TYPE record<ledger_entry> PERMISSIONS FULL;

DEFINE TABLE has_generated TYPE RELATION IN llm_call OUT llm_result ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_generated TYPE record<llm_call> PERMISSIONS FULL;
DEFINE FIELD out ON has_generated TYPE record<llm_result> PERMISSIONS FULL;

DEFINE TABLE has_heading TYPE RELATION IN report OUT report_heading ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_heading TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_heading TYPE record<report_heading> PERMISSIONS FULL;

DEFINE TABLE has_provider TYPE RELATION IN llm_call OUT api | local_api ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_provider TYPE record<llm_call> PERMISSIONS FULL;
DEFINE FIELD out ON has_provider TYPE record<api | local_api> PERMISSIONS FULL;

DEFINE TABLE has_research TYPE RELATION IN report OUT report_source ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_research TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_research TYPE record<report_source> PERMISSIONS FULL;

DEFINE TABLE has_subject TYPE RELATION IN report OUT report_subject ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_subject TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_subject TYPE record<report_subject> PERMISSIONS FULL;

DEFINE TABLE has_suspected_date TYPE RELATION IN report_source OUT source_date ENFORCED SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_suspected_date TYPE record<report_source> PERMISSIONS FULL;
DEFINE FIELD justification ON has_suspected_date TYPE string PERMISSIONS FULL;
DEFINE FIELD out ON has_suspected_date TYPE record<source_date> PERMISSIONS FULL;

DEFINE TABLE has_verdict TYPE RELATION IN report OUT report_verdict ENFORCED SCHEMALESS PERMISSIONS NONE;
DEFINE FIELD in ON has_verdict TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_verdict TYPE record<report_verdict> PERMISSIONS FULL;

DEFINE TABLE ledger_entry TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD amount ON ledger_entry TYPE int PERMISSIONS FULL;
DEFINE FIELD description ON ledger_entry TYPE string PERMISSIONS FULL;
DEFINE FIELD incoming ON ledger_entry TYPE bool PERMISSIONS FULL;
DEFINE FIELD timestamp ON ledger_entry TYPE datetime PERMISSIONS FULL;
DEFINE FIELD url ON ledger_entry TYPE string PERMISSIONS FULL;

DEFINE TABLE llm_call TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD prompt ON llm_call TYPE string PERMISSIONS FULL;
DEFINE FIELD requested_at ON llm_call TYPE datetime PERMISSIONS FULL;

DEFINE TABLE llm_result TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD completed_at ON llm_result TYPE datetime PERMISSIONS FULL;
DEFINE FIELD generated_amount ON llm_result TYPE int PERMISSIONS FULL;
DEFINE FIELD started_at ON llm_result TYPE datetime PERMISSIONS FULL;
DEFINE FIELD tokens ON llm_result TYPE string PERMISSIONS FULL;
DEFINE FIELD used_context ON llm_result TYPE int PERMISSIONS FULL;

DEFINE TABLE local_api TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD engine ON local_api TYPE string PERMISSIONS FULL;
DEFINE FIELD model ON local_api TYPE string PERMISSIONS FULL;

DEFINE TABLE model_cost TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD input_MTok ON model_cost TYPE int PERMISSIONS FULL;
DEFINE FIELD output_MTok ON model_cost TYPE int PERMISSIONS FULL;
DEFINE FIELD prompt_caching_read_MTok ON model_cost TYPE int PERMISSIONS FULL;
DEFINE FIELD prompt_caching_write_MTok ON model_cost TYPE int PERMISSIONS FULL;

DEFINE TABLE report TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD created_at ON report TYPE datetime PERMISSIONS FULL;
DEFINE FIELD status ON report TYPE string DEFAULT Pending PERMISSIONS FULL;
DEFINE FIELD updated_at ON report TYPE datetime PERMISSIONS FULL;
DEFINE FIELD user_input ON report TYPE string PERMISSIONS FULL;

DEFINE TABLE report_heading TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD description ON report_heading TYPE string PERMISSIONS FULL;
DEFINE FIELD title ON report_heading TYPE string PERMISSIONS FULL;

DEFINE TABLE report_source TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD description ON report_source TYPE string PERMISSIONS FULL;
DEFINE FIELD file_path ON report_source TYPE string PERMISSIONS FULL;
DEFINE FIELD title ON report_source TYPE string PERMISSIONS FULL;
DEFINE FIELD type ON report_source TYPE string PERMISSIONS FULL;
DEFINE FIELD url ON report_source TYPE string PERMISSIONS FULL;

DEFINE TABLE source_date TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD context ON source_date TYPE string PERMISSIONS FULL;
DEFINE FIELD date ON source_date TYPE datetime PERMISSIONS FULL;

DEFINE TABLE report_subject TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD ticker ON report_subject TYPE string PERMISSIONS FULL;
DEFINE FIELD title ON report_subject TYPE string PERMISSIONS FULL;

DEFINE TABLE report_verdict TYPE NORMAL SCHEMALESS PERMISSIONS NONE;
DEFINE FIELD justification ON report_verdict TYPE string PERMISSIONS FULL;
DEFINE FIELD valid ON report_verdict TYPE bool PERMISSIONS FULL;

DEFINE TABLE source_chunk TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD content ON source_chunk TYPE string PERMISSIONS FULL;
DEFINE FIELD embedding ON source_chunk TYPE array<float> PERMISSIONS FULL;

DEFINE INDEX mt_pts ON source_chunk FIELDS embedding MTREE DIMENSION 768 DIST COSINE TYPE F32;

DEFINE TABLE user TYPE ANY SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD email ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD password ON user TYPE string PERMISSIONS FULL;

// Create a default user
LET $user = CREATE user SET email = 'test@gmail.com', password = crypto::argon2::generate('password');

// Create a default report
LET $report = CREATE report:jgq1yy5g4i5zfgv8w1xy SET created_at = time::now(), status = 'Pending', updated_at = time::now(), user_input = 'Apple\'s current state';
RELATE $user -> has -> $report;

// Create a default report source
LET $report_source = (CREATE report_source CONTENT {
	description: 'Apple earnings report',
	file_path: '/tmp/apple_earnings_report.pdf',
	title: 'Apple Earnings Report',
	type: 'pdf',
	url: 'https://www.apple.com/investor/earnings-call/'
});
RELATE $report -> has_research -> $report_source;

// Create some source chunks
FOR $chunk IN ["Example chunk 1", "Example chunk 2", "Example chunk 3"] {
    LET $source_chunk = CREATE source_chunk CONTENT {
        embedding: [
            0.016254911,
            -0.00043637396,
            -0.15902433,
            -0.012892965,
            -0.0172784,
            0.06110683,
            -0.0053953817,
            -0.010089864,
            -0.005629155,
            -0.040778756,
            0.013354421,
            0.0732588,
            0.019538859,
            0.0513835,
            0.026831353,
            -0.059427485,
            0.0079023205,
            -0.06280571,
            -0.029268954,
            0.0256792,
            -0.031386003,
            -0.08904142,
            0.009123797,
            0.019633215,
            0.12306427,
            0.008892872,
            -0.03725695,
            0.071691394,
            0.012402903,
            -0.0033838197,
            -0.0047466676,
            0.0068348898,
            -0.0014511297,
            0.033730187,
            0.05598927,
            0.00034147387,
            0.021389892,
            0.008167023,
            0.025392987,
            -0.026033407,
            0.012819578,
            -0.0008957967,
            0.012305027,
            0.005799904,
            0.075801685,
            -0.01702415,
            -0.019477796,
            -0.034216546,
            0.07028795,
            -0.035438947,
            -0.046220824,
            -0.0075475234,
            -0.0033922927,
            0.05474323,
            0.047018692,
            0.017201329,
            0.05374641,
            -0.0351629,
            0.018457584,
            0.055940226,
            0.0418015,
            0.05448949,
            0.028336437,
            0.03242835,
            0.00066294224,
            -0.047678288,
            -0.0138054965,
            0.073616944,
            0.023393292,
            0.009358933,
            0.089784175,
            0.00053856603,
            0.026362129,
            0.026482718,
            0.03156578,
            0.024478462,
            0.0020515798,
            0.008926828,
            0.011390615,
            -0.025483457,
            0.053719774,
            -0.036151677,
            0.06266546,
            -0.042954557,
            -0.037245076,
            -0.06406072,
            -0.0051861294,
            -0.008150036,
            -0.055017065,
            0.031552,
            0.06844971,
            0.017666277,
            -0.017259901,
            0.007675906,
            -0.030067496,
            0.016248649,
            -0.0027327545,
            0.0092629455,
            -0.03195381,
            -0.013932235,
            0.0016247706,
            -0.050808482,
            -0.015998283,
            0.03757105,
            0.070076406,
            0.035128552,
            -0.02432436,
            -0.056604568,
            -0.06195315,
            -0.043433163,
            -0.034205202,
            0.026064618,
            -0.0012876978,
            0.04469657,
            -0.0335993,
            -0.016898233,
            0.036773834,
            -0.056790773,
            -0.004483315,
            0.07051025,
            -0.012960287,
            -0.01042646,
            -0.031579316,
            0.011406969,
            0.04446774,
            0.036491223,
            -0.041881472,
            -0.025882613,
            0.026985576,
            -0.010543098,
            -0.016599309,
            -0.01726475,
            -0.034798224,
            -0.013752503,
            -0.020725781,
            0.0254771,
            0.0010465744,
            -0.017104164,
            0.057848,
            0.029337995,
            -0.002939717,
            0.024324518,
            0.017688336,
            -0.048347875,
            -0.06513728,
            -0.012693794,
            0.0570027,
            -0.034992848,
            -0.009078733,
            -0.018584453,
            -0.010902371,
            0.020280425,
            0.050182473,
            0.039606225,
            -0.0038344786,
            -0.027167749,
            -0.02578824,
            0.0005659638,
            0.04967395,
            -0.020640424,
            0.005008519,
            -0.018822694,
            -0.056925613,
            0.0030114972,
            0.055004723,
            -0.08782884,
            0.058349784,
            0.03486455,
            0.008794552,
            0.020060366,
            -0.026693616,
            -0.053005856,
            0.004517519,
            -0.028890735,
            -0.049838737,
            -0.040559758,
            -0.012591395,
            -0.03761822,
            -0.012360078,
            0.006415256,
            0.060797233,
            -0.04780856,
            0.0019034359,
            0.018856846,
            -0.0016165673,
            -0.06905213,
            -0.013583964,
            0.0009247573,
            -0.010584774,
            -0.018356763,
            -0.028497428,
            0.022428775,
            -0.050781745,
            -0.0029953257,
            -0.012208071,
            -0.028334552,
            -0.01052624,
            0.0099771675,
            0.011145627,
            -0.07075714,
            -0.01074353,
            0.007145657,
            -0.03374084,
            -0.02642117,
            -0.026443617,
            0.04230103,
            0.06962315,
            0.013445293,
            0.044431917,
            0.033152197,
            0.08411201,
            -0.05262185,
            -0.052821025,
            -0.03669411,
            -0.01896677,
            0.015413465,
            -0.014226844,
            -0.053471435,
            -0.0053411773,
            0.0002400809,
            0.0110865515,
            -0.0107344575,
            0.049655695,
            0.011634367,
            0.082588926,
            0.010530306,
            -0.086960256,
            0.015345567,
            -0.070564024,
            -0.011367941,
            -0.03496298,
            -0.038054448,
            0.024039386,
            0.024892123,
            -0.019208135,
            0.033334516,
            0.042393565,
            0.05416773,
            -0.016435461,
            -0.037797887,
            -0.053559277,
            0.045539707,
            0.023308914,
            -0.021430328,
            -0.04139066,
            0.033042397,
            -0.037615214,
            0.007249224,
            -0.019129999,
            -0.0031587777,
            -0.03754396,
            -0.023810694,
            0.0013892043,
            0.0203759,
            0.020735288,
            -0.016572628,
            -0.022458538,
            0.009296195,
            -0.007217065,
            0.005256232,
            0.04700883,
            -0.09427108,
            0.04826332,
            -0.04230895,
            -0.03108754,
            0.0077613094,
            -0.011539925,
            0.059134647,
            0.053210426,
            -0.047996372,
            0.014793002,
            0.004506558,
            0.0026065337,
            0.027050355,
            -0.032390792,
            -0.03283611,
            0.020549579,
            0.008480435,
            -0.043080427,
            0.051919233,
            -0.050094653,
            0.008800591,
            -0.014646965,
            -0.034802884,
            -0.021809215,
            0.00907109,
            0.024163468,
            0.01308868,
            -0.025261134,
            -0.063917495,
            0.045170072,
            0.03827205,
            0.0015030892,
            0.08796984,
            0.011842788,
            0.01768568,
            0.06351691,
            0.012667784,
            -0.035864543,
            -0.027942244,
            0.0002442538,
            -0.012804468,
            0.038963724,
            -0.025660394,
            -0.036372177,
            0.015216353,
            0.04379357,
            -0.02396284,
            0.05632549,
            -0.0069846334,
            -0.052102026,
            0.08468243,
            -0.08234075,
            0.013689319,
            -0.043966178,
            0.023039835,
            -0.022672309,
            0.03775266,
            0.062920675,
            -0.0020236045,
            -0.03344067,
            -0.03834441,
            0.0030475531,
            -0.03214231,
            0.008595216,
            0.024371864,
            0.040481478,
            0.033736706,
            0.010122048,
            -0.02849376,
            -0.04083042,
            0.018369945,
            -0.030569594,
            -0.041534495,
            -0.019863727,
            0.050852496,
            -0.008540644,
            -0.05484172,
            -0.016268251,
            0.061083615,
            0.01314264,
            0.034743346,
            0.033074018,
            -0.027830234,
            -0.0018178938,
            -0.025765983,
            -0.011336784,
            0.005551951,
            -0.01426412,
            0.046096265,
            0.004548779,
            0.023150383,
            -0.022007592,
            0.015929526,
            0.0036965418,
            -0.01916365,
            0.019481149,
            0.046757307,
            0.00012519403,
            -0.04823001,
            -0.017908286,
            -0.006748191,
            0.026715733,
            0.0015925198,
            -0.012453798,
            0.0009427767,
            0.04382854,
            -0.01050435,
            0.036083587,
            -0.006400733,
            -0.004180591,
            -0.015386975,
            -0.035789248,
            0.033253845,
            0.013453682,
            0.011640717,
            -0.062952206,
            -0.031066446,
            -0.014972934,
            0.018065661,
            -0.04139905,
            -0.052819017,
            -0.0010312537,
            -0.03162261,
            -0.02002243,
            0.023501232,
            -0.0018853748,
            -0.007592,
            0.0064349463,
            0.024687385,
            0.025799766,
            -0.045388106,
            0.029083395,
            -0.02019375,
            -0.0034700397,
            0.013081033,
            0.0025375027,
            -0.012745596,
            -0.046694137,
            0.012122382,
            0.0016544189,
            0.0012135138,
            0.018239878,
            -0.07238807,
            0.001418626,
            -0.029050037,
            -0.041638594,
            -0.010754641,
            0.041417867,
            -0.0120293535,
            0.04910788,
            0.023409566,
            0.017024836,
            -0.055720087,
            -0.025271904,
            0.0213097,
            0.00721677,
            -0.06578562,
            -0.013925319,
            0.0023769953,
            0.043482184,
            0.06691991,
            0.02371849,
            0.030516308,
            -0.025120849,
            0.07588952,
            0.04199412,
            0.037695065,
            -0.009743137,
            -0.05322108,
            -0.0031681038,
            0.042738397,
            0.0081365295,
            0.042461175,
            0.00046475953,
            -0.034445677,
            0.013820874,
            -0.0044250563,
            0.05238439,
            0.034077298,
            0.063443735,
            -0.016525216,
            -0.022440441,
            -0.00549857,
            0.0054963254,
            0.110994294,
            0.09434135,
            -0.012265344,
            -0.05059153,
            0.013160749,
            -0.0537197,
            0.026321594,
            0.067513525,
            0.011355059,
            0.092808485,
            -0.06767822,
            0.05550602,
            -0.026778478,
            -0.007985118,
            0.00054272683,
            0.0036211275,
            0.016389064,
            -0.046017215,
            0.002505876,
            0.0063405163,
            -0.049198397,
            0.0014152684,
            -0.0026098,
            -0.007678801,
            0.004925502,
            -0.031621337,
            -0.022822393,
            0.010055205,
            -0.018567871,
            -0.058437906,
            0.03817273,
            -0.02961723,
            -0.027300563,
            -0.010281102,
            0.030999787,
            0.013672738,
            0.015917813,
            -0.04373392,
            -0.027017457,
            0.03582796,
            0.04410636,
            0.017508319,
            0.011716745,
            -0.018782116,
            0.039365966,
            -0.0016701105,
            -0.029008871,
            0.044517178,
            -0.0036875037,
            -0.024446353,
            0.01239379,
            -0.039319314,
            -0.013930817,
            -0.01713593,
            -0.0015152597,
            0.017169956,
            -0.019592496,
            -0.0136104915,
            0.038939442,
            -0.0071666213,
            0.015007949,
            0.010040083,
            -0.059799902,
            -0.023847647,
            0.04175188,
            -0.0190671,
            0.015053019,
            -0.010374418,
            0.030733919,
            0.0774623,
            -0.027255565,
            0.024721956,
            -0.020016499,
            -0.04402498,
            0.044720158,
            0.009627889,
            -0.03399204,
            -0.024680924,
            0.012640111,
            -0.05656377,
            0.06456782,
            0.015314775,
            -0.055318844,
            0.019426318,
            0.021564811,
            -0.013497278,
            0.027775008,
            0.016548185,
            -0.025258726,
            0.012476016,
            -0.021288171,
            -0.017671008,
            0.026469413,
            0.05742645,
            0.0054637324,
            -0.024477527,
            0.011225123,
            0.007245137,
            -0.04127099,
            0.04024835,
            0.04725242,
            -0.04107754,
            0.045155574,
            0.0027744807,
            0.04987477,
            -0.0352612,
            -0.034618396,
            -0.0061544986,
            -0.0147184525,
            -0.016522638,
            -0.07542899,
            -0.021417556,
            0.014726984,
            -0.04785604,
            -0.08532085,
            0.038882364,
            0.054549858,
            0.00616073,
            -0.004380331,
            0.019510858,
            0.005229618,
            0.026112793,
            0.004193882,
            0.0050868182,
            -0.013302289,
            -0.023022404,
            -0.071326315,
            0.018112939,
            -0.027992284,
            0.004372468,
            -0.029233024,
            0.03865706,
            -0.009055716,
            -0.046109613,
            -0.029306628,
            -0.012644727,
            -0.061733194,
            0.0034401757,
            0.02734171,
            -0.008685693,
            0.004693524,
            0.006790015,
            -0.05653363,
            0.043826904,
            -0.003095969,
            -0.032378234,
            0.008285143,
            0.029128965,
            -0.04642125,
            -0.02156631,
            0.03731392,
            -0.030226681,
            -0.04259669,
            -0.03033532,
            -0.025253432,
            -0.014565528,
            0.02590004,
            0.092695184,
            -0.08380106,
            0.04816553,
            0.062334567,
            0.00149072,
            0.019364607,
            0.024082398,
            0.018781783,
            0.081858665,
            0.012632368,
            -0.009858322,
            0.0062583364,
            0.022587474,
            0.005825591,
            0.07318961,
            -0.0046825423,
            0.010784519,
            -0.045272734,
            -0.050583538,
            -0.014514274,
            0.03632614,
            0.0040846285,
            0.03944053,
            0.00041095438,
            -0.03358918,
            -0.04079458,
            -0.059894565,
            0.058988787,
            0.004126151,
            -0.013350332,
            -0.08063164,
            0.007584617,
            -0.011294577,
            0.03277774,
            0.022244113,
            -0.0050149146,
            -0.017225277,
            0.012810426,
            0.022439364,
            -0.012520944,
            0.010501332,
            0.0025589871,
            0.030246845,
            -0.018519193,
            0.06416155,
            0.03854359,
            0.05734164,
            0.03808318,
            0.07740367,
            0.05188216,
            0.037014056,
            0.035601407,
            -0.0034890154,
            -0.02875637,
            -0.0019385277,
            -0.041411955,
            -0.056213878,
            0.041620143,
            -0.016910624,
            0.010910943,
            -0.059755195,
            -0.031667687,
            0.031426787,
            -0.031336486,
            -0.029794449,
            0.02842037,
            -0.086790584,
            0.015061666,
            0.0006790968,
            -0.015767721,
            -0.032308474,
            -0.03255979,
            -0.010031328,
            0.021627968,
            0.02254975,
            0.033290964,
            0.044683773,
            0.0020173984,
            0.051129792,
            -0.014782958,
            -0.047998313,
            0.03997748,
            0.05438307,
            -0.034361143,
            0.04670934,
            -0.023195304,
            -0.07981718,
            -0.043761022,
            -0.013680226,
            -0.014554889,
            -0.0065255584,
            -0.04574163,
            -0.029121852,
            0.016499737,
            -0.036660418,
            -0.0048266267,
            -0.04239846,
            0.041018613,
            -0.022563862,
            -0.031641193,
            0.027931305,
            0.020639606,
            -0.0071845637,
            -0.026974097,
            -0.022516968,
            0.020396601,
            -0.017563201,
            -0.006730198,
            -0.02034999,
            0.04538039,
            0.0134374,
            0.037104465,
            0.004136851,
            0.01981498,
            -0.022107081,
            0.017321786,
            0.02246305,
            0.052299995,
            0.067283034,
            0.0074340776,
            -0.05282149,
            0.0002347331,
            -0.008166186,
            0.0146671375,
            -0.012885897,
            -0.0033912628,
            0.018223412,
            -0.0007485414,
            0.006190673,
            -0.034162547,
            -0.026106358,
            0.06598076,
            -0.030035874,
            0.015673446,
            -0.015394942,
            -0.06639062,
            -0.043370526,
            -0.042706832,
            -0.053457987,
            -0.040775396,
            -0.004778764,
            0.03632704,
            -0.019562576,
            -0.031032726,
            0.03983224,
            0.059826385,
            -0.015353301,
            -0.025527129,
            -0.019886475,
            -0.0010565204,
            -0.022129856,
            -0.03842838,
            0.023157053,
            0.057532545,
            -0.0015149137,
            0.03773977,
            0.083989084,
            -0.03700513,
            0.010624633,
            0.0074445885,
            0.024649307,
            0.04723193,
            0.032546986,
            -0.015602126,
            -0.046390086,
            0.0076707676,
        ],
        content: $chunk
    };
    RELATE $report_source -> has_content_chunk -> $source_chunk;
}
