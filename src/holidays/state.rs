use chrono::NaiveDate;

use super::types::{Holiday, HolidayKind, HolidayScope};

/// Entrada estática: (mês, dia, título, descrição, legislação).
type Fixed = (u32, u32, &'static str, &'static str, &'static str);

const ALL_UFS: &[&str] = &[
    "AC", "AL", "AM", "AP", "BA", "CE", "DF", "ES", "GO", "MA", "MG", "MS", "MT", "PA", "PB", "PE",
    "PI", "PR", "RJ", "RN", "RO", "RR", "RS", "SC", "SE", "SP", "TO",
];

/// Feriados estaduais curados (não exaustivo; expandível).
fn catalog(uf: &str) -> &'static [Fixed] {
    match uf {
        "AC" => &[
            (
                6,
                15,
                "Aniversário do Acre",
                "Elevação do Acre a estado.",
                "",
            ),
            (9, 5, "Dia do Acre", "Incorporação do Acre ao Brasil.", ""),
        ],
        "AL" => &[(6, 24, "São João", "Padroeiro / tradição alagoana.", "")],
        "AM" => &[
            (
                9,
                5,
                "Elevação do Amazonas a Província",
                "Data estadual.",
                "",
            ),
            (
                12,
                8,
                "Nossa Senhora da Conceição",
                "Padroeira de Manaus (observada no estado).",
                "",
            ),
        ],
        "AP" => &[(3, 19, "Dia de São José", "Padroeiro do Amapá.", "")],
        "BA" => &[(
            7,
            2,
            "Independência da Bahia",
            "Independência da Bahia.",
            "",
        )],
        "CE" => &[(
            3,
            25,
            "Data Magna do Ceará",
            "Abolição da escravidão no Ceará.",
            "",
        )],
        "DF" => &[(
            4,
            21,
            "Fundação de Brasília",
            "Aniversário de Brasília (coincidente com Tiradentes).",
            "",
        )],
        "ES" => &[(
            5,
            23,
            "Colonização do Solo Espírito-santense",
            "Data estadual.",
            "",
        )],
        "GO" => &[(7, 26, "Fundação de Goiás", "Data estadual.", "")],
        "MA" => &[(
            7,
            28,
            "Adesão do Maranhão à Independência",
            "Data estadual.",
            "",
        )],
        "MG" => &[(
            4,
            21,
            "Tiradentes (estadual)",
            "Mártir da Inconfidência Mineira.",
            "",
        )],
        "MS" => &[(
            10,
            11,
            "Criação do Mato Grosso do Sul",
            "Data estadual.",
            "",
        )],
        "MT" => &[(
            11,
            20,
            "Consciência Negra (MT)",
            "Observância estadual histórica.",
            "",
        )],
        "PA" => &[(
            8,
            15,
            "Adesão do Pará à Independência",
            "Data estadual.",
            "",
        )],
        "PB" => &[(
            7,
            26,
            "Homenagem à Paraíba / Nossa Senhora das Neves",
            "Data estadual.",
            "",
        )],
        "PE" => &[
            (3, 6, "Revolução Pernambucana", "Data estadual.", ""),
            (6, 24, "São João", "Festa junina / tradição.", ""),
        ],
        "PI" => &[(3, 13, "Dia da Batalha do Jenipapo", "Data estadual.", "")],
        "PR" => &[(
            12,
            19,
            "Emancipação Política do Paraná",
            "Data estadual.",
            "",
        )],
        "RJ" => &[
            (1, 20, "São Sebastião", "Padroeiro do Rio de Janeiro.", ""),
            (
                4,
                23,
                "São Jorge",
                "Padroeiro do estado do Rio de Janeiro.",
                "",
            ),
        ],
        "RN" => &[(10, 3, "Mártires de Cunhaú e Uruaçu", "Data estadual.", "")],
        "RO" => &[(1, 4, "Criação de Rondônia", "Data estadual.", "")],
        "RR" => &[(10, 5, "Elevação de Roraima a estado", "Data estadual.", "")],
        "RS" => &[(
            9,
            20,
            "Revolução Farroupilha",
            "Data magna do Rio Grande do Sul.",
            "",
        )],
        "SC" => &[(
            8,
            11,
            "Criação da Capitania de Santa Catarina",
            "Data estadual.",
            "",
        )],
        "SE" => &[(
            7,
            8,
            "Emancipação Política de Sergipe",
            "Data estadual.",
            "",
        )],
        "SP" => &[(
            7,
            9,
            "Revolução Constitucionalista",
            "Revolução de 1932.",
            "",
        )],
        "TO" => &[(10, 5, "Criação do Tocantins", "Data estadual.", "")],
        _ => &[],
    }
}

pub fn normalize_uf(uf: &str) -> Option<String> {
    let u = uf.trim().to_ascii_uppercase();
    if ALL_UFS.contains(&u.as_str()) {
        Some(u)
    } else {
        None
    }
}

pub fn state_for_year(uf: &str, year: i32) -> Vec<Holiday> {
    let Some(uf) = normalize_uf(uf) else {
        return Vec::new();
    };
    catalog(&uf)
        .iter()
        .filter_map(|(month, day, title, description, legislation)| {
            let date = NaiveDate::from_ymd_opt(year, *month, *day)?;
            Some(Holiday {
                date,
                title: (*title).into(),
                description: (*description).into(),
                legislation: (*legislation).into(),
                scope: HolidayScope::State,
                kind: HolidayKind::Feriado,
                uf: Some(uf.clone()),
                city_ibge: None,
                category: None,
                category_color: None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn sp_constitutionalista() {
        let items = state_for_year("sp", 2026);
        assert!(items.iter().any(|h| {
            h.title.contains("Constitucionalista")
                && h.date == NaiveDate::from_ymd_opt(2026, 7, 9).unwrap()
        }));
    }

    #[test]
    fn rj_sao_jorge() {
        let items = state_for_year("RJ", 2026);
        assert!(items
            .iter()
            .any(|h| h.title == "São Jorge" && h.date.month() == 4 && h.date.day() == 23));
    }

    #[test]
    fn unknown_uf_empty() {
        assert!(state_for_year("XX", 2026).is_empty());
        assert!(normalize_uf("XX").is_none());
    }
}
