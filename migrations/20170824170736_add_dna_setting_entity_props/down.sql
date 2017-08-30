drop table settings;
drop table dnas;

alter table entities
drop column dna_id,
drop column setting_id,
drop column fitness,
drop column life_expectancy,
drop column nickname,
drop column start_mating_at,
drop column last_seed_at;
