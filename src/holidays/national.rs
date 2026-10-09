use chrono::NaiveDate;

use super::easter::{easter_sunday, shift};
use super::types::{Holiday, HolidayKind};

const LEI_10607: &str =
    "Lei nº 10.607, de 19 de dezembro de 2002 — https://www.planalto.gov.br/ccivil_03/leis/2002/L10607.htm";
const LEI_9093: &str =
    "Lei nº 9.093, de 12 de setembro de 1995 — https://www.planalto.gov.br/ccivil_03/Leis/L9093.htm";
const LEI_6802: &str =
    "Lei nº 6.802, de 30 de junho de 1980 — https://www.planalto.gov.br/ccivil_03/Leis/L6802.htm";
const LEI_14759: &str =
    "Lei nº 14.759, de 21 de dezembro de 2023 — https://www.planalto.gov.br/ccivil_03/_ato2023-2026/2023/lei/L14759.htm";

fn ymd(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("data nacional inválida")
}

/// Feriados nacionais (fixos + móveis) para um ano civil.
pub fn national_for_year(year: i32) -> Vec<Holiday> {
    let easter = easter_sunday(year);
    let mut items = vec![
        Holiday::national(
            ymd(year, 1, 1),
            "Confraternização Universal",
            "Início do ano civil (Dia Mundial da Paz).",
            LEI_10607,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            shift(easter, -48),
            "Carnaval (segunda-feira)",
            "Ponto facultativo tradicionalmente observado em várias localidades.",
            "",
            HolidayKind::Facultativo,
        ),
        Holiday::national(
            shift(easter, -47),
            "Carnaval",
            "Ponto facultativo tradicionalmente observado em várias localidades.",
            "",
            HolidayKind::Facultativo,
        ),
        Holiday::national(
            shift(easter, -2),
            "Sexta-Feira Santa",
            "Paixão de Cristo.",
            LEI_9093,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            easter,
            "Páscoa",
            "Domingo de Páscoa (referência cristã; não é feriado civil nacional).",
            "",
            HolidayKind::Commemorative,
        ),
        Holiday::national(
            ymd(year, 4, 21),
            "Tiradentes",
            "Homenagem ao mártir da Inconfidência Mineira.",
            LEI_10607,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            ymd(year, 5, 1),
            "Dia do Trabalhador",
            "Dia Internacional dos Trabalhadores.",
            LEI_10607,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            shift(easter, 60),
            "Corpus Christi",
            "Celebração católica do Corpo de Cristo (feriado em muitos municípios; ponto facultativo federal).",
            "",
            HolidayKind::Facultativo,
        ),
        Holiday::national(
            ymd(year, 9, 7),
            "Independência do Brasil",
            "Grito do Ipiranga.",
            LEI_10607,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            ymd(year, 10, 12),
            "Nossa Senhora Aparecida",
            "Padroeira do Brasil.",
            LEI_6802,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            ymd(year, 11, 2),
            "Finados",
            "Dia de memória aos mortos.",
            LEI_10607,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            ymd(year, 11, 15),
            "Proclamação da República",
            "Instalação da República.",
            LEI_10607,
            HolidayKind::Feriado,
        ),
        Holiday::national(
            ymd(year, 12, 25),
            "Natal",
            "Celebração do nascimento de Cristo.",
            LEI_10607,
            HolidayKind::Feriado,
        ),
    ];

    // Dia Nacional de Zumbi e da Consciência Negra — feriado nacional a partir de 2024.
    if year >= 2024 {
        items.push(Holiday::national(
            ymd(year, 11, 20),
            "Consciência Negra",
            "Dia Nacional de Zumbi e da Consciência Negra.",
            LEI_14759,
            HolidayKind::Feriado,
        ));
    }

    items.sort_by_key(|h| h.date);
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movable_dates_2026() {
        let items = national_for_year(2026);
        let by_title = |t: &str| {
            items
                .iter()
                .find(|h| h.title == t)
                .unwrap_or_else(|| panic!("missing {t}"))
                .date
        };
        assert_eq!(by_title("Carnaval"), ymd(2026, 2, 17));
        assert_eq!(by_title("Sexta-Feira Santa"), ymd(2026, 4, 3));
        assert_eq!(by_title("Páscoa"), ymd(2026, 4, 5));
        assert_eq!(by_title("Corpus Christi"), ymd(2026, 6, 4));
    }

    #[test]
    fn movable_dates_differ_across_years() {
        let carnaval_2025 = national_for_year(2025)
            .into_iter()
            .find(|h| h.title == "Carnaval")
            .unwrap()
            .date;
        let carnaval_2026 = national_for_year(2026)
            .into_iter()
            .find(|h| h.title == "Carnaval")
            .unwrap()
            .date;
        assert_ne!(carnaval_2025, carnaval_2026);
    }

    #[test]
    fn consciencia_negra_from_2024() {
        assert!(national_for_year(2023)
            .iter()
            .all(|h| h.title != "Consciência Negra"));
        assert!(national_for_year(2024)
            .iter()
            .any(|h| h.title == "Consciência Negra" && h.date == ymd(2024, 11, 20)));
    }

    #[test]
    fn fixed_holidays_present() {
        let titles: Vec<_> = national_for_year(2026)
            .into_iter()
            .filter(|h| h.kind == HolidayKind::Feriado)
            .map(|h| h.title)
            .collect();
        for expected in [
            "Confraternização Universal",
            "Tiradentes",
            "Dia do Trabalhador",
            "Independência do Brasil",
            "Nossa Senhora Aparecida",
            "Finados",
            "Proclamação da República",
            "Consciência Negra",
            "Natal",
            "Sexta-Feira Santa",
        ] {
            assert!(titles.iter().any(|t| t == expected), "faltou {expected}");
        }
    }
}
