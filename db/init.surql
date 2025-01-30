OPTION IMPORT;

DEFINE TABLE has TYPE RELATION IN user OUT report SCHEMALESS PERMISSIONS NONE;
DEFINE FIELD in ON has TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD out ON has TYPE record<report> PERMISSIONS FULL;

DEFINE TABLE has_accepted_date TYPE RELATION IN report_source OUT report_source_date SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_accepted_date TYPE record<report_source> PERMISSIONS FULL;
DEFINE FIELD out ON has_accepted_date TYPE record<report_source_date> PERMISSIONS FULL;

DEFINE TABLE has_chunk TYPE RELATION IN report_source OUT source_chunk SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_chunk TYPE record<report_source> PERMISSIONS FULL;
DEFINE FIELD out ON has_chunk TYPE record<source_chunk> PERMISSIONS FULL;

DEFINE TABLE has_heading TYPE RELATION IN report OUT report_heading SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_heading TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_heading TYPE record<report_heading> PERMISSIONS FULL;

DEFINE TABLE has_research TYPE RELATION IN report OUT report_source SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_research TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_research TYPE record<report_source> PERMISSIONS FULL;

DEFINE TABLE has_subject TYPE RELATION IN report OUT report_subject SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_subject TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_subject TYPE record<report_subject> PERMISSIONS FULL;

DEFINE TABLE has_suspected_date TYPE RELATION IN report_source OUT report_source_date SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD in ON has_suspected_date TYPE record<report_source> PERMISSIONS FULL;
DEFINE FIELD justification ON has_suspected_date TYPE string PERMISSIONS FULL;
DEFINE FIELD out ON has_suspected_date TYPE record<report_source_date> PERMISSIONS FULL;

DEFINE TABLE has_verdict TYPE RELATION IN report OUT report_verdict SCHEMALESS PERMISSIONS NONE;
DEFINE FIELD in ON has_verdict TYPE record<report> PERMISSIONS FULL;
DEFINE FIELD out ON has_verdict TYPE record<report_verdict> PERMISSIONS FULL;

DEFINE TABLE report TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD created_at ON report TYPE datetime PERMISSIONS FULL;
DEFINE FIELD has_verdict ON report TYPE bool PERMISSIONS FULL;
DEFINE FIELD status ON report TYPE string DEFAULT Pending PERMISSIONS FULL;
DEFINE FIELD updated_at ON report TYPE datetime PERMISSIONS FULL;
DEFINE FIELD user_input ON report TYPE string PERMISSIONS FULL;

DEFINE TABLE report_heading TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD description ON report_heading TYPE string PERMISSIONS FULL;
DEFINE FIELD title ON report_heading TYPE string PERMISSIONS FULL;

DEFINE TABLE report_source TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD description ON report_source TYPE string PERMISSIONS FULL;
DEFINE FIELD title ON report_source TYPE string PERMISSIONS FULL;
DEFINE FIELD type ON report_source TYPE string PERMISSIONS FULL;
DEFINE FIELD url ON report_source TYPE string PERMISSIONS FULL;

DEFINE TABLE report_source_date TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD context ON report_source_date TYPE string PERMISSIONS FULL;
DEFINE FIELD date ON report_source_date TYPE datetime PERMISSIONS FULL;

DEFINE TABLE report_subject TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD ticker ON report_subject TYPE string PERMISSIONS FULL;
DEFINE FIELD title ON report_subject TYPE string PERMISSIONS FULL;

DEFINE TABLE report_verdict TYPE NORMAL SCHEMALESS PERMISSIONS NONE;
DEFINE FIELD justification ON report_verdict TYPE string PERMISSIONS FULL;
DEFINE FIELD valid ON report_verdict TYPE bool PERMISSIONS FULL;

DEFINE TABLE source_chunk TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD content ON source_chunk TYPE string PERMISSIONS FULL;
DEFINE FIELD embedding ON source_chunk TYPE array<float> PERMISSIONS FULL;

DEFINE TABLE user TYPE ANY SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD email ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD password ON user TYPE string PERMISSIONS FULL;
