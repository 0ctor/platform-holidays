use chrono::NaiveDate;

use super::types::{Holiday, HolidayKind, HolidayScope};

/// Datas comemorativas da saúde (fixas DD/MM). Não são feriados legais.
const HEALTH_DAYS: &[(u32, u32, &str, &str)] = &[
    (1, 2, "Dia do Sanitarista", "Datas comemorativas da saúde"),
    (
        1,
        13,
        "Dia Mundial do Combate à Depressão",
        "Datas comemorativas da saúde",
    ),
    (
        1,
        27,
        "Dia Mundial contra a Hanseníase",
        "Datas comemorativas da saúde",
    ),
    (
        2,
        4,
        "Dia Mundial contra o Câncer",
        "Datas comemorativas da saúde",
    ),
    (
        2,
        15,
        "Dia Internacional da Luta contra o Câncer Infantil",
        "Datas comemorativas da saúde",
    ),
    (
        3,
        8,
        "Dia Internacional da Mulher",
        "Datas comemorativas da saúde",
    ),
    (
        3,
        24,
        "Dia Mundial da Tuberculose",
        "Datas comemorativas da saúde",
    ),
    (
        4,
        2,
        "Dia Mundial de Conscientização do Autismo",
        "Datas comemorativas da saúde",
    ),
    (4, 7, "Dia Mundial da Saúde", "Datas comemorativas da saúde"),
    (
        5,
        5,
        "Dia Mundial da Asma / Higiene das Mãos",
        "Datas comemorativas da saúde",
    ),
    (
        5,
        12,
        "Dia Internacional da Enfermagem",
        "Datas comemorativas da saúde",
    ),
    (
        5,
        17,
        "Dia Mundial da Hipertensão",
        "Datas comemorativas da saúde",
    ),
    (
        5,
        31,
        "Dia Mundial sem Tabaco",
        "Datas comemorativas da saúde",
    ),
    (
        6,
        14,
        "Dia Mundial do Doador de Sangue",
        "Datas comemorativas da saúde",
    ),
    (
        7,
        10,
        "Dia Mundial da Saúde Ocular",
        "Datas comemorativas da saúde",
    ),
    (
        7,
        28,
        "Dia Mundial de Luta contra as Hepatites Virais",
        "Datas comemorativas da saúde",
    ),
    (
        8,
        1,
        "Dia Mundial da Amamentação (início da semana)",
        "Datas comemorativas da saúde",
    ),
    (
        9,
        10,
        "Dia Mundial de Prevenção ao Suicídio",
        "Datas comemorativas da saúde",
    ),
    (
        9,
        21,
        "Dia Nacional de Luta das Pessoas com Deficiência / Alzheimer",
        "Datas comemorativas da saúde",
    ),
    (
        9,
        29,
        "Dia Mundial do Coração",
        "Datas comemorativas da saúde",
    ),
    (
        10,
        1,
        "Dia Internacional do Idoso",
        "Datas comemorativas da saúde",
    ),
    (
        10,
        10,
        "Dia Mundial da Saúde Mental",
        "Datas comemorativas da saúde",
    ),
    (
        10,
        16,
        "Dia Mundial da Alimentação",
        "Datas comemorativas da saúde",
    ),
    (
        11,
        14,
        "Dia Mundial do Diabetes",
        "Datas comemorativas da saúde",
    ),
    (
        12,
        1,
        "Dia Mundial de Luta contra a AIDS",
        "Datas comemorativas da saúde",
    ),
];

pub fn health_for_year(year: i32) -> Vec<Holiday> {
    HEALTH_DAYS
        .iter()
        .filter_map(|(month, day, title, category)| {
            let date = NaiveDate::from_ymd_opt(year, *month, *day)?;
            Some(Holiday {
                date,
                title: (*title).into(),
                description: (*title).into(),
                legislation: String::new(),
                scope: HolidayScope::Health,
                kind: HolidayKind::Commemorative,
                uf: None,
                city_ibge: None,
                category: Some((*category).into()),
                category_color: Some("#6B8E9F".into()),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_includes_world_health_day() {
        let items = health_for_year(2026);
        assert!(items.iter().any(|h| h.title.contains("Mundial da Saúde")
            && h.date == NaiveDate::from_ymd_opt(2026, 4, 7).unwrap()));
    }
}
