use chrono::NaiveDate;

use super::types::{Holiday, HolidayKind, HolidayScope};

/// (código IBGE, UF, mês, dia, título, descrição)
type CityFixed = (
    &'static str,
    &'static str,
    u32,
    u32,
    &'static str,
    &'static str,
);

/// Municipais curados (capitais e cidades frequentes). Expandível via PRs.
const CITIES: &[CityFixed] = &[
    (
        "3550308",
        "SP",
        1,
        25,
        "Aniversário de São Paulo",
        "Fundação de São Paulo.",
    ),
    (
        "3304557",
        "RJ",
        3,
        1,
        "Aniversário do Rio de Janeiro",
        "Dia da cidade do Rio de Janeiro.",
    ),
    (
        "3106200",
        "MG",
        12,
        12,
        "Aniversário de Belo Horizonte",
        "Fundação de Belo Horizonte.",
    ),
    (
        "4106902",
        "PR",
        3,
        29,
        "Aniversário de Curitiba",
        "Emancipação de Curitiba.",
    ),
    (
        "4314902",
        "RS",
        3,
        26,
        "Aniversário de Porto Alegre",
        "Fundação de Porto Alegre.",
    ),
    (
        "2927408",
        "BA",
        3,
        29,
        "Aniversário de Salvador",
        "Emancipação de Salvador.",
    ),
    (
        "2304400",
        "CE",
        4,
        13,
        "Aniversário de Fortaleza",
        "Emancipação de Fortaleza.",
    ),
    (
        "2611606",
        "PE",
        3,
        12,
        "Aniversário do Recife",
        "Emancipação do Recife.",
    ),
    (
        "1302603",
        "AM",
        10,
        24,
        "Aniversário de Manaus",
        "Elevação de Manaus a cidade.",
    ),
    (
        "1501402",
        "PA",
        1,
        12,
        "Aniversário de Belém",
        "Fundação de Belém.",
    ),
    (
        "5208707",
        "GO",
        10,
        24,
        "Aniversário de Goiânia",
        "Fundação de Goiânia.",
    ),
    (
        "2111300",
        "MA",
        9,
        8,
        "Aniversário de São Luís",
        "Fundação de São Luís.",
    ),
    (
        "2507507",
        "PB",
        8,
        5,
        "Aniversário de João Pessoa",
        "Fundação de João Pessoa.",
    ),
    (
        "2800308",
        "SE",
        3,
        17,
        "Aniversário de Aracaju",
        "Emancipação de Aracaju.",
    ),
    (
        "2704302",
        "AL",
        12,
        5,
        "Aniversário de Maceió",
        "Emancipação de Maceió.",
    ),
    (
        "2211001",
        "PI",
        8,
        16,
        "Aniversário de Teresina",
        "Fundação de Teresina.",
    ),
    (
        "2408102",
        "RN",
        12,
        25,
        "Aniversário de Natal",
        "Fundação de Natal (coincidente com o feriado de Natal).",
    ),
    (
        "5002704",
        "MS",
        8,
        26,
        "Aniversário de Campo Grande",
        "Emancipação de Campo Grande.",
    ),
    (
        "5103403",
        "MT",
        4,
        8,
        "Aniversário de Cuiabá",
        "Fundação de Cuiabá.",
    ),
    (
        "3205309",
        "ES",
        9,
        8,
        "Aniversário de Vitória",
        "Emancipação de Vitória.",
    ),
    (
        "4205407",
        "SC",
        3,
        23,
        "Aniversário de Florianópolis",
        "Emancipação de Florianópolis.",
    ),
    (
        "1721000",
        "TO",
        5,
        20,
        "Aniversário de Palmas",
        "Fundação de Palmas.",
    ),
    (
        "1400100",
        "RR",
        7,
        9,
        "Aniversário de Boa Vista",
        "Emancipação de Boa Vista.",
    ),
    (
        "1200401",
        "AC",
        12,
        28,
        "Aniversário de Rio Branco",
        "Emancipação de Rio Branco.",
    ),
    (
        "1600303",
        "AP",
        2,
        2,
        "Aniversário de Macapá",
        "Emancipação de Macapá.",
    ),
    (
        "1100205",
        "RO",
        2,
        2,
        "Aniversário de Porto Velho",
        "Emancipação de Porto Velho.",
    ),
];

pub fn normalize_city_ibge(code: &str) -> Option<String> {
    let c = code.trim();
    if c.len() == 7 && c.chars().all(|ch| ch.is_ascii_digit()) {
        Some(c.to_owned())
    } else {
        None
    }
}

pub fn city_for_year(city_ibge: &str, year: i32) -> Vec<Holiday> {
    let Some(code) = normalize_city_ibge(city_ibge) else {
        return Vec::new();
    };
    CITIES
        .iter()
        .filter(|(ibge, ..)| *ibge == code)
        .filter_map(|(ibge, uf, month, day, title, description)| {
            let date = NaiveDate::from_ymd_opt(year, *month, *day)?;
            Some(Holiday {
                date,
                title: (*title).into(),
                description: (*description).into(),
                legislation: String::new(),
                scope: HolidayScope::City,
                kind: HolidayKind::Feriado,
                uf: Some((*uf).into()),
                city_ibge: Some((*ibge).into()),
                category: None,
                category_color: None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sao_paulo_anniversary() {
        let items = city_for_year("3550308", 2026);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].date, NaiveDate::from_ymd_opt(2026, 1, 25).unwrap());
    }

    #[test]
    fn invalid_ibge() {
        assert!(city_for_year("123", 2026).is_empty());
        assert!(normalize_city_ibge("abc").is_none());
    }
}
